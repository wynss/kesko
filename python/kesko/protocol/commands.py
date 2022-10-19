from typing import Union

import torch

from ..model import KeskoModel
from ..color import Rgba, Color


class CheckAlive:
    def to_json(self):
        return CheckAlive.to_json()
    def to_json():
        return "IsAlive"

class Spawn:
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

class Despawn:
    def __init__(self, id: int):
        self.id = id
    
    def to_json(self):
        return {
            "Despawn": {
                "id": self.id
            }
        }

class DespawnAll:
    def to_json(self):
        return DespawnAll.to_json()
    
    def to_json():
        return "DespawnAll"
 
class Shutdown:
    def to_json(self):
        return Shutdown.to_json()
    
    def to_json():
        return "Close"


class GetState:
    def to_json(self):
        return GetState.to_json()

    def to_json():
        return "GetState"
    
    
class ApplyControl:
    def __init__(self, name: str, values: Union[dict[str, float], torch.Tensor]):
        self.name = name
        self.values = values
    
    def to_json(self):
        return {
            "ApplyMotorCommand": {
                "name": self.name,
                "command": self.values
            }
        }

class PausePhysics:
    def to_json(self):
        return PausePhysics.to_json()
    
    def to_json():
        return "PausePhysics"

class RunPhysics:
    def to_json(self):
        return RunPhysics.to_json()
    
    def to_json():
        return "RunPhysics"