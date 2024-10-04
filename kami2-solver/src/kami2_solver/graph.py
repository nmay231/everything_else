from collections import defaultdict
from dataclasses import dataclass
from typing import Iterable, Mapping

from .utils import ColorTup


@dataclass
class Node:
    center: tuple[float, float]
    color: ColorTup

    def __hash__(self):
        return hash(self.center)

    def with_color(self, color: ColorTup) -> "Node":
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
        color_labels: dict[ColorTup, int],
        average_color: dict[int, ColorTup],
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
        self, to_recolor: Node, color: ColorTup
    ) -> tuple["ColorGraph", Node]:
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
        return (result, recolored)

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

    def exclude_impossible_starts(self, moves_left: int) -> Iterable[Node]:
        eccentricities = self.eccentricities()
        if max(eccentricities.values()) > 2 * moves_left:
            # A move can only reduce eccentricity by at most 2, so any move set
            # would leave more than 2 nodes after the flood fill.
            return

        # TODO: I thought there was a way to filter out high eccentricity nodes
        # here, but that's only valid if we know that focusing on that node will
        # leave another one with connections. Maybe there's something about how
        # many highly eccentric nodes that matters? Ugh...
        yield from eccentricities.keys()

    def n_edges(self) -> int:
        double_count = sum(len(neigh) for neigh in self.connections.values())
        assert not (double_count & 1), "Double counted edges should be even"
        return double_count // 2
