from dataclasses import dataclass
from typing import Collection, Generator

from .graph import ColorGraph, Node
from .utils import ColorTup


@dataclass
class SolverStep:
    current_graph: ColorGraph
    steps: list[Node]
    state: "SolverCache"
    found_a_solution: bool


@dataclass
class SolverCache:
    """A mix of cache and state that affects a solve globally"""

    minimum_ceiling: int
    node_ranking: list[Node]
    color_ranking: list[ColorTup]

    def reorder_and_dedup_colors(self, colors: Collection[ColorTup]) -> list[ColorTup]:
        return [color for color in self.color_ranking if color in colors]


@dataclass
class SearchInfo:
    graph: ColorGraph
    chosen_nodes: list[Node]
    focused_node: Node
    other_allowed_nodes: list[Node]
    untried_colors: list[ColorTup]


def solve(graph: ColorGraph) -> Generator[SolverStep, None, None]:
    colors = list({node.color for node in graph.connections.keys()})
    cache = SolverCache(len(graph.connections), list(graph.connections.keys()), colors)

    for length in range(len(graph.connections) - 1):
        allowed_nodes = cache.node_ranking[:length]
        focused_node = cache.node_ranking[length]
        neighbor_colors = {node.color for node in graph.connections[focused_node]}
        untried_colors = cache.reorder_and_dedup_colors(neighbor_colors)

        search = SearchInfo(
            graph,
            chosen_nodes=[],
            focused_node=focused_node,
            other_allowed_nodes=allowed_nodes,
            untried_colors=untried_colors,
        )
        yield from _solve(cache, search)


def _solve(
    cache: SolverCache,
    search: SearchInfo,
    previously_flooded: bool = False,
) -> Generator[SolverStep, None, None]:
    if len(search.chosen_nodes) == cache.minimum_ceiling:
        # If we don't care how many solutions there are, but just the minimum
        # number of FFs.
        return

    # We start by exhausting colors for the current node continuing until we
    # either solve the graph or hit the minimum ceiling. If we solve the graph,
    # then we update the minimum ceiling.
    for i, color in enumerate(search.untried_colors):
        # To avoid doing both (green then red) and (red then green), we only
        # allow colors later in the list and colors of newly exposed nodes
        next_colors = search.untried_colors[i + 1 :]

        for neigh in search.graph.connections[search.focused_node]:
            if neigh.color != color:
                continue

            # Now watch me whip...
            for neigh_neigh in search.graph.connections[neigh]:
                if neigh_neigh == search.focused_node:
                    continue
                next_colors.append(neigh_neigh.color)

        next_colors = cache.reorder_and_dedup_colors(next_colors)

        next_graph, merged_node = search.graph.recolor_node_and_merge(
            search.focused_node, color
        )

        next_search = SearchInfo(
            graph=next_graph,
            chosen_nodes=search.chosen_nodes + [merged_node],
            focused_node=merged_node,
            other_allowed_nodes=search.other_allowed_nodes,
            untried_colors=next_colors,
        )

        found_solution = 1 == len(next_graph.connections)
        step = SolverStep(next_graph, next_search.chosen_nodes, cache, found_solution)

        if found_solution:
            cache.minimum_ceiling = len(next_search.chosen_nodes)
            yield step
            return
        else:
            yield step

        yield from _solve(cache, next_search, True)

    if previously_flooded:
        allowed_moves = cache.minimum_ceiling - len(search.chosen_nodes)
        bad_starts = search.graph.bad_starting_nodes(allowed_moves)
        starting_nodes = [
            node
            for node in search.other_allowed_nodes
            if node not in bad_starts and node in search.graph.connections
        ]

        for focused_node in reversed(starting_nodes):
            # Probably not good to modify while iterating, but whatever
            starting_nodes.pop()

            search.focused_node = focused_node
            search.other_allowed_nodes = starting_nodes
            search.untried_colors = cache.reorder_and_dedup_colors(
                {node.color for node in search.graph.connections[focused_node]}
            )
            yield from _solve(cache, search)
