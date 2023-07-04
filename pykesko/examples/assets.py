from pykesko import Kesko, KeskoModel
from pykesko.backend.backend import BackendType
from pykesko.protocol.commands import Spawn, SpawnAsset, SpawnUrdf
from pykesko.color import Color


if __name__ == "__main__":
    kesko = Kesko(backend_type=BackendType.TCP)
    kesko.initialize()

    kesko.send(
        [
            SpawnAsset(
                asset_path="/home/azazdeaz/repos/temp/bevy/assets/models/FlightHelmet/FlightHelmet.gltf#Scene0",
                position=[0.0, 1.0, 0.0],
            ),
            Spawn(
                model=KeskoModel.Sphere,
                position=[1.0, 0.0, 0.0],
                scale=[2.0, 2.0, 2.0],
                color=Color.RED,
            ),
            Spawn(model=KeskoModel.Arena, position=[0.0, 0.0, 0.0], color=Color.WHITE),
        ]
    )

    for _ in range(1000):
        kesko.step()

    kesko.close()
