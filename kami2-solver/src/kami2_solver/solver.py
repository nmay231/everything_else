import copy
from dataclasses import dataclass
from typing import Collection, Generator

from .graph import ColorGraph, Node
from .utils import ColorTup


@dataclass
class SolverStep:
    """An object containing debugging information about the current search in
    progress."""

    current_graph: ColorGraph
    moves: list[Node]
    """The resulting nodes *after* each FF of the graph"""
    cache: "SolverCache"
    found_a_solution: bool
    """Is the graph a solution using the (currently known) minimum number of
    moves"""


@dataclass
class SolverCache:
    """A mix of cache and state that affects a solve globally"""

    minimum_ceiling: int
    """A ceiling on the minimum number of moves to get a solution"""
    node_ranking: list[Node]
    """An arbitrary ordering of nodes (to avoid duplicate work)"""
    color_ranking: list[ColorTup]
    """An arbitrary ordering of colors (to avoid duplicate work)"""
    node_pool_size: int
    """**Purely for debugging purposes** The current size of the node pool. Starts at 1 and increases up to the
    total number of nodes."""

    def reorder_and_dedup_colors(self, colors: Collection[ColorTup]) -> list[ColorTup]:
        return [color for color in self.color_ranking if color in colors]


@dataclass
class SearchInfo:
    """Contains information about a graph after a certain number of FF's have
    happened including the information needed to choose the next nodes to FF."""

    graph: ColorGraph
    """The current state of the graph"""
    chosen_nodes: list[Node]
    """The nodes that have been FF in the past during this search"""
    focused_node: Node
    """The node that will be FF the next solver iteration"""
    other_allowed_nodes: list[Node]
    """A set of nodes that can become the next focused node. This starts out
    empty to seed the minimum_ceiling calculation, and slowly expands into the
    whole set of nodes. This is in the same order as node_ranking and sometimes
    has nodes removed when it's clear those nodes cannot be FF in this search."""
    untried_colors: list[ColorTup]
    """A subset of neighboring colors. This is used to avoid duplicate work"""
    parent: "SearchInfo | None"
    """The previous search that produced this one. At the time of writing,
    mostly used to see the previous graph before a FF."""


def solve(
    graph: ColorGraph,
    *,
    color_ranking: list[ColorTup] | None = None,
) -> Generator[SolverStep, None, None]:
    colors = list({node.color for node in graph.connections.keys()})
    if color_ranking is not None:
        assert set(color_ranking) == set(colors)
        colors = color_ranking

    cache = SolverCache(
        len(graph.connections),
        list(graph.connections.keys()),
        colors,
        1,
    )

    for length in range(len(graph.connections) - 1):
        allowed_nodes = cache.node_ranking[:length]
        focused_node = cache.node_ranking[length]
        neighbor_colors = {node.color for node in graph.connections[focused_node]}
        untried_colors = cache.reorder_and_dedup_colors(neighbor_colors)
        cache.node_pool_size = length + 1

        search = SearchInfo(
            graph,
            chosen_nodes=[],
            focused_node=focused_node,
            other_allowed_nodes=allowed_nodes,
            untried_colors=untried_colors,
            parent=None,
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
            parent=search,
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

        previous_focused_node = search.focused_node

        while starting_nodes:
            next_node = starting_nodes.pop()

            search = copy.copy(search)
            search.focused_node = next_node
            search.other_allowed_nodes = starting_nodes
            search.untried_colors = cache.reorder_and_dedup_colors(
                {node.color for node in search.graph.connections[next_node]}
            )

            if search.parent:
                original_color = search.parent.focused_node.color
                # In the (sub)graph [a1-b-a2], we check for the condition that
                # we FFed a2->b. If so, we would now be in the graph [a1-b']. To
                # reduce duplicate work, we disallow FFing b'->a since it is
                # always better to have FFed b->a in the first place.
                if (
                    original_color == next_node.color
                    and next_node in search.graph.connections[previous_focused_node]
                ):
                    # Should never raise ValueError because next_node is a neighbor of previous_focused_node
                    search.untried_colors.remove(previous_focused_node.color)

            yield from _solve(cache, search)
