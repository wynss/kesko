import torch
import gym
import numpy as np

from ..kesko import Kesko
from ..protocol import RunPhysics, SpawnAction, ApplyControlAction
from ..color import Color
from ..model import KeskoModel


class SpiderEnv:
    def __init__(self):
        
        self.action_space = gym.spaces.Box(low=np.zeros((2,)), high=np.zeros((2,)))
        
        self._kesko = Kesko()
        self._kesko.initialize()
        self._kesko.send([
            SpawnAction(model=KeskoModel.Plane, position=[0.0, 0.0, 0.0], color=Color.WHITE),
            SpawnAction(model=KeskoModel.Spider, position=[0.0, 2.0, 0.0], color=Color.RED)
        ])
        self._kesko.send(RunPhysics())
        
    def step(self, action: torch.Tensor):
         self._kesko.step()
    
    def reset(self):
        pass
    
    def close(self):
        self._kesko.close()
