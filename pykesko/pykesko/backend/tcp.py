from typing import Optional
from multiprocessing import Process
import logging
import json

from ..pykesko import run_kesko_tcp
from .backend import RenderMode
from ..protocol.communicator import Communicator
from ..protocol.commands import Shutdown, Command
from ..protocol.request import KeskoRequest
from ..protocol.response import (
    KeskoResponse,
    MultibodySpawned,
    MultibodyStates,
    CollisionStarted,
    CollisionStopped,
)


logger = logging.getLogger(__name__)


class TcpBackend:
    def __init__(self, url: str):
        self.com = Communicator(url=url)
        self.process: Optional[Process] = None

    def initialize(self, render_mode: RenderMode):
        self.process = Process(
            target=run_kesko_tcp,
            args=[
                render_mode == RenderMode.WINDOW,
            ],
        )
        self.process.start()

    def close(self):
        try:
            resp = self.step([Shutdown()])
            self.com.sess.close()
            logger.info("Closing down...")
            return resp
        except Exception as e:
            logging.error(e)

        if self.process is not None:
            self.process.join()

    def step(self, commands: list[Command]) -> KeskoResponse:
        response = self.com.request(KeskoRequest(commands))
        if response is None:
            self.close()
            raise ValueError("Response was None")

        logger.debug(f"Got response {json.dumps(response.json(), indent=4)}")

        # Because we get some strange things from the Serialization on Kesko's side
        json_response = [resp[-1] for resp in response.json()]
        return self._parse_response(json_response)

    def _parse_response(self, json_response) -> KeskoResponse:
        """Parses the responses and deserializes them into their corresponding dataclass"""

        response_objs = []
        for response in json_response:

            if MultibodySpawned.__name__ in response:
                multibody = MultibodySpawned(**response[MultibodySpawned.__name__])
                response_objs.append(multibody)

            elif CollisionStarted.__name__ in response:
                collision_started = CollisionStarted(**response[CollisionStarted.__name__])
                response_objs.append(collision_started)

            elif CollisionStopped.__name__ in response:
                collision_stopped = CollisionStopped(**response[CollisionStopped.__name__])
                response_objs.append(collision_stopped)

            elif MultibodyStates.__name__ in response:
                multibody_states = [MultibodyStates(**mb) for mb in response[MultibodyStates.__name__]]
                response_objs.extend(multibody_states)

        return KeskoResponse(response_objs)
