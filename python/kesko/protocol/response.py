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


class JointInfo(BaseModel):
    name: str
    type: str
    axis: str
    limits: list
    damping: float
    stiffness: float
    max_motor_force: float
    

class JointState(BaseModel):
    type: str
    axis: str
    angle: float
    angular_velocity: float


class MultibodySpawned(BaseModel):
    id: int
    name: str
    joints: dict[int, JointInfo]


class RigidBodySpawned(BaseModel):
    id: int
    name: str


class MultibodyStates(BaseModel):
    name: str
    global_position: list
    global_orientation: list
    global_angular_velocity: list
    relative_positions: dict[str, list[float]]
    joint_states: dict[str, JointState]


class KeskoResponse:
    """
    Holds responses from a request to Kesko. This class is meant to have some convenient methods
    when it comes to get responses for certain conditions
    """

    def __init__(self, responses: list):
        self.responses = responses

    def get_state_for_body(self, name: str) -> Optional[MultibodyStates]:
        """Returns the state for a given body if any"""
        for resp in self.responses:
            if isinstance(resp, MultibodyStates):
                if resp.name == name:
                    return resp
        return None

    def get_collision_with_body(self, id: int) -> Optional[CollisionStarted]:
        """Return the collision response for a given body if any"""
        for resp in self.responses:
            if isinstance(resp, CollisionStarted):
                if resp.entity1 == id or resp.entity2 == id:
                    return resp

        return None
