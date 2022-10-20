import logging

import requests
from requests.adapters import HTTPAdapter, Retry

from .request import KeskoRequest
from .commands import GetState, Shutdown


logger = logging.getLogger(__name__)

class Communicator:
    def __init__(self, url: str, max_retries: int = 5, backoff_factor: float = 0.5):
        self.url = url
        self.sess = requests.Session()
        max_retries = Retry(total=max_retries, backoff_factor=backoff_factor)
        self.sess.mount("http://", HTTPAdapter(max_retries=max_retries))
        
    def request(self, request: KeskoRequest):
        msg = request.to_json()
        logger.debug(f"Sending {msg}")
        res = self.sess.get(self.url, json=msg)
        return res

if __name__ == '__main__':
    request = KeskoRequest([GetState(), Shutdown()])
    print(request.to_json())
