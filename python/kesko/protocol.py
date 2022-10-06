import logging
from typing import Union

import requests
from requests.adapters import HTTPAdapter, Retry

from .color import Rgba, Color
from .model import KeskoModel


logger = logging.getLogger(__name__)

MULTIBODY_NAME = "name"
JOINT_STATES = "joint_states"
MULTIBODY_STATES = "multibody_states"
GLOBAL_POSITION = "global_position"


class SpawnAction:
    def __init__(self, model: KeskoModel, position: list[int], color: Union[Rgba, Color]):
        self._model = model
        self._position = position
        self._color = color
    
    def to_json(self):
        return {
            "SpawnModel": {
                "model": self._model.name,
                "position": self._position,
                "color": self._color.to_json()
            }
        }
 
class CloseAction:
    def to_json(self):
        return "Close"


class GetStateAction:
    def to_json(self):
        return "GetState"
    
    
class ApplyControlAction:
    def __init__(self, values):
        self.values = values
    
    def to_json(self):
        return {
            "ApplyControl": {
                "body_name": self.name,
                "values": self.values
            }
        }

class PausePhysics:
    def to_json(self):
        return "PausePhysics"

class RunPhysics:
    def to_json(self):
        return "RunPhysics"


class KeskoRequest:
    def __init__(self, actions):
        self._actions = actions
    
    def to_json(self):
        return {
            "actions": [action.to_json() for action in self._actions]
        }


class Communicator:
    def __init__(self, url: str, max_retries: int = 5, backoff_factor: float = 0.5):
        self.url = url
        self.sess = requests.Session()
        max_retries = Retry(total=max_retries, backoff_factor=backoff_factor)
        self.sess.mount("http://", HTTPAdapter(max_retries=max_retries))
        
    def request(self, request: KeskoRequest):
        try:
            msg = request.to_json()
            logger.debug(f"Sending {msg}")
            res = self.sess.get(self.url, json=msg)
            return res
        except Exception as e:
            logger.error(e, exc_info=True)

if __name__ == '__main__':
    request = KeskoRequest([GetStateAction(), CloseAction()])
    print(request.to_json())
        