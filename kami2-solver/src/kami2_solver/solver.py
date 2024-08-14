import math
from collections import defaultdict
from dataclasses import dataclass
from itertools import cycle, islice, product, zip_longest

from PIL import Image, ImageDraw, ImageFilter
from PIL.Image import Image as ImageType
from PIL.ImageStat import Stat

ColorTriple = tuple[int, int, int]

# F'in Pylance can't auto import but it keeps removing it from imports
dont_remove_from_imports = {Image, ImageFilter, ImageDraw, Stat, ImageType}


def get_mean_color(image: ImageType, point: tuple[float, float]) -> ColorTriple | None:
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


@dataclass
class Node:
    center: tuple[float, float]
    color: ColorTriple

    def __hash__(self):
        return hash(self.center)


class ColorGraph:
    def __init__(self, connections: dict[Node, list[Node]]):
        self.connections: defaultdict[Node, set[Node]] = defaultdict(set)
        for node, neighbors in connections.items():
            self.connections[node] |= set(neighbors)
            for neighbor in neighbors:
                self.connections[neighbor].add(node)


image = Image.open("kami2.jpg")

# Prepare for ALL the magic numbers
edges = image.crop((0, 146, image.width, image.height - 383))
draw = ImageDraw.Draw(edges)

LONG_RADIUS = 125
SHORT_RADIUS = LONG_RADIUS * (3**0.5) / 2

START = (0, 0)

columns: list[list[Node]] = []
directed_connections: defaultdict[Node, list[Node]] = defaultdict(list)

for x_index, tri_pointing_right in product(range(11), [True, False]):
    column: list[Node] = []
    for y_index in range(15):
        x = (
            START[0]
            + x_index * SHORT_RADIUS
            + (1 if tri_pointing_right else 2) * SHORT_RADIUS / 3
        )
        y = (
            START[1]
            + y_index * LONG_RADIUS
            + (0 if tri_pointing_right else LONG_RADIUS / 2)
        )
        if x_index & 1:
            if tri_pointing_right:
                y += LONG_RADIUS / 2
            else:
                y -= LONG_RADIUS / 2

        node_center = (x, y)
        color = get_mean_color(edges, node_center)
        if color is None:
            continue
        column.append(Node(node_center, color))
    columns.append(column)

    if len(columns) > 1:
        previous, current = columns[-2:]
        if tri_pointing_right:
            for left, right in zip(previous, current):
                directed_connections[right].append(left)
        else:
            if x_index & 1:
                previous = [None] + previous
            for left1, right, left2 in zip_longest(previous, current, previous[1:]):
                if right is None:
                    continue
                assert left1 or left2, "Only one neighbor can be None"

                if left1 is not None:
                    directed_connections[right].append(left1)
                if left2 is not None:
                    directed_connections[right].append(left2)

colors = cycle(["red", "blue", "green", "yellow"])

for a, bs in directed_connections.items():
    draw.circle(a.center, fill=a.color, radius=10, outline="black", width=2)

    bs = set(bs)

    for b in bs:
        draw.line(a.center + b.center, fill=next(colors), width=5)


graph = ColorGraph(directed_connections)
used_colors = sorted(set(key.color for key in graph.connections.keys()))

height = math.ceil(len(used_colors) / 5)
palette = Image.new("RGB", (500, 100 * height), color=(0, 0, 0))

for i, color in enumerate(used_colors):
    x = i % 5
    y = i // 5
    palette.paste(color, (100 * x, 100 * y, 100 * (x + 1), 100 * (y + 1)))
# palette.show()

# edges.show()

# Perform hierarchical clustering
distances = [[float("inf")] * len(used_colors) for _ in range(len(used_colors))]

for i, color in enumerate(used_colors):
    distances[i][i] = 0
    for j, other in islice(enumerate(used_colors), i + 1, None):
        distance = sum((c1 - c2) ** 2 for c1, c2 in zip(color, other))
        distances[i][j] = distance
        distances[j][i] = distance

lookup = dict(zip(used_colors, range(len(used_colors))))
clusters: list[list[ColorTriple]] = [used_colors[:]]

for _ in range(10):
    for i, color in enumerate(clusters):
        min_distance = min(distances[i])
        min_index = distances[i].index(min_distance)
        if min_distance > 0:
            clusters[i] = clusters[min_index]
            clusters[min_index] = color
            distances[i][min_index] = float("inf")
            distances[min_index][i] = float("inf")

representatives = defaultdict[ColorTriple, list[ColorTriple]](list)

for color in used_colors:
    for rep in representatives.keys():
        diff = sum((c1 - c2) ** 2 for c1, c2 in zip(color, rep))
        if diff < 4000:
            representatives[rep].append(color)
            break
    else:
        representatives[color] = [color]

print(len(representatives))
