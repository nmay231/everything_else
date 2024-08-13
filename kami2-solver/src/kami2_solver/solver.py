from collections import defaultdict
from dataclasses import dataclass
from itertools import cycle, product
from typing import Literal

from PIL import Image, ImageDraw, ImageFilter

Color = Literal["red", "blue", "green", "yellow"]

# F'in Pylance can't auto import but it keeps removing it from imports
dont_remove_from_imports = [Image, ImageFilter, ImageDraw]


@dataclass
class Node:
    center: tuple[float, float]
    color: Color | None

    def __hash__(self):
        return hash(self.center)


class ColorGraph:
    def __init__(self, connections: dict[Node, list[Node]]):
        self.connections: defaultdict[Node, set[Node]] = defaultdict(set)
        for node, neighbors in connections.items():
            self.connections[node] |= set(neighbors)
            for neighbor in neighbors:
                self.connections[neighbor].add(node)


# right_left = lambda: cycle([True, False])


image = Image.open("kami2.jpg")

edges = image.crop((0, 146, image.width, image.height - 383))
draw = ImageDraw.Draw(edges)

# Prepare for ALL the magic numbers
LONG_RADIUS = 125
SHORT_RADIUS = LONG_RADIUS * (3**0.5) / 2

START = (0, 0)

columns: list[list[Node]] = []
previous: list[Node] = []
current: list[Node] = []
directed_connections: defaultdict[Node, list[Node]] = defaultdict(list)

for x_index, tri_pointing_left in product(range(11), [True, False]):
    column: list[Node] = []
    for y_index in range(15):
        x = (
            START[0]
            + x_index * SHORT_RADIUS
            + (1 if tri_pointing_left else 2) * SHORT_RADIUS / 3
        )
        y = (
            START[1]
            + y_index * LONG_RADIUS
            + (0 if tri_pointing_left else LONG_RADIUS / 2)
        )
        if x_index & 1:
            y += LONG_RADIUS / 2

        fill = "purple"
        # match (x_index & 1, y_index & 1):
        #     case (0, 0):
        #         fill = "red"
        #     case (0, 1):
        #         fill = "blue"
        #     case (1, 0):
        #         fill = "green"
        #     case (1, 1):
        #         fill = "yellow"
        match (tri_pointing_left, x_index & 1):
            case (True, 0):
                fill = "red"
            case (True, 1):
                fill = "blue"
            case (False, 0):
                fill = "green"
            case (False, 1):
                fill = "yellow"
        # draw.circle((x, y), 10, fill=fill)

        column.append(Node((x, y), None))
    columns.append(column)

    previous, current = current, column
    if len(columns) > 1:
        if tri_pointing_left:
            for left, right in zip(previous, current):
                directed_connections[right].append(left)
        else:
            for left1, right, left2 in zip(previous, current, previous[1:]):
                directed_connections[right].append(left1)
                directed_connections[right].append(left2)

# seen = Counter()
# for coli, column in islice(enumerate(columns), 5):
#     fill = ["red", "blue", "green", "yellow"][coli % 4]
#     seen[fill] += 1
#     for node in column:
#         draw.circle(node.center, 10, fill=fill)


# print(seen)

# edges.show()
# exit()

colors = cycle(["red", "blue", "green", "yellow"])

for a, bs in directed_connections.items():
    draw.circle(a.center, fill="pink", radius=10)

    bs = set(bs)

    for b in bs:
        draw.line(a.center + b.center, fill=next(colors), width=5)
# graph = ColorGraph(directed_connections)
# for a, bs in graph.connections.items():
#     print(a, bs)

edges.show()

for y_index, (c1, c2) in enumerate(zip(columns, columns[1:], strict=False)):
    if y_index & 1:
        c2 = c2[1:]

    for x_index, (node1, node2) in enumerate(zip(c1, c2)):
        x1, y1 = node1.center
        x2, y2 = node2.center

        # print(edges[x1, y1])
        # edges.filter(ImageFilter)
        RAD = 5
        # edges.res
        square = edges.crop(
            (
                max(0, x1 - RAD),
                max(0, y1 - RAD),
                min(image.width, x1 + RAD),
                min(image.height, y1 + RAD),
            )
        )
        colors = square.getcolors()
        assert colors, "Too many colors"
        print(len(colors), colors)
        square.show()
        # edges.getcolors()
        break
    break

    # fill = "purple"
    # match (x_index & 1, y_index & 1):
    #     case (0, 0):
    #         fill = "red"
    #     case (0, 1):
    #         fill = "blue"
    #     case (1, 0):
    #         fill = "green"
    #     case (1, 1):
    #         fill = "yellow"
    # # draw.circle((x, y), 10, fill=fill)

    # draw.line((x1, y1, x2, y2), fill=fill, width=5)

# edges.show()
# image.show("testing")

# image.filter(ImageFilter.GaussianBlur(50)).show()
# image.filter(ImageFilter.FIND_EDGES()).show("FIND_EDGES")
# image.filter(ImageFilter.EDGE_ENHANCE()).show("EDGE_ENHANCE")
# image.filter(ImageFilter.EDGE_ENHANCE_MORE()).show("EDGE_ENHANCE_MORE")
# image.filter(ImageFilter.EDGE_ENHANCE()).filter(ImageFilter.FIND_EDGES()).show(
#     "EDGE_ENHANCE 2"
# )
# image.filter(ImageFilter.EDGE_ENHANCE_MORE()).filter(ImageFilter.FIND_EDGES()).show(
#     "EDGE_ENHANCE_MORE 2"
# )
