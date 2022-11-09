import logging

import requests
from requests.adapters import HTTPAdapter, Retry

from .request import KeskoRequest
from .commands import GetState, Shutdown


logger = logging.getLogger(__name__)


class Communicator:
    def __init__(self, url: str, retries: int = 5, backoff_factor: float = 0.5):
        self.url = url
        self.sess = requests.Session()
        retries = Retry(total=retries, backoff_factor=backoff_factor)
        self.sess.mount("http://", HTTPAdapter(max_retries=retries))

    def request(self, request: KeskoRequest):
        msg = request.to_json()
        logger.debug(f"Sending {msg}")
        res = self.sess.get(self.url, json=msg)
        return res


if __name__ == "__main__":
    request = KeskoRequest([GetState(), Shutdown()])
    print(request.to_json())
