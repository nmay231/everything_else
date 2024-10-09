import pickle
from typing import Any, Callable

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


def make_json_serializer(color_map: dict[ColorTup, int]) -> Callable[[Any], Any]:
    from kami2_solver.graph import ColorGraph, Node
    from kami2_solver.solver import SolverCache, SolverStep

    def serialize(unknown: Any) -> Any:
        match unknown:
            case ColorGraph():
                nodes = [*unknown.connections.keys()]
                node_index = {node: i for i, node in enumerate(nodes)}
                connections = {
                    node_index[node]: [node_index[neighbor] for neighbor in neighbors]
                    for node, neighbors in unknown.connections.items()
                }
                return {"connections": connections, "nodes": nodes}
            case Node():
                return {"center": unknown.center, "color": color_map[unknown.color]}
            case SolverStep():
                return {
                    "current_graph": unknown.current_graph,
                    "moves": unknown.moves,
                    "cache": unknown.cache,
                    "found_a_solution": unknown.found_a_solution,
                }
            case SolverCache():
                return {
                    "minimum_ceiling": unknown.minimum_ceiling,
                    "node_ranking": unknown.node_ranking,
                    "color_ranking": unknown.color_ranking,
                }
            case _:
                raise TypeError(f"Unknown type {type(unknown)}")

    return serialize
