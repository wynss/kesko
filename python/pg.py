from kesko.kesko import Kesko
from kesko.protocol import SpawnAction
from kesko.color import Color
from kesko.model import KeskoModel


if __name__ == "__main__":
    kesko = Kesko()
    kesko.initialize()
    kesko.step([
        SpawnAction(model=KeskoModel.Plane, position=[0.0, 0.0, 0.0], color=Color.WHITE),
        SpawnAction(model=KeskoModel.Spider, position=[1.0, 2.0, -1.0], color=Color.RED)
    ])

    for i in range(10000):
        state = kesko.step()
        print(state)
            
    kesko.close()
    