import logging
from time import sleep
import subprocess
from typing import Optional
from urllib import response

from .config import KESKO_BIN_PATH, URL
from .protocol import (
    Communicator, KeskoRequest, 
    GetStateAction, SpawnAction, CloseAction,
    GLOBAL_POSITION, JOINT_STATES, MULTIBODY_STATES, MULTIBODY_NAME
)


logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',  level=logging.DEBUG)


class Multibody:
    def __init__(self, name: str, joints: list[str]):
        self.name = name
        self.joint = joints


class Kesko:

    def __init__(self) -> None:
        self.com = Communicator(url=URL)
        
        # holds the bodies and there joints
        self.bodies: dict[str, Multibody]

    def initialize(self):
        subprocess.Popen(KESKO_BIN_PATH)
        
    def send(self, actions: list):
        if actions is not None:
            if not isinstance(actions, list):
                actions = [actions]
        
        response = self.com.request(KeskoRequest(actions))
        
        self._parse_response(response)
        return response.json()

    def step(self, actions: Optional[list] = None):
        
        if actions is not None:
            if not isinstance(actions, list):
                actions = [actions]
         
        extra_actions = actions if actions is not None else []
        return self.send([GetStateAction()] + extra_actions)
    
    def _parse_response(self, response):
        if MULTIBODY_STATES in response:
            for body_state in response[MULTIBODY_STATES]:
                if body_state[MULTIBODY_NAME] not in self.bodies:
                    # Add body info
                    name = body_state[MULTIBODY_NAME]
                    joint_names = [k for k, _ in body_state[JOINT_STATES].iter()] 
                    self.bodies[name]: Multibody(name, joint_names)
    
    def get_body_name(self, idx: int) -> Optional[str]:
        return list(self.bodies.keys())[idx]
    
    def close(self):
        # send close command to Nora
        try:
            return self.com.request(KeskoRequest([CloseAction()]))
        except Exception as e:
            logging.error(e)
