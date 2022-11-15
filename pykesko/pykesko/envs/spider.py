from typing import Optional, Tuple
from queue import Queue

import gym
from gym.spaces.box import Box
import numpy as np

from ..backend import BackendType, RenderMode
from ..kesko import Kesko
from ..protocol.commands import (
    DespawnAll,
    GetState,
    PausePhysics,
    RunPhysics,
    Spawn,
    ApplyControl,
)
from ..protocol.response import KeskoResponse, MultibodyStates, CollisionStarted
from ..color import Color
from pykesko import KeskoModel
from ..utils import action_space_from_limits


class SpiderEnv(gym.Env):
    metadata = {"render_modes": ["human", "rgb_array"]}

    def __init__(
        self,
        max_steps: Optional[int] = None,
        render_mode: Optional[str] = None,
        backend_type: str = "bindings",
        reward_step_length: int = 5,
        large_movement_reward_factor: float = 1.0,
        joint_acceleration_reward_factor: float = 0.02,
    ):
        """
        Creates the spider environment. A four legged agent which is tasked with
        learning to walk in the positive x-direction.

        Reward description
        -------------------
        The reward consists of 4 parts.

        1. For each step when the agent is not terminated it will receive 1.0

        2. It will get a reward equal to the velocity in the x-direction

        3. A positive reward for making large movements instead of small movements.

        4. It will get a negative reward proportional to the magnitude of the accelerations in the joints.
           This in order to minimize erratic movements

        The episode will be terminated if the head collides with the ground.

        Args:
            max_steps: Maximum steps before resetting. Defaults to None.
            render_mode: If the environment should be rendered or run in headless. Defaults to None.
            backend: Type of backend to use for communication with Kesko. Can be either 'tcp' or 'bindings'.
            reward_step_length: The step interval to use when calculating the reward. Defaults to 5.
            large_movement_reward_factor: How much to reward large movements over small movements.
            joint_acceleration_reward_factor: How much to punish large joint accelerations

        Raises:
            ValueError: If incorrect 'render_mode' or 'backend'.
        """

        assert render_mode is None or render_mode in self.metadata["render_modes"]
        self.render_mode = render_mode

        self.max_steps = max_steps
        self.reward_step_length = reward_step_length
        self.large_movement_reward_factor = large_movement_reward_factor
        self.joint_acceleration_reward_factor = joint_acceleration_reward_factor

        self.past_states_queue = Queue(reward_step_length)
        self.step_count = 0
        self.reward_move = 0.0
        self.reward_survive = 0.0
        self.reward_acceleration = 0.0
        self.reward_large_movements = 0.0

        # setup kesko
        mode = RenderMode.WINDOW if self.render_mode == "human" else RenderMode.HEADLESS
        if backend_type not in ("bindings", "tcp"):
            raise ValueError("Invalid option for backend, use 'native' or 'tcp'")

        self.backend = BackendType.TCP if backend_type == "tcp" else BackendType.BINDINGS
        self._kesko = Kesko(render_mode=mode, backend_type=self.backend)
        self._kesko.initialize()
        self._setup()

    def _setup(self) -> Tuple[np.ndarray, dict]:

        # Spawn bodies
        self._kesko.send(
            [
                Spawn(model=KeskoModel.Plane, position=[0.0, 0.0, 0.0], color=Color.WHITE),
                Spawn(
                    model=KeskoModel.Spider,
                    position=[0.0, 0.4, 0.0],
                    color=Color.DEEP_ORANGE,
                ),
            ]
        )

        # Kesko stores all the bodies that are in the environment, get the body named by base name.
        try:
            self.spider_body = [body for body in self._kesko.bodies.values() if "spider" in body.name][0]
        except IndexError as e:
            self.close()
            raise ValueError(f"Could not get body from Kesko: {e}")

        # get initial state
        initial_state = self._get_state(self._kesko.send(GetState()))
        initial_state = self._to_numpy(initial_state)

        # start physics
        self._kesko.send(RunPhysics())

        # Define spaces
        self.action_space = action_space_from_limits(
            [joint.limits for joint in self.spider_body.joints.values()],
            normalized=False,
        )
        self.observation_space = Box(low=-np.inf, high=np.inf, shape=initial_state.shape)

        return initial_state, {}

    def _to_numpy(self, state: MultibodyStates, dtype=np.float32) -> np.ndarray:

        position = state.position
        orientation = state.orientation
        linear_velocity = state.velocity
        angular_velocity = state.angular_velocity
        joint_angles = [joint_state.angle for joint_state in state.joint_states.values() if joint_state is not None]
        joint_angular_velocities = [
            joint_state.angular_velocity for joint_state in state.joint_states.values() if joint_state is not None
        ]

        return np.array(
            position + orientation + linear_velocity + angular_velocity + joint_angles + joint_angular_velocities,
            dtype=dtype,
        )

    def step(self, action: np.ndarray) -> Tuple[np.ndarray, float, bool, bool, dict]:
        """Take one step in the environment and perform an action"""

        # perform action
        response = self._kesko.step(ApplyControl(self.spider_body.id, action))

        # get state
        state = self._get_state(response)

        body_collision = response.get_collision_with_body(self.spider_body.entity)
        reward = self._calc_reward(state, body_collision)

        done = True if body_collision is not None else False
        if self.max_steps is not None:
            if self.step_count > self.max_steps:
                done = True

        self.step_count += 1

        state = self._to_numpy(state)
        return state, reward, done, done, {}

    def _calc_reward(self, state: MultibodyStates, collision: Optional[CollisionStarted]) -> float:

        position = np.array(state.position)
        if self.past_states_queue.full():

            past_state = self.past_states_queue.get()
            self.past_states_queue.put(state)

            # reward that encourage movement in the positive x direction
            position = np.array(state.position)
            past_position = np.array(past_state.position)
            # TODO: Send the dt from Kesko and use here instead of a hardcoded value
            reward = (position[0] - past_position[0]) * 60 / self.reward_step_length
            # limit reward
            self.reward_move = np.maximum(np.minimum(100.0, reward), -100.0)

            # positive reward for using large movements
            angles = np.array([js.angle for js in state.joint_states.values() if js is not None])
            past_angles = np.array([js.angle for js in past_state.joint_states.values() if js is not None])
            self.reward_large_movements = np.linalg.norm(angles - past_angles) * self.large_movement_reward_factor

            # punish large accelerations in joints
            angvel = np.array([js.angular_velocity for js in state.joint_states.values() if js is not None])
            past_angvel = np.array([js.angular_velocity for js in past_state.joint_states.values()])
            self.reward_acceleration = -np.linalg.norm(angvel - past_angvel) * self.joint_acceleration_reward_factor
        else:
            # need to gather more states
            self.past_states_queue.put(state)
            self.reward_move = 0.0
            self.reward_acceleration = 0.0
            self.reward_large_movements = 0.0

        # If the torso have not collided give some reward
        self.reward_survive = 1.0 if collision is None else 0.0

        return float(self.reward_move + self.reward_survive + self.reward_acceleration + self.reward_large_movements)

    def reset(self, seed: Optional[int] = None, options: Optional[dict] = None) -> Tuple[np.ndarray, dict]:
        """Reset the environment to it's initial state"""
        super().reset(seed=seed)
        self._kesko.send([PausePhysics(), DespawnAll()])
        self.step_count = 0
        return self._setup()

    def seed(self, seed: Optional[int]):
        super().reset(seed=seed)

    def close(self):
        """Shutdown Kesko and the backend"""
        self._kesko.close()

    def _get_state(self, response: KeskoResponse) -> MultibodyStates:
        """Extract state from response"""
        state = response.get_state_for_body(self.spider_body.name)
        if state is None:
            raise ValueError("Could not get state")
        return state
