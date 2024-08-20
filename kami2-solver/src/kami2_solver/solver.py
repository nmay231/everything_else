import json
import logging
import math
import pickle
from collections import Counter, defaultdict
from dataclasses import dataclass
from itertools import cycle, product, zip_longest
from typing import Any, Iterable, Mapping, cast

from PIL import Image, ImageDraw, ImageFilter, ImageFont
from PIL.Image import Image as ImageType
from PIL.ImageStat import Stat
from sklearn.cluster import AgglomerativeClustering

# F'in Pylance can't auto import but it keeps removing it from imports
dont_remove_from_imports = {Image, ImageFilter, ImageDraw, Stat, ImageType, ImageFont}

random_colors = cycle(["red", "blue", "green", "yellow"])


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


def get_mean_color(
    image: ImageType, point: tuple[float, float]
) -> tuple[int, int, int] | None:
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
    color: tuple[int, int, int]

    def __hash__(self):
        return hash(self.center)

    def with_color(self, color: tuple[int, int, int]) -> "Node":
        return Node(self.center, color)


class ColorGraph:
    def __init__(self, connections: Mapping[Node, Iterable[Node]]):
        self.connections: defaultdict[Node, set[Node]] = defaultdict(set)
        for node, neighbors in connections.items():
            self.connections[node] |= set(neighbors)
            for neighbor in neighbors:
                self.connections[neighbor].add(node)

    def combine_neighbors(
        self,
        color_labels: dict[tuple[int, int, int], int],
        average_color: dict[int, tuple[int, int, int]],
    ):
        """I guess after this point, nodes aren't represented by their centers
        anymore"""
        ungrouped = set(self.connections.keys())
        groups: list[set[Node]] = []
        while ungrouped:
            node = ungrouped.pop()
            group = {node}
            border_nodes = {node}
            while border_nodes:
                node = border_nodes.pop()
                for neighbor in self.connections[node]:
                    if neighbor in group:
                        continue
                    elif color_labels[node.color] != color_labels[neighbor.color]:
                        continue
                    group.add(neighbor)
                    border_nodes.add(neighbor)

            groups.append(group)

        frozen_groups = [frozenset(group) for group in groups]
        group_connections: dict[frozenset[Node], set[frozenset[Node]]] = defaultdict(
            set
        )
        for group in frozen_groups:
            neighbors: set[Node] = set()
            for node in group:
                for neighbor in self.connections[node]:
                    if neighbor in group:
                        continue
                    neighbors.add(neighbor)

            # TODO: Map nodes to groups instead of this
            group_connections[group] = {
                next(group for group in frozen_groups if neighbor in group)
                for neighbor in neighbors
            }

        self.connections = defaultdict(set)
        representatives = {group: next(iter(group)) for group in frozen_groups}
        for group, neighbors_ in group_connections.items():
            for neighbor in neighbors_:
                self.connections[representatives[group]].add(representatives[neighbor])
                self.connections[representatives[neighbor]].add(representatives[group])

        for node in self.connections:
            node.color = average_color[color_labels[node.color]]

    @staticmethod
    def _merge(nodes: set[Node], to_replace: set[Node], replacement: Node) -> set[Node]:
        if nodes & to_replace:
            return (nodes - to_replace) | {replacement}
        else:
            return nodes

    def recolor_node_and_merge(
        self, to_recolor: Node, color: tuple[int, int, int]
    ) -> "ColorGraph":
        recolored = to_recolor.with_color(color)
        to_merge = {
            node for node in self.connections[to_recolor] if node.color == color
        }
        to_merge.add(to_recolor)

        connections = defaultdict(
            set,
            (
                (node, self._merge(neighbors, to_merge, recolored))
                for node, neighbors in self.connections.items()
                if node not in to_merge
            ),
        )

        new_neighbors = set()
        for merging in to_merge:
            assert merging in self.connections
            new_neighbors |= self.connections[merging]
        connections[recolored] = new_neighbors - to_merge

        # The real fix is to make two factory functions, one that assumes the
        # connections are already bidirectional, and one that assumes they are
        # not. But I don't want to do that right now.
        result = ColorGraph.__new__(ColorGraph)
        result.connections = connections
        return result

    def distance_matrix(
        self, indexes_: dict[Node, int] | None = None
    ) -> tuple[dict[Node, int], list[list[int]]]:
        """d(u, v) for u, v in V(G) is the distance between the vertices u and
        v. This method returns a map from each node to an arbitrary index,
        and the distance between each vertex in the graph using those
        indexes.
        """
        indexes = indexes_ or {
            node: i for i, node in enumerate(self.connections.keys())
        }
        assert len(indexes) == len(
            self.connections
        ), "Indexes must be the same length as the graph"

        node_count = len(self.connections)
        distances: list[list[int]] = [
            [node_count] * node_count for _ in range(node_count)
        ]

        # TODO: I feel like there's a better way than O(n^2) here, but ah well
        for center in indexes.keys():
            frontier = {(center, 1)}
            seen: set[Node] = set()
            center_index = indexes[center]

            while frontier:
                (next_, distance) = frontier.pop()
                seen.add(next_)

                for neighbor in self.connections[next_]:
                    if neighbor in seen:
                        continue
                    frontier.add((neighbor, distance + 1))

                    neighbor_index = indexes[neighbor]
                    if distance < distances[center_index][neighbor_index]:
                        distances[center_index][neighbor_index] = distance
                        distances[neighbor_index][center_index] = distance

        for i in range(node_count):
            distances[i][i] = 0

        return indexes, distances

    def eccentricities(self) -> dict[Node, int]:
        """d(u, v) for u, v in V(G) is the distance between the vertices u and
        v. Eccentricity of a vertex v is the max distance d(v, u) for all
        vertex u in V(G). This method calculates the eccentricity of each
        node in the graph.
        """
        indexes, distances = self.distance_matrix()

        eccentricity: dict[Node, int] = {}
        for node, i in indexes.items():
            max_distance = max(distances[i])
            eccentricity[node] = max_distance

        return eccentricity


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


