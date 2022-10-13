import logging
import subprocess
from typing import Optional
import json

import numpy as np
import torch

from .config import KESKO_BIN_PATH, URL
from .protocol import (
    LINKS, ApplyControlAction, Communicator, KeskoRequest, 
    GetState, Shutdown,
    JOINT_STATES, MULTIBODY_STATES, NAME, MULTIBODY_SPAWNED
)


logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',  level=logging.DEBUG)
logger = logging.getLogger(__name__)


class Multibody:
    def __init__(self, name: str, joints: list[str]):
        self.name = name
        self.joints = joints


class Kesko:

    def __init__(self) -> None:
        self.com = Communicator(url=URL)
        
        # holds the bodies and there joints
        self.bodies: dict[str, Multibody] = {}

    def initialize(self):
        subprocess.Popen(KESKO_BIN_PATH)
        
    def send(self, actions: list) -> Optional[list]:
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
        logger.debug(json_response)
        self._parse_response(json_response)
        
        return json_response

    def step(self, actions: Optional[list] = None):
        
        if actions is not None:
            if not isinstance(actions, list):
                actions = [actions]
         
        extra_actions = actions if actions is not None else []
        return self.send([GetState] + extra_actions)
    
    def _prepare_actions(self, actions: list):
        for action in actions:
            if isinstance(action, ApplyControlAction):
                if isinstance(action.values, (np.ndarray, torch.Tensor)):
                    # convert tensor or array to dict
                    action.values = {joint_name: val for joint_name, val in zip(self.bodies[action.name].joints, action.values.tolist())}

        return actions
    
    def _parse_response(self, response):
        for rp in response:
            if isinstance(rp, dict):
                if MULTIBODY_SPAWNED in rp:
                    body = rp[MULTIBODY_SPAWNED]
                    if body[NAME] not in self.bodies:
                        # Add body info
                        name = body[NAME]
                        self.bodies[name] = Multibody(name, body[LINKS])
    
    def get_body_name(self, idx: int) -> Optional[str]:
        return list(self.bodies.keys())[idx]
    
    def close(self):
        # send close command to Nora
        try:
            return self.send(Shutdown)
        except Exception as e:
            logging.error(e)
