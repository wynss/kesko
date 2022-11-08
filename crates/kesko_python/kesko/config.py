from pathlib import Path


IP = "http://localhost"
PORT = 8080
URL = f"{IP}:{PORT}"

ROOT = Path(__file__).parent.parent.parent.parent

KESKO_BIN_PATH = ROOT / "target" / "release" / "kesko_tcp"
KESKO_HEADLESS_BIN_PATH = ROOT / "target" / "release" / "kesko_tcp_headless"
