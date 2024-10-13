from collections import defaultdict

import pytest

from kami2_solver.graph import ColorGraph, Node
from kami2_solver.solver import solve


def test_simple_graph():
    """Just a random graph I hand checked"""
    # r1 ----- g2
    #  \       |
    #   \      |
    #    \     |
    #     g3   |
    #    / \   |
    #   /   \  |
    #  /     \ |
    # b4 ---- r5
    #  \      /
    #   \    /
    #    \  /
    #     g6
    r1 = Node((1, 0), (1, 0, 0))
    g2 = Node((2, 0), (0, 1, 0))
    g3 = Node((3, 0), (0, 1, 0))
    b4 = Node((4, 0), (0, 0, 1))
    r5 = Node((5, 0), (1, 0, 0))
    g6 = Node((6, 0), (0, 1, 0))
    graph = ColorGraph(
        {
            r1: [g2, g3],
            b4: [g3, r5, g6],
            r5: [g2, g3, b4, g6],
        }
    )

    # Just a sanity check
    assert len(graph.connections) == 6
    assert graph.n_edges() == 8

    assert list(graph.connections.keys()) == [r1, g2, g3, b4, r5, g6]

    for index, step in enumerate(solve(graph)):
        ...

    print(f"Simple example took {index} steps")
    assert step.cache.minimum_ceiling == 3


@pytest.mark.parametrize("size", range(1, 6))
@pytest.mark.parametrize("checkerboard", [True, False])
@pytest.mark.parametrize("cyclic", [True, False])
def test_palindrome(size: int, checkerboard: bool, cyclic: bool):
    """A litany of graphs with predictable solutions"""
    # If checkerboard is True:
    # a -- b -- a -- b -- a
    # If checkerboard is False:
    # c -- b -- a -- b -- c
    # If cyclic is True:
    # c -- b -- a -- b -- c
    #  \                 /
    #   --------d--------

    # Either way, the minimum number of moves is equal to size
    middle = Node((0, 0), (0, 0, 0))
    prev = (middle, middle)
    connections = defaultdict[Node, set[Node]](set)
    for index in range(1, size + 1):
        color = (index % 2, 0, 0) if checkerboard else (index, 0, 0)

        left = Node((index, 1), color)
        right = Node((index, 2), color)
        connections[left].add(prev[0])
        connections[right].add(prev[1])
        prev = (left, right)

    if cyclic:
        index = size + 1
        color = (index % 2, 0, 0) if checkerboard else (index, 0, 0)
        other_middle = Node((index, 1), color)
        connections[other_middle].update(prev)

    graph = ColorGraph(connections)

    # Sanity check
    assert len(graph.connections) == size * 2 + 2 if cyclic else size * 2 + 1
    assert graph.n_edges() == size * 2 + 2 if cyclic else size * 2
    assert len(set(node.color for node in graph.connections.keys())) == (
        2 if checkerboard else (size + 2 if cyclic else size + 1)
    )

    for index, step in enumerate(solve(graph)):
        ...

    print(f"palindrome {size=} {index=:,} {checkerboard=} {cyclic=}")
    assert step.cache.minimum_ceiling == (size + 1 if cyclic else size)


@pytest.mark.parametrize("size", range(1, 6))
def test_will_change_current_node(size: int):
    """Some graphs require changing the focused node to find the optimal solution"""
    # a -- b -- a -- c -- a -- ...

    nodes = [Node((0, 1), (0, 0, 0))]
    for i in range(size):
        nodes.extend([Node((i, 2), (i, 0, 0)), Node((i, 1), (0, 0, 0))])
    connections = {a: [b] for a, b in zip(nodes, nodes[1:])}
    graph = ColorGraph(connections)

    for index, step in enumerate(solve(graph)):
        ...

    print(f"change current node {size=} {index=:,}")
    assert step.cache.minimum_ceiling == size
