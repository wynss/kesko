from time import sleep
from pathlib import Path
import subprocess

import requests


NORA_PATH = "/Users/toniaxelsson/dev/projects/nora/target/release/nora_tcp" 

class Nora:

    def __init__(self) -> None:
        self.connection = None

    def initialize(self):
        subprocess.Popen(NORA_PATH)

    def connect(self):
        print("Trying to make a request")
        requests.get("http://localhost:8080")



if __name__ == "__main__":

    nora = Nora()
    nora.initialize()
    sleep(3.0)
    nora.connect()
