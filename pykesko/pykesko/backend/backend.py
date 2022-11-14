from enum import Enum, auto
from typing import Protocol

from ..protocol.response import KeskoResponse
from ..protocol.commands import Command


class RenderMode(Enum):
    HEADLESS = auto()
    WINDOW = auto()


class BackendType(Enum):
    TCP = auto()
    BINDINGS = auto()


class Backend(Protocol):
    def initialize(self, render_mode: RenderMode):
        ...

    def step(self, commands: list[Command]) -> KeskoResponse:
        ...

    def close(self):
        ...
