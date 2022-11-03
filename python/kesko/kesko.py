import logging
from enum import Enum, auto
import subprocess
from typing import Any, Optional, Union
import json

import numpy as np
import torch

from .config import KESKO_HEADLESS_BIN_PATH, URL, KESKO_BIN_PATH
from .protocol.request import KeskoRequest
from .protocol.communicator import Communicator
from .protocol.commands import ApplyControl, Despawn, DespawnAll, GetState, Shutdown
from .protocol.response import (
    KeskoResponse,
    CollisionStarted,
    MultibodySpawned,
    CollisionStopped,
    MultibodyStates,
)


logging.basicConfig(format="%(asctime)s %(levelname)s: %(message)s", level=logging.INFO)
logger = logging.getLogger(__name__)


class KeskoMode(Enum):
    HEADLESS = auto()
    RENDER = auto()

class Kesko:
    def __init__(self) -> None:
        self.com = Communicator(url=URL)

        # holds the bodies and there joints
        self.bodies: dict[str, MultibodySpawned] = {}

    def initialize(self, mode: KeskoMode):

        if mode == KeskoMode.HEADLESS:
            subprocess.Popen(KESKO_HEADLESS_BIN_PATH)
        elif mode == KeskoMode.RENDER:
            subprocess.Popen(KESKO_BIN_PATH)

    def send(self, actions: Any) -> KeskoResponse:

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

    def step(self, actions: Optional[Union[list, Any]] = None) -> KeskoResponse:

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
                    action.values = {
                        joint.name: val
                        for joint, val in zip(
                            self.bodies[action.name].joints.values(), action.values.tolist()
                        )
                    }
            elif isinstance(action, DespawnAll) or action == DespawnAll:
                self.bodies = {}
            elif isinstance(action, Despawn):
                self.bodies.pop(action.name)

        return actions

    def _parse_response(self, responses) -> KeskoResponse:
        """Parses the responses and deserializes them into their corresponding dataclass"""

        response_objects = []
        for response in responses:

            if MultibodySpawned.__name__ in response:
                multibody = MultibodySpawned(**response[MultibodySpawned.__name__])
                if multibody.name not in self.bodies:
                    # Add body info
                    self.bodies[multibody.name] = multibody

                response_objects.append(multibody)

            elif CollisionStarted.__name__ in response:
                collision_started = CollisionStarted(
                    **response[CollisionStarted.__name__]
                )
                response_objects.append(collision_started)

            elif CollisionStopped.__name__ in response:
                collision_stopped = CollisionStopped(
                    **response[CollisionStopped.__name__]
                )
                response_objects.append(collision_stopped)

            elif MultibodyStates.__name__ in response:
                multibody_states = [
                    MultibodyStates(**mb) for mb in response[MultibodyStates.__name__]
                ]
                response_objects.extend(multibody_states)

        return KeskoResponse(response_objects)

    def close(self):
        # send close command to Nora
        try:
            resp = self.send(Shutdown)
            self.com.sess.close()
            logger.info("Closing down...")
            return resp
        except Exception as e:
            logging.error(e)
