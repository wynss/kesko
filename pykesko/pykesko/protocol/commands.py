from typing import Union, Protocol

import numpy as np

from ..color import Rgba, Color
from ..pykesko import Model


class Command(Protocol):
    def to_json(self) -> Union[dict, str]:
        ...


class CheckAlive:
    def to_json(self):
        return "IsAlive"


class Spawn:
    def __init__(self, model: Model, position: list[float], color: Union[Rgba, Color]):
        self.model = model
        self.position = position
        self.color = color

    def to_json(self):
        return {
            "SpawnModel": {
                "model": self.model.name,
                "position": self.position,
                "color": self.color.to_json(),
            }
        }


class Despawn:
    def __init__(self, id: int):
        self.id = id

    def to_json(self):
        return {"Despawn": {"id": self.id}}


class DespawnAll:
    def to_json(self):
        return "DespawnAll"


class Shutdown:
    def to_json(self):
        return "Close"


class GetState:
    def to_json(self):
        return "GetState"


class ApplyControl:
    def __init__(
        self,
        body_id: int,
        values: Union[dict[np.uint64, float], np.ndarray],
    ):
        self.body_id = body_id
        self.values = values

    def to_json(self):
        return {"ApplyMotorCommand": {"id": self.body_id, "command": self.values}}


class PausePhysics:
    def to_json(self):
        return "PausePhysics"


class RunPhysics:
    def to_json(self):
        return "RunPhysics"
