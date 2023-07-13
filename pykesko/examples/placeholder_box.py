import time

from pykesko import Kesko, RenetServerWorker
from pykesko.backend.backend import BackendType
from pykesko.protocol.commands import PublishFlatBuffers
from pykesko.color import Color

import flatbuffers
from messages.kesko.placeholder_box import Vec3, Transform, SpawnPlaceholderBox, Clear

def spawn_model_msg(rotation) -> bytearray:
    builder = flatbuffers.Builder(1024)
    name = builder.CreateString("Boxy")
    SpawnPlaceholderBox.Start(builder)
    SpawnPlaceholderBox.AddName(builder, name)
    transform = Transform.CreateTransform(builder, 0.0, 1.0, 0.0, 0.0, rotation, 0.3, 0.5, 2.0, 1.0)
    SpawnPlaceholderBox.AddTransform(builder, transform)
    SpawnPlaceholderBox.AddColor(builder, Vec3.CreateVec3(builder, 1.0, 0.0, 0.0))

    message = SpawnPlaceholderBox.End(builder)
    builder.Finish(message, b"PBSP")

    return builder.Output()

def clear_msg() -> bytearray:
    builder = flatbuffers.Builder(1024)
    Clear.Start(builder)
    message = Clear.End(builder)
    builder.Finish(message, b"PBCL")

    return builder.Output()

# if __name__ == "__main__":
#     kesko = Kesko(backend_type=BackendType.BINDINGS)
#     kesko.initialize()

#     for rotation in range(100):
#         kesko.step()
#         kesko.send(
#         [
#             PublishFlatBuffers(data=clear_msg()),
#             PublishFlatBuffers(data=spawn_model_msg(rotation / 100.0)),
#         ]
#     )

#     kesko.close()

if __name__ == "__main__":
    connection = RenetServerWorker()
    for i in range(9999999999):
        connection.send("Message #{}".format(i))
        time.sleep(1.0)

        message = connection.try_receive()
        if message is not None:
            print(message)
