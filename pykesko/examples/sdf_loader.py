import time

from pykesko import Kesko, RenetServerWorker
from pykesko.backend.backend import BackendType
from pykesko.protocol.commands import PublishFlatBuffers
from pykesko.color import Color

import flatbuffers
from messages.kesko.sdf_model_loader import SpawnSdfModel, Transform

def spawn_sdf_msg(sdf_path) -> bytearray: 
    builder = flatbuffers.Builder(1024)
    sdf_path = builder.CreateString(sdf_path)
    SpawnSdfModel.Start(builder)
    SpawnSdfModel.AddSdfPath(builder, sdf_path)
    SpawnSdfModel.AddTransform(builder, Transform.CreateTransform(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 0.3, 1.0))

    message = SpawnSdfModel.End(builder)
    builder.Finish(message, b"SLSP")

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
    connection.send(spawn_sdf_msg("/home/azazdeaz/repos/art-e-fact/openai-quickstart-python/.roboprop/models/LowPolyBananaTriple"))
    for i in range(9999999999):
        
        time.sleep(1.0)

        message = connection.try_receive()
        if message is not None:
            print(message)
