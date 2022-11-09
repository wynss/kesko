from pykesko.kesko import Kesko

# import the the rust bindings
from .pykesko import KeskoApp, Model

from gym.envs.registration import register

register(id="kesko/Spider-v0", entry_point="kesko.envs:SpiderEnv")
