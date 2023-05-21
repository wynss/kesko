import logging

from pykesko import Kesko
from pykesko.backend import Backend, RenderMode


if __name__ == "__main__":
    kesko = Kesko(backend_type=Backend.TCP, render_mode=RenderMode.WINDOW)
    kesko.initialize()

    for i in range(1000):
        kesko.step()

    kesko.close()
