import math
import sys
import time
from collections import defaultdict
from itertools import cycle, product, zip_longest
from typing import Any, cast

from PIL import Image, ImageDraw, ImageFilter, ImageFont
from PIL.Image import Image as ImageType
from PIL.ImageStat import Stat
from sklearn.cluster import AgglomerativeClustering

from kami2_solver.graph import ColorGraph, Node
from kami2_solver.solver import SolverStep, solve
from kami2_solver.utils import ColorTup, get_mean_color, make_json_serializer

# F'in Pylance can't auto import but it keeps removing it from imports
dont_remove_from_imports = {Image, ImageFilter, ImageDraw, Stat, ImageType, ImageFont}

random_colors = cycle(["red", "blue", "green", "yellow"])


image = Image.open(sys.argv[1])

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


graph = ColorGraph(directed_connections)
used_colors = sorted(set(key.color for key in graph.connections.keys()))

# TODO: I guess I could require the user to specify the number of clusters, but
# I kinda like the idea of it being automatic
model = AgglomerativeClustering(distance_threshold=500, n_clusters=None)
model.fit(cast(Any, used_colors))
print(f"Discovered {model.n_clusters_} colors")

color_labels: dict[ColorTup, int] = dict(zip(used_colors, model.labels_))

# print(model.labels_)

height = math.ceil(len(used_colors) / 5)
palette = Image.new("RGB", (500, 100 * height), color=(0, 0, 0))
draw = ImageDraw.Draw(palette)

font = ImageFont.truetype("arial.ttf", 100)
for i, (color, label) in enumerate(color_labels.items()):
    x = i % 5
    y = i // 5
    palette.paste(color, (100 * x, 100 * y, 100 * (x + 1), 100 * (y + 1)))
    draw.text((100 * x + 10, 100 * y + 10), str(label), fill="black", font=font)

# palette.show()

# TODO: Turn into a list of length model.n_clusters_
average_color: dict[int, ColorTup] = {}

for i in range(model.n_clusters_):
    colors = [color for color in used_colors if color_labels[color] == i]
    r = b = g = 0
    for color in colors:
        r += color[0]
        g += color[1]
        b += color[2]

    average_color[i] = (
        int(r / len(colors)),
        int(g / len(colors)),
        int(b / len(colors)),
    )

inverted_colors = {
    label: (255 - r, 255 - g, 255 - b) for label, (r, g, b) in average_color.items()
}

graph.combine_neighbors(color_labels, average_color)
graph_size = len(graph.connections)
print(f"There are a total of {graph_size} nodes")

# logger = logging.getLogger(__name__)
# Each line will be json so I can parse with jq
filename = sys.argv[1].replace(".jpg", ".log")
# logging.basicConfig(level=logging.INFO, filename=filename, format="%(message)s")

color_map = {color: i for i, color in average_color.items()}
serialize = make_json_serializer(color_map)

color_ranking = list({node.color for node in graph.connections.keys()})
solve_process = solve(graph, color_ranking=color_ranking)

a_solution: SolverStep | None = None
start = time.time()
try:
    for step_n, step in enumerate(solve_process, 1):
        if step_n % 1000 == 0:
            print(
                f"{step.cache.minimum_ceiling=} node_pool_progress={step.cache.node_pool_size}/{graph_size} {step_n=:,}"
            )

        # TODO: This doesn't find all solutions since the algorithm actually
        # aborts *on* the N'th step if we've already found a solution with N
        # moves. This means we only get a few solutions.
        if step.found_a_solution:
            a_solution = step
            # logger.info(json.dumps(step, default=serialize))
except KeyboardInterrupt:
    print("\nCancelled run...")
    a_solution = None

duration = time.time() - start
print(f"Took {duration:.2f} seconds")

if a_solution is None:
    print("No solution found")
    exit(1)

print(
    f"Minimum number of moves: {a_solution.cache.minimum_ceiling} {a_solution.moves}, total iterations: {step_n}. {color_ranking=}"
)

moves = a_solution.moves

font = ImageFont.truetype("arial.ttf", 30)
draw = ImageDraw.Draw(edges)

previous = set()
for i, move in enumerate(moves, start=1):
    center = move.center
    color = move.color

    start_of_arc: tuple[float, float] | None = None
    while center in previous:
        start_of_arc = start_of_arc or center
        center = (center[0] + 50, center[1])
    previous.add(center)

    if start_of_arc is not None:
        height = (center[0] - start_of_arc[0]) / 3
        draw.arc(
            [
                start_of_arc[0],
                start_of_arc[1] - height,
                center[0],
                center[1] + height,
            ],
            start=0,
            end=180,
            fill="purple",
            width=10,
        )

    draw.circle(center, 30, fill=color, width=2)
    draw.text(
        center,
        str(i),
        fill="black",
        font=font,
        anchor="mm",
    )

edges.show()
