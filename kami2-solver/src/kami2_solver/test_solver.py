import copy
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


def test_2d_checkerboards_ignore_duplicate_searches():
    """Given a graph with a-b-a, there is no point of FF both a's to the color b
    since you can just FF the b to a then back to b. You reach more nodes that
    way and, more importantly, do not miss any.

    This graph has a checkerboard with plenty of aba patterns. The nodes c-f are
    necessary to prevent optimizations from eccentricity alone."""

    # I should honestly consider making a parser for this kind of thing...
    #    0    1    2    3    4
    # 0            a -- c
    #              |
    # 1  d    a -- b -- a    e
    #    |    |    |    |    |
    # 2  a -- b -- a -- b -- a
    #         |    |    |
    # 3       a -- b -- a
    #              |
    # 4            a -- f

    checkerboard: list[list[None | Node]] = [[None for _ in range(5)] for _ in range(5)]
    for row in range(5):
        for col in range(5):
            if 2 <= row + col <= 6 and abs(row - col) <= 2:
                color = ((row + col) % 2, 0, 0)
                checkerboard[row][col] = Node((row, col), color)

    connections = defaultdict[Node, set[Node]](set)
    for row in range(5):
        for col in range(5):
            node = checkerboard[row][col]
            if node is None:
                continue

            for dx, dy in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                if not (0 <= row + dy < 5 and 0 <= col + dx < 5):
                    continue
                neighbor = checkerboard[row + dy][col + dx]
                if neighbor is None:
                    continue
                connections[node].add(neighbor)

    u, r, d, l = (
        checkerboard[0][2],
        checkerboard[2][4],
        checkerboard[4][2],
        checkerboard[2][0],
    )

    assert u and r and d and l, "Assumed the corners of the diamonds are present"

    connections[Node((0, 0), (0, 0, 1))].add(u)
    connections[Node((0, 0), (0, 0, 2))].add(r)
    connections[Node((0, 0), (0, 0, 3))].add(d)
    connections[Node((0, 0), (0, 0, 4))].add(l)
    graph = ColorGraph(connections)

    # Sanity check
    assert len(graph.connections) == 17
    assert graph.n_edges() == 20

    for index, step in enumerate(solve(graph)):
        ...

    assert step.cache.minimum_ceiling == 6
    assert index <= 4670


def test_2d_checkerboards_ignore_duplicate_searches_change_current_node():
    """This test uses practically the same graph as the last one except that `-f`
    from the previous graph gets replaced with `-b-a`. This still takes 6 moves,
    but requires changing the focused node."""

    # I should honestly consider making a parser for this kind of thing...
    #    0    1    2    3    4
    # 0            a -- c
    #              |
    # 1  d    a -- b -- a    e
    #    |    |    |    |    |
    # 2  a -- b -- a -- b -- a
    #         |    |    |
    # 3       a -- b -- a
    #              |
    # 4            a -- b -- a

    checkerboard: list[list[None | Node]] = [[None for _ in range(5)] for _ in range(5)]
    for row in range(5):
        for col in range(5):
            if 2 <= row + col <= 6 and abs(row - col) <= 2:
                color = ((row + col) % 2, 0, 0)
                checkerboard[row][col] = Node((row, col), color)

    connections = defaultdict[Node, set[Node]](set)
    for row in range(5):
        for col in range(5):
            node = checkerboard[row][col]
            if node is None:
                continue

            for dx, dy in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                if not (0 <= row + dy < 5 and 0 <= col + dx < 5):
                    continue
                neighbor = checkerboard[row + dy][col + dx]
                if neighbor is None:
                    continue
                connections[node].add(neighbor)

    u, r, d, l = (
        checkerboard[0][2],
        checkerboard[2][4],
        checkerboard[4][2],
        checkerboard[2][0],
    )

    assert u and r and d and l, "Assumed the corners of the diamonds are present"

    connections[Node((0, 0), (0, 0, 1))].add(u)
    connections[Node((0, 0), (0, 0, 2))].add(r)
    connections[Node((0, 0), (0, 0, 4))].add(l)

    connections[Node((0, 0), (1 - d.color[0], 0, 0))].update([d, copy.copy(d)])
    graph = ColorGraph(connections)

    # Sanity check
    assert len(graph.connections) == 17
    assert graph.n_edges() == 20

    for index, step in enumerate(solve(graph)):
        ...

    assert step.cache.minimum_ceiling == 6
    assert index <= 3334