graph = ColorGraph(directed_connections)
used_colors = sorted(set(key.color for key in graph.connections.keys()))

# TODO: I guess I could require the user to specify the number of clusters, but
# I kinda like the idea of it being automatic
model = AgglomerativeClustering(distance_threshold=500, n_clusters=None)
model.fit(cast(Any, used_colors))
print(f"Discovered {model.n_clusters_} colors")

color_labels: dict[tuple[int, int, int], int] = dict(zip(used_colors, model.labels_))

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

average_color: dict[int, tuple[int, int, int]] = {}

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


@dataclass
class FloodFillSearch:
    graph: ColorGraph
    steps: list[Node]


def json_default_serialize(unknown: Any) -> Any:
    if isinstance(unknown, ColorGraph):
        nodes = [node for node in unknown.connections]
        node_index = {node: i for i, node in enumerate(nodes)}
        connections = {
            node_index[node]: [node_index[neighbor] for neighbor in neighbors]
            for node, neighbors in unknown.connections.items()
        }
        return {"nodes": nodes, "connections": connections}
    elif isinstance(unknown, Node):
        return {"center": unknown.center, "color": unknown.color}
    elif isinstance(unknown, FloodFillSearch):
        return {"graph": unknown.graph, "steps": unknown.steps}
    else:
        raise TypeError(f"Unknown type {type(unknown)}")


if __name__ == "__main__":
    logger = logging.getLogger(__name__)
    # Each line will be json so I can parse with jq
    logging.basicConfig(level=logging.INFO, filename="kami2.log", format="%(message)s")

    searches = [FloodFillSearch(graph, [])]
    logger.info(json.dumps(searches[0], default=json_default_serialize))
    step = 0

    # Breadth-first search
    # I know this puzzle can at least be solved in 5 iterations
    for iteration in range(5):
        print(f"Iteration {iteration} ==============")

        next_iteration: list[FloodFillSearch] = []
        for search in searches:
            for node in search.graph.connections.keys():
                colors = {neighbor.color for neighbor in search.graph.connections[node]}
                for color in colors:
                    new_graph = search.graph.recolor_node_and_merge(node, color)

                    assert len(new_graph.connections) < len(
                        search.graph.connections
                    ), "Flood fill should only remove nodes"
                    assert all(
                        n.color != neighbor.color
                        for n in new_graph.connections
                        for neighbor in new_graph.connections[n]
                    ), "nodes should be merged"
                    assert node not in new_graph.connections and all(
                        node not in conn for conn in new_graph.connections.values()
                    ), "node should be merged into a new node"

                    new_search = FloodFillSearch(new_graph, search.steps + [node])
                    logger.info(json.dumps(new_search, default=json_default_serialize))
                    next_iteration.append(new_search)

                    step += 1
                    if step % 1000 == 0:
                        print(f"Step {step}")
        searches = next_iteration

    with open("kami2.json", "w") as f:
        json.dump(searches, f, default=json_default_serialize)

    exit()

    largest_neighborhood = max(graph.connections.items(), key=lambda x: len(x[1]))
    center, neighbors = largest_neighborhood
    common_color = Counter(node.color for node in neighbors).most_common(1)[0][0]

    backup = edges.copy()
    draw = ImageDraw.Draw(edges)
    for a, bs in graph.connections.items():
        color = a.color
        radius = 30 if a == center else 8
        draw.circle(a.center, fill=color, radius=radius, outline="black", width=2)

        width = 10 if a == center else 2
        for b in bs:
            fill = "blue" if a == center and b.color == common_color else "black"
            draw.line(a.center + b.center, fill=fill, width=width)

    # color_labels = {color: label for label, color in average_color.items()}
    edges.show()

    graph = graph.recolor_node_and_merge(center, common_color)

    draw = ImageDraw.Draw(backup)

    for a, bs in graph.connections.items():
        color = a.color
        draw.circle(a.center, fill=color, radius=8, outline="black", width=2)

        for b in bs:
            draw.line(a.center + b.center, fill="black", width=5)
    backup.show()
