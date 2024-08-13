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


class ColorGraph:
    def __init__(self, connections: dict[Node, list[Node]]):
        self.connections = connections


right_left = lambda: cycle([True, False])


image = Image.open("kami2.jpg")

# edges = image.filter(ImageFilter.FIND_EDGES())
# image.show()
# image.crop((0, 85, image.width, image.height)).show()
# edges = image
edges = image.crop((0, 146, image.width, image.height - 383))
# edges.show()
draw = ImageDraw.Draw(edges)
# draw.line((0, 146, edges.width, 146), fill="red", width=5)
# edges.show()

LONG_RADIUS = 125
SHORT_RADIUS = LONG_RADIUS * (3**0.5) / 2

# START = (SHORT_RADIUS / 3, LONG_RADIUS / 2)
START = (0, 0)

columns: list[list[Node]] = []

# for is_right in right_left():
for is_right, x_index in product([True, False], range(11)):
    column: list[Node] = []
    for y_index in range(15):
        x = (
            START[0]
            + x_index * SHORT_RADIUS
            + (1 if is_right else 2) * SHORT_RADIUS / 3
        )
        y = START[1] + y_index * LONG_RADIUS + (0 if is_right else LONG_RADIUS / 2)
        if x_index & 1:
            y += LONG_RADIUS / 2

        fill = "purple"
        match (x_index & 1, y_index & 1):
            case (0, 0):
                fill = "red"
            case (0, 1):
                fill = "blue"
            case (1, 0):
                fill = "green"
            case (1, 1):
                fill = "yellow"
        # draw.circle((x, y), 10, fill=fill)

        column.append(Node((x, y), None))
    columns.append(column)

edges.show()
# exit()

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
