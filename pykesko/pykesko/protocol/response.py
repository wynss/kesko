from typing import Optional, Union

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
    limits: Optional[list]
    damping: float
    stiffness: float
    max_motor_force: float


class RevoluteJointState(BaseModel):
    type: str
    axis: str
    angle: float
    angular_velocity: float

class PrismaticJointState(BaseModel):
    type: str
    axis: str
    position: float
    velocity: float


class MultibodySpawned(BaseModel):
    id: int
    entity: int
    name: str
    joints: dict[int, JointInfo]


class RigidBodySpawned(BaseModel):
    id: int
    name: str


class MultibodyStates(BaseModel):
    name: str
    id: int
    position: list
    orientation: list
    velocity: list
    angular_velocity: list
    relative_positions: dict[str, list[float]]
    joint_states: dict[str, Optional[Union[RevoluteJointState, PrismaticJointState]]]


class KeskoResponse:
    """
    Holds responses from a request to Kesko. This class is meant to have some convenient methods
    when it comes to get responses for certain criterions
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

    def get_collision_with_body(self, entity: int) -> Optional[CollisionStarted]:
        """Return the collision response for a given body if any"""
        for resp in self.responses:
            if isinstance(resp, CollisionStarted):
                if resp.entity1 == entity or resp.entity2 == entity:
                    return resp

        return None
