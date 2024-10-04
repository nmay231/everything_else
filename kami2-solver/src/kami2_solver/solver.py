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
class NewSearchInfo:
    graph: ColorGraph
    max_node_rank: int
    chosen_nodes: list[Node]
    focused_node: Node
    untried_colors: list[ColorTup]


def solve(graph: ColorGraph) -> Generator[SolverStep, None, None]:
    # colors = list(average_color.values())
    colors = list({node.color for node in graph.connections.keys()})
    cache = SolverCache(len(graph.connections), list(graph.connections.keys()), colors)

    for first_node in range(0, len(graph.connections)):
        # first_node acts as a ceiling that is slowly raised until every node
        # gets a chance to go first, while the inner function only decreases
        # max_node_rank.
        focused_node = cache.node_ranking[first_node]
        untried_colors = [*colors]
        search = NewSearchInfo(
            graph,
            max_node_rank=first_node,
            chosen_nodes=[],
            focused_node=focused_node,
            untried_colors=untried_colors,
        )
        yield from _solve(cache, search)


def _solve(
    cache: SolverCache,
    search: NewSearchInfo,
    previously_flooded: bool = False,
) -> Generator[SolverStep, None, None]:
    if len(search.chosen_nodes) == cache.minimum_ceiling:
        # If we don't care how many solutions there are, but just the minimum
        # number of FFs.
        return
    # We yield whenever we FF a node. This allows for a progress bar of sorts.
    # We also need an early return if we are already at the minimum ceiling.

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

        next_search = NewSearchInfo(
            graph=next_graph,
            max_node_rank=search.max_node_rank,
            chosen_nodes=search.chosen_nodes + [merged_node],
            focused_node=merged_node,
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
        for max_node_rank in range(search.max_node_rank - 1, -1, -1):
            search.max_node_rank = max_node_rank
            yield from _solve(cache, search)
