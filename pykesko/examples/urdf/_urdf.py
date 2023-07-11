
from pykesko import Kesko, KeskoModel
from pykesko.backend.backend import BackendType
from pykesko.protocol.commands import Spawn, SpawnAsset, SpawnUrdf
from pykesko.color import Color


if __name__ == "__main__":
    kesko = Kesko(backend_type=BackendType.BINDINGS)
    kesko.initialize()
    
    urdf_path = "/home/azazdeaz/repos/temp/urdf-viz/crane7.urdf"
    package_map = {
        "crane_x7_description": "/home/azazdeaz/repos/art-e-fact/wizard_separate_tests/gen/crane_x7_test_project/src/crane_x7_description",
        "sciurus17_description": "/home/azazdeaz/repos/art-e-fact/wizard_separate_tests/gen/crane_x7_test_project/src/sciurus17_ros/sciurus17_description",
    }

    kesko.send(
        [
            SpawnUrdf(urdf_path=urdf_path, package_map=package_map, position=[0.0, 0.0, 0.0]),
        ]
    )

    for _ in range(1000):
        kesko.step()

    kesko.close()