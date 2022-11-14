import json

from ..color import Color, Rgba
from .backend import RenderMode
from ..protocol.commands import (
    ApplyControl,
    Command,
    DespawnAll,
    PausePhysics,
    RunPhysics,
    Spawn,
    Despawn,
)
from ..protocol.response import (
    CollisionStarted,
    CollisionStopped,
    KeskoResponse,
    MultibodyStates,
    MultibodySpawned,
)
from ..pykesko import KeskoApp


class BindingBackend:
    def __init__(self):
        self.kesko = KeskoApp()

    def initialize(self, render_mode: RenderMode):
        if render_mode == RenderMode.HEADLESS:
            self.kesko.init_headless()
        elif render_mode == RenderMode.WINDOW:
            self.kesko.init_default()

    def step(self, commands: list[Command]) -> KeskoResponse:

        # apply all the commands
        for command in commands:

            if isinstance(command, DespawnAll):
                self.kesko.despawn_all()

            elif isinstance(command, Despawn):
                self.kesko.despawn(command.id)

            elif isinstance(command, Spawn):
                if isinstance(command.color, Color):
                    color = command.color.value.to_list()
                elif isinstance(command.color, Rgba):
                    color = command.color.to_list()
                else:
                    raise ValueError(f"Spawn had an invalid color type, {type(command.color)}")

                self.kesko.spawn(model=command.model, position=command.position, color=color)

            elif isinstance(command, RunPhysics):
                self.kesko.start_physics()

            elif isinstance(command, PausePhysics):
                self.kesko.stop_physics()

            elif isinstance(command, ApplyControl):
                self.kesko.apply_motor_commands(command.values)

        # step simulation
        self.kesko.step()

        # Get responses
        responses = []

        # body states
        body_states = json.loads(self.kesko.get_multibody_state())
        multibody_states = [MultibodyStates(**mb) for mb in body_states]
        responses.extend(multibody_states)

        # physics events
        if (physic_events := self.kesko.get_physics_events()) is not None:
            physic_events = json.loads(physic_events)
            for ev in physic_events:
                if MultibodySpawned.__name__ in ev:
                    multibody = MultibodySpawned(**ev[MultibodySpawned.__name__])
                    responses.append(multibody)

        # Collision events
        if (collision_events := self.kesko.get_collisions()) is not None:
            collision_events = json.loads(collision_events)
            for ev in collision_events:
                if CollisionStarted.__name__ in ev:
                    collision_started = CollisionStarted(**ev[CollisionStarted.__name__])
                    responses.append(collision_started)

                elif CollisionStopped.__name__ in ev:
                    collision_stopped = CollisionStopped(**ev[CollisionStopped.__name__])
                    responses.append(collision_stopped)

        return KeskoResponse(responses)

    def close(self):
        self.kesko.close()
