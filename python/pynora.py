import logging
from time import sleep
from pathlib import Path
import subprocess

import requests
from requests.adapters import HTTPAdapter, Retry

logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s', level=logging.INFO)


NORA_PATH = "/Users/toniaxelsson/dev/projects/nora/target/release/nora_tcp"
URL = "http://localhost:8080" 

class Nora:

    def __init__(self) -> None:
        self.sess = requests.Session()
        retries = Retry(total=5, backoff_factor=0.5)
        self.sess.mount("http://", HTTPAdapter(max_retries=retries))

    def initialize(self):
        subprocess.Popen(NORA_PATH)

    def step(self):
        res = self.sess.get(URL, json={
            'action': 'get_state'
        })
        logging.info(res.content)

    
    def close(self):
        # send close command to Nora
        try:
            self.sess.get(URL, json={
                'action': 'close'
            })
        except Exception as e:
            print(e)


if __name__ == "__main__":

    nora = Nora()
    nora.initialize()

    for _ in range(1000):
        nora.step()

    nora.close()
