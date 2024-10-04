import pickle

from PIL.Image import Image as ImageType
from PIL.ImageStat import Stat

type ColorTup = tuple[int, int, int]


# https://stackoverflow.com/questions/1574458/python-object-that-monitors-changes-in-objects
# Used to debug an issue I had
class ChangeDetector:
    def __init__(self):
        self.objects = dict()

    def detect_change(self, obj):
        current_pickle = pickle.dumps(obj, -1)
        if id(obj) in self.objects and current_pickle != self.objects[id(obj)]:
            raise ValueError("Object changed")
        self.objects[id(obj)] = current_pickle


def get_mean_color(image: ImageType, point: tuple[float, float]) -> ColorTup | None:
    RAD = 10
    minx = max(0, point[0] - RAD)
    miny = max(0, point[1] - RAD)
    maxx = min(image.width, point[0] + RAD)
    maxy = min(image.height, point[1] + RAD)

    if minx >= maxx or miny >= maxy:
        return None

    square = image.crop((minx, miny, maxx, maxy))

    r, g, b = Stat(square).mean
    return (int(r), int(g), int(b))
