from typing import Optional
from pydantic import BaseModel


class CollisionStarted(BaseModel):
    entity1: int
    entity2: int
    flag: dict[str, int]

class CollisionStopped(BaseModel):
    entity1: int
    entity2: int
    flag: dict[str, int]


class MultibodySpawned(BaseModel):
    id: int
    name: str
    links: dict[str, int]


class RigidBodySpawned(BaseModel):
    id: int
    name: str


class JointState(BaseModel):
    type: str
    axis: str
    angle: float


class MultibodyStates(BaseModel):
    name: str
    global_position: list
    global_orientation: list
    global_angular_velocity: list
    relative_positions: dict[str, list[float]]
    joint_states: dict[str, JointState]


class KeskoResponse:
    def __init__(self, responses: list):
        self.responses = responses
    
    def get_state_for_body(self, name: str) -> Optional[MultibodyStates]:
        for resp in self.responses:
            if isinstance(resp, MultibodyStates):
                if resp.name == name:
                    return resp
        return None
