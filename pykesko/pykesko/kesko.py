import logging
from typing import Any, Optional, Union

import numpy as np

from .config import URL
from .backend import Backend, TcpBackend, BindingBackend, RenderMode, BackendType
from .protocol.commands import ApplyControl, Despawn, DespawnAll, GetState, Command
from .protocol.response import KeskoResponse, MultibodySpawned


logging.basicConfig(format="%(asctime)s %(levelname)s: %(message)s", level=logging.INFO)
logger = logging.getLogger(__name__)


class Kesko:
    """Class responsible to communicate with Kesko"""

    def __init__(self, render_mode: RenderMode = RenderMode.WINDOW, backend_type: BackendType = BackendType.TCP) -> None:

        self.render_mode = render_mode
        if backend_type == BackendType.TCP:
            self.backend: Backend = TcpBackend(url=URL)
        else:
            self.backend: Backend = BindingBackend()

        # holds the bodies and their joints
        self.bodies: dict[int, MultibodySpawned] = {}

    def initialize(self):
        self.backend.initialize(self.render_mode)

    def send(self, commands: Union[list[Command], Command]) -> KeskoResponse:

        if not isinstance(commands, list):
            commands = [commands]
        commands = self._prepare_commands(commands)
        response = self.backend.step(commands)

        # Update the body dict if we have spawned a new body
        for resp in response.responses:
            if isinstance(resp, MultibodySpawned):
                self.bodies[resp.id] = resp

        return response

    def step(self, commands: Optional[Union[list, Any]] = None) -> KeskoResponse:
        if commands is not None:
            if not isinstance(commands, list):
                commands = [commands]

        commands = [GetState()] + commands if commands else [GetState()]
        return self.send(commands)

    def _prepare_commands(self, actions: list):
        for action in actions:
            if isinstance(action, ApplyControl):
                if isinstance(action.values, np.ndarray):
                    # convert array to dict
                    action.values = {
                        joint_id: val for joint_id, val in zip(self.bodies[action.body_id].joints, action.values.tolist())
                    }
            elif isinstance(action, DespawnAll) or action == DespawnAll:
                self.bodies = {}
            elif isinstance(action, Despawn):
                self.bodies.pop(action.id)

        return actions

    def close(self):
        self.backend.close()
