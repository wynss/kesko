from pathlib import Path

from pykesko import Kesko, KeskoModel
from pykesko.backend.backend import BackendType
from pykesko.protocol.commands import Spawn, SpawnAsset, SpawnUrdf
from pykesko.color import Color


if __name__ == "__main__":
    kesko = Kesko(backend_type=BackendType.BINDINGS)
    kesko.initialize()
    
    here = Path(__file__).parent
    urdf_path = str(here / "crane_x7.urdf")
    package_map = {
        "crane_x7_description": str(here / "crane_x7_description"),
    }

    print(package_map)

    kesko.send(
        [
            SpawnUrdf(urdf_path=urdf_path, package_map=package_map, position=[0.0, 0.0, 0.0]),
        ]
    )

    for _ in range(1000):
        kesko.step()

    kesko.close()