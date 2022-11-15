import pprint
from zipfile import ZipFile

path = "pykesko/dist/pykesko-0.0.2-cp39-cp39-linux_x86_64.whl"
names = ZipFile(path).namelist()
pprint.pprint(names)
