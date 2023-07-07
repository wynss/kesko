from pathlib import Path

from pykesko import Kesko, KeskoModel
from pykesko.backend.backend import BackendType
from pykesko.protocol.commands import PublishFlatBuffers
from pykesko.color import Color

import flatbuffers
from messages.kesko.urdf import SpawnUrdf, Vec3, PackageMap


if __name__ == "__main__":
    kesko = Kesko(backend_type=BackendType.TCP)
    kesko.initialize()
    
    here = Path(__file__).parent
    package_map = {
        "crane_x7_description": str(here / "crane_x7_description"),
    }

    builder = flatbuffers.Builder(1024)
    package_name = builder.CreateString("crane_x7_description")
    package_path = builder.CreateString(str(here / "crane_x7_description"))
    PackageMap.Start(builder)
    PackageMap.AddPackageName(builder, package_name)
    PackageMap.AddPackagePath(builder, package_path)
    package_map = PackageMap.PackageMapEnd(builder)

    SpawnUrdf.StartPackageMappingsVector(builder, 1)
    builder.PrependUOffsetTRelative(package_map)
    package_map = builder.EndVector(1)

    urdf_path = builder.CreateString(str(here / "crane_x7.urdf"))
    SpawnUrdf.Start(builder)
    SpawnUrdf.AddPosition(builder, Vec3.CreateVec3(builder, 0.0, 0.0, 0.0))
    SpawnUrdf.AddUrdfPath(builder, urdf_path)
    SpawnUrdf.AddPackageMappings(builder, package_map)

    # Finish the SpawnUrdf message and get its byte representation
    spawn_urdf = SpawnUrdf.SpawnUrdfEnd(builder)
    builder.Finish(spawn_urdf)

    # Get the byte array representation
    buf = builder.Output()
    print(buf)


    print(package_map)

    kesko.send(
        [
            PublishFlatBuffers(data=buf),
        ]
    )

    for _ in range(1000):
        kesko.step()

    kesko.close()