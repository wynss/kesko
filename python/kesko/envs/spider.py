from typing import Union, Optional

import torch
import gym
import numpy as np

from ..kesko import Kesko
from ..protocol import GetState, RunPhysics, SpawnAction, ApplyControlAction, MULTIBODY_STATES
from ..color import Color
from ..model import KeskoModel


BODY_NAME = "spider-0"


class SpiderEnv(gym.Env):
    def __init__(self, device: Optional[Union[str, torch.device]] = None): 

        self.device = device if device is not None else torch.device("cpu")
        
        self._kesko = Kesko()
        self._kesko.initialize()

        # Spawn models and start physics
        self._kesko.send([
            SpawnAction(model=KeskoModel.Plane, position=[0.0, 0.0, 0.0], color=Color.WHITE),
            SpawnAction(model=KeskoModel.Spider, position=[0.0, 2.0, 0.0], color=Color.GREEN),
            RunPhysics
        ])

        # get initial state 
        initial_state = self._kesko.send(GetState)[0][MULTIBODY_STATES][0]
        tensor_state = self._to_tensor(initial_state)
        
        # Kesko stores all the bodies that are in the environment, get the body named by name.
        if BODY_NAME in self._kesko.bodies:
            body = self._kesko.bodies[BODY_NAME]
            self.spider_name = body.name
            self.spider_joints = body.joints
        else:
            self.close()
            raise ValueError("Could not get body from Kesko")

        # TODO: Send the limits from Kesko
        low = -np.pi / 8.0
        high = np.pi / 8.0
        
        # Define actions space
        dim_actions_space = len(self.spider_joints)
        self.action_space = gym.spaces.Box(low=low* np.ones((dim_actions_space,)), high=high * np.zeros((dim_actions_space,)))

        self.observation_space = gym.spaces.Space(tensor_state.shape)

    def _to_tensor(self, state: dict):

        position = state["global_position"]
        orientation = state["global_orientation"]
        angular_velocity = state["global_angular_velocity"]
        joint_positions = [state['angle'] for state in state["joint_states"].values()]

        state_tensor = torch.FloatTensor(position + orientation + angular_velocity + joint_positions, device=self.device)
        return state_tensor

        
    def step(self, action: Union[np.ndarray, torch.Tensor]):

        state = self._kesko.step(ApplyControlAction(self.spider_name, action))[0][MULTIBODY_STATES][0]

        # TODO: Distance moved during one step
        reward = None

        # TODO: Kesko need support to send collision events back
        terminated = False
        done = False

        state = self._to_tensor(state)
        return state, reward, terminated, done, {}
    
    def reset(self):
        # TODO: Kesko needs to support despawning before implementing this
        pass
    
    def close(self):
        self._kesko.close()
