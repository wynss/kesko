import logging
from os import isatty
from signal import raise_signal
import subprocess
from typing import Optional
import json

import numpy as np
import torch

from .config import KESKO_BIN_PATH, URL
from .protocol import KeskoRequest
from .protocol.communicator import Communicator
from .protocol.commands import (
    ApplyControl, Despawn, DespawnAll, GetState, Shutdown
)
from .protocol.response import KeskoResponse, CollisionStarted, MultibodySpawned, CollisionStopped, MultibodyStates


logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',  level=logging.INFO)
logger = logging.getLogger(__name__)


class Multibody:
    def __init__(self, id: int, name: str, joints: list[str]):
        self.id = id
        self.name = name
        self.joints = joints


class Kesko:

    def __init__(self) -> None:
        self.com = Communicator(url=URL)
        
        # holds the bodies and there joints
        self.bodies: dict[str, Multibody] = {}

    def initialize(self):
        subprocess.Popen(KESKO_BIN_PATH)
        
    def send(self, actions: list) -> KeskoResponse:

        if actions is not None:
            if not isinstance(actions, list):
                actions = [actions]
        
        actions = self._prepare_actions(actions)
         
        response = self.com.request(KeskoRequest(actions))
        if response is None:
            self.close()
            raise ValueError("Response was None") 

        logger.debug(f"Got response {json.dumps(response.json(), indent=4)}")

        # Because we get some strange things from the Serialization on Kesko's side
        json_response = [resp[-1] for resp in response.json()]

        response_objects = self._parse_response(json_response)
        return response_objects

    def step(self, actions: Optional[list] = None) -> KeskoResponse:
        
        if actions is not None:
            if not isinstance(actions, list):
                actions = [actions]
         
        extra_actions = actions if actions is not None else []
        return self.send([GetState] + extra_actions)
    
    def _prepare_actions(self, actions: list):
        for action in actions:
            if isinstance(action, ApplyControl):
                if isinstance(action.values, (np.ndarray, torch.Tensor)):
                    # convert tensor or array to dict
                    action.values = {joint_name: val for joint_name, val in zip(self.bodies[action.id].links, action.values.tolist())}
            elif isinstance(action, DespawnAll) or action == DespawnAll:
                self.bodies = {}
            elif isinstance(action, Despawn):
                self.bodies.pop(action.id)

        return actions
    
    def _parse_response(self, responses) -> KeskoResponse:
        """Parses the responses and deserializes them into their corresponding dataclass"""

        response_objects = []
        for response in responses:

            if MultibodySpawned.__name__ in response:
                multibody = MultibodySpawned(**response[MultibodySpawned.__name__])
                if multibody.id not in self.bodies:
                    # Add body info
                    self.bodies[multibody.id] = multibody

                response_objects.append(multibody)    

            elif CollisionStarted.__name__ in response:
                collision_started = CollisionStarted(**response[CollisionStarted.__name__])
                response_objects.append(collision_started)
                
            elif CollisionStopped.__name__ in response:
                collision_stopped = CollisionStopped(**response[CollisionStopped.__name__])
                response_objects.append(collision_stopped)
            
            elif MultibodyStates.__name__ in response:
                multibody_states = [MultibodyStates(**mb) for mb in response[MultibodyStates.__name__]]
                response_objects.extend(multibody_states)

        return KeskoResponse(response_objects)
    
    def close(self):
        # send close command to Nora
        try:
            return self.send(Shutdown)
        except Exception as e:
            logging.error(e)
