from pathlib import Path

from pykesko import Kesko, KeskoModel
from pykesko.backend.backend import BackendType
from pykesko.protocol.commands import PublishFlatBuffers
from pykesko.color import Color

import flatbuffers
from messages.kesko.placeholder_box import Vec3, Transform, SpawnPlaceholderBox


if __name__ == "__main__":
    kesko = Kesko(backend_type=BackendType.TCP)
    kesko.initialize()
    
    builder = flatbuffers.Builder(1024)
    name = builder.CreateString("Boxy")
    SpawnPlaceholderBox.Start(builder)
    SpawnPlaceholderBox.AddName(builder, name)
    transform = Transform.CreateTransform(builder, 0.0, 1.0, 0.0, 0.0, 1.0, 0.3, 0.5, 2.0, 1.0)
    SpawnPlaceholderBox.AddTransform(builder, transform)
    SpawnPlaceholderBox.AddColor(builder, Vec3.CreateVec3(builder, 1.0, 0.0, 0.0))

    message = SpawnPlaceholderBox.End(builder)
    builder.Finish(message)

    kesko.send(
        [
            PublishFlatBuffers(data=builder.Output()),
        ]
    )

    for _ in range(1000):
        kesko.step()

    kesko.close()