from typing import Union, Optional

import torch
import gym
import numpy as np

from ..kesko import Kesko
from ..protocol.commands import DespawnAll, GetState, PausePhysics, RunPhysics, Spawn, ApplyControl
from ..protocol.response import KeskoResponse, MultibodyStates
from ..color import Color
from ..model import KeskoModel


class SpiderEnv(gym.Env):
    def __init__(self, max_steps: Optional[int] = None, device: Optional[Union[str, torch.device]] = None): 

        self.device = device if device is not None else torch.device("cpu")
        self.max_steps = max_steps
        self.step_count = 0
        self.prev_position: Optional[torch.Tensor] = None
        
        self._kesko = Kesko()
        self._kesko.initialize()
        self._setup()
    
    def _setup(self):

        # Spawn models and start physics
        self._kesko.send([
            Spawn(model=KeskoModel.Plane, position=[0.0, 0.0, 0.0], color=Color.WHITE),
            Spawn(model=KeskoModel.Spider, position=[0.0, 2.0, 0.0], color=Color.GREEN)
        ])

        # Kesko stores all the bodies that are in the environment, get the body named by base name.
        try:
            self.spider_body = [body for body in self._kesko.bodies.values() if 'spider' in body.name][0]
        except IndexError as e:
            self.close()
            raise ValueError(f"Could not get body from Kesko: {e}")

        # get initial state 
        initial_state = self._get_state(self._kesko.send(GetState))
        tensor_state = self._to_tensor(initial_state)

        # start physics
        self._kesko.send(RunPhysics)

        # TODO: Send the limits from Kesko
        low = -np.pi / 6.0
        high = np.pi / 6.0
        
        # Define spaces
        dim_actions_space = len(self.spider_body.links)
        self.action_space = gym.spaces.Box(low=low* np.ones((dim_actions_space,)), high=high * np.zeros((dim_actions_space,)))
        self.observation_space = gym.spaces.Space(tensor_state.shape)

        return tensor_state

    def _to_tensor(self, state: MultibodyStates):
        """Convert state to tensor"""
        position = state.global_position
        orientation = state.global_orientation
        angular_velocity = state.global_angular_velocity
        joint_positions = [joint_state.angle for joint_state in state.joint_states.values()]

        state_tensor = torch.FloatTensor(
            position + orientation + angular_velocity + joint_positions, 
            device=self.device
        )
        return state_tensor

        
    def step(self, action: Union[np.ndarray, torch.Tensor]):

        # perform action
        response = self._kesko.step(ApplyControl(self.spider_body.name, action))

        state = self._get_state(response)
        body_collision = response.get_collision_with_body(self.spider_body.id)

        # calc reward, distance moved from last step. only considering the horizontal movement
        if self.prev_position is None:
            reward = 0
        else:
            position = torch.Tensor(state.global_position)
            reward = (position[0, 2] - self.prev_position[0, 2]).pow(2).sum().sqrt()
            self.prev_position = position

        state = self._to_tensor(state)

        done = True if body_collision is not None else False
        if self.max_steps is not None:
            if self.step_count > self.max_steps:
                done = True

        self.step_count += 1

        return state, reward, done, {}
    
    def reset(self):
        self._kesko.send([PausePhysics, DespawnAll])
        self.step_count = 0
        return self._setup(), {}
    
    def close(self):
        self._kesko.close()

    def _get_state(self, response: KeskoResponse) -> MultibodyStates:
        state = response.get_state_for_body(self.spider_body.name)
        if state is None:
            raise ValueError("Could not get state")
        return state
