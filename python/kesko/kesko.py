import logging
from time import sleep
import subprocess
from typing import Optional

from .config import KESKO_BIN_PATH, URL
from .protocol import Communicator, KeskoRequest, GetStateAction, CloseAction


logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',  level=logging.DEBUG)


class Kesko:

    def __init__(self) -> None:
        self.com = Communicator(url=URL)

    def initialize(self):
        subprocess.Popen(KESKO_BIN_PATH)

    def step(self, actions: Optional[list] = None):
         
        extra_actions = actions if actions is not None else []
        
        res = self.com.request(KeskoRequest([GetStateAction()] + extra_actions))
        logging.info(res.content)
    
    def close(self):
        # send close command to Nora
        try:
            return self.com.request(KeskoRequest([CloseAction()]))
        except Exception as e:
            logging.error(e)
