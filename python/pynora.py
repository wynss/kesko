from operator import sub
from pathlib import Path
import subprocess


NORA_PATH = "/Users/toniaxelsson/dev/projects/nora/target/release/nora" 

def initialize():
    subprocess.run(NORA_PATH)


if __name__ == "__main__":
    initialize()
    print("started")