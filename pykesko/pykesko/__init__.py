from pykesko.kesko import Kesko

# import the the rust bindings
from .pykesko import KeskoApp as _KeskoApp
from .pykesko import Model as KeskoModel
from .pykesko import run_kesko_tcp

from gym.envs.registration import register

register(id="Spider-v0", entry_point="pykesko.envs:SpiderEnv")
