from typing import Union

import torch
import numpy as np

from ..model import KeskoModel
from ..color import Rgba, Color


class CheckAlive:
    def __init__(self):
        self.to_json = self._to_json

    def _to_json(self):
        return self.__class__.to_json()

    @staticmethod
    def to_json():
        return "IsAlive"


class Spawn:
    def __init__(
        self, model: KeskoModel, position: list[float], color: Union[Rgba, Color]
    ):
        self._model = model
        self._position = position
        self._color = color

    def to_json(self):
        return {
            "SpawnModel": {
                "model": self._model.name,
                "position": self._position,
                "color": self._color.to_json(),
            }
        }


class Despawn:
    def __init__(self, name: str):
        self.name = name

    def to_json(self):
        return {"Despawn": {"name": self.name}}


class DespawnAll:
    def __init__(self):
        self.to_json = self._to_json

    def _to_json(self):
        return self.__class__.to_json()

    @staticmethod
    def to_json():
        return "DespawnAll"


class Shutdown:
    def __init__(self):
        self.to_json = self._to_json

    def _to_json(self):
        return self.__class__.to_json()

    @staticmethod
    def to_json():
        return "Close"


class GetState:
    def __init__(self):
        self.to_json = self._to_json

    def _to_json(self):
        return self.__class__.to_json()

    @staticmethod
    def to_json():
        return "GetState"


class ApplyControl:
    def __init__(self, name: str, values: Union[dict[str, float], torch.Tensor, np.ndarray]):
        self.name = name
        self.values = values

    def to_json(self):
        return {"ApplyMotorCommand": {"name": self.name, "command": self.values}}


class PausePhysics:
    def __init__(self):
        self.to_json = self._to_json

    def _to_json(self):
        return self.__class__.to_json()

    @staticmethod
    def to_json():
        return "PausePhysics"


class RunPhysics:
    def __init__(self):
        self.to_json = self._to_json

    def _to_json(self):
        return self.__class__.to_json()

    @staticmethod
    def to_json():
        return "RunPhysics"
