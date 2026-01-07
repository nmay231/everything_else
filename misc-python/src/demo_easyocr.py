import sys
from pathlib import Path

import easyocr

file = Path(sys.argv[1])
assert file.exists(), f"{file} does not exist"

# print(sys.argv)
reader = easyocr.Reader(["en"], gpu=False)
print(reader.readtext(str(file.resolve()), detail=0))
