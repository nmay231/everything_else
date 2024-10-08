from collections import defaultdict

import pytest

from kami2_solver.solver import ColorGraph, Node, solve


def test_flood_fill():
    #     a
    #    / \
    #   b   c
    #    \ /
    #     d
    a = Node((0, 0), (0, 0, 0))
    b = Node((1, 0), (0, 0, 1))
    c = Node((2, 0), (0, 0, 2))
    d = Node((3, 0), (0, 0, 0))
    graph = ColorGraph({a: {b, c}, d: {b, c}})
    assert len(graph.connections) == 4

    new_a = a.with_color((0, 0, 1))
    new_graph, _node = graph.recolor_node_and_merge(a, new_a.color)

    assert len(new_graph.connections) == 3
    assert new_graph.connections == {new_a: {c, d}, c: {new_a, d}, d: {new_a, c}}

    new_b = b.with_color((0, 0, 0))
    new_graph, _node = graph.recolor_node_and_merge(b, new_b.color)

    assert len(new_graph.connections) == 2
    assert new_graph.connections == {new_b: {c}, c: {new_b}}

    new_c = c.with_color((0, 0, 0))
    new_graph, _node = graph.recolor_node_and_merge(c, new_c.color)

    assert len(new_graph.connections) == 2
    assert new_graph.connections == {new_c: {b}, b: {new_c}}


def test_distance_matrix_cycle():
    #     a
    #    / \
    #   b   c
    #    \ /
    #     d
    a = Node((0, 0), (0, 0, 0))
    b = Node((1, 0), (0, 0, 0))
    c = Node((2, 0), (0, 0, 0))
    d = Node((3, 0), (0, 0, 0))
    graph = ColorGraph({a: {b, c}, d: {b, c}})

    indexes = {a: 0, b: 1, c: 2, d: 3}
    assert graph.distance_matrix(indexes) == (
        indexes,
        [[0, 1, 1, 2], [1, 0, 2, 1], [1, 2, 0, 1], [2, 1, 1, 0]],
    )


def test_distance_matrix_tee():
    #           d
    #           |
    # a -- b -- c
    #           |
    #           e
    a = Node((0, 0), (0, 0, 0))
    b = Node((1, 0), (0, 0, 0))
    c = Node((2, 0), (0, 0, 0))
    d = Node((3, 0), (0, 0, 0))
    e = Node((4, 0), (0, 0, 0))
    graph = ColorGraph({b: {a, c}, c: {b, d, e}})
    assert len(graph.connections) == 5

    indexes = {a: 0, b: 1, c: 2, d: 3, e: 4}
    assert graph.distance_matrix(indexes) == (
        indexes,
        [
            [0, 1, 2, 3, 3],
            [1, 0, 1, 2, 2],
            [2, 1, 0, 1, 1],
            [3, 2, 1, 0, 2],
            [3, 2, 1, 2, 0],
        ],
    )


@pytest.mark.parametrize("size", [1, 2, 3, 4, 5])
def test_distance_matrix_clique(size: int):
    # Every node is connected to every other node
    a = Node((0, 0), (0, 0, 0))
    b = Node((1, 0), (0, 0, 0))
    c = Node((2, 0), (0, 0, 0))
    d = Node((3, 0), (0, 0, 0))
    e = Node((4, 0), (0, 0, 0))

    selected = [a, b, c, d, e][:size]
    indexes = {node: i for i, node in enumerate(selected)}
    conns = {
        node: {neighbor for neighbor in selected if neighbor != node}
        for node in selected
    }
    graph = ColorGraph(conns)
    assert len(graph.connections) == size

    # Distance is 1 for every node since they're all connected (except for nodes
    # to themselves)
    expected = [[1] * size for _ in range(size)]
    for i in range(size):
        expected[i][i] = 0

    assert graph.distance_matrix(indexes) == (
        indexes,
        expected,
    )


def test_eccentricities_cycle():
    #     a
    #    / \
    #   b   c
    #    \ /
    #     d
    a = Node((0, 0), (0, 0, 0))
    b = Node((1, 0), (0, 0, 0))
    c = Node((2, 0), (0, 0, 0))
    d = Node((3, 0), (0, 0, 0))
    graph = ColorGraph({a: {b, c}, d: {b, c}})
    assert graph.eccentricities() == {a: 2, b: 2, c: 2, d: 2}


def test_eccentricities_tee():
    #           d
    #           |
    # a -- b -- c
    #           |
    #           e
    a = Node((0, 0), (0, 0, 0))
    b = Node((1, 0), (0, 0, 0))
    c = Node((2, 0), (0, 0, 0))
    d = Node((3, 0), (0, 0, 0))
    e = Node((4, 0), (0, 0, 0))
    graph = ColorGraph({b: {a, c}, c: {b, d, e}})
    assert len(graph.connections) == 5

    assert graph.eccentricities() == {a: 3, b: 2, c: 2, d: 3, e: 3}


@pytest.mark.parametrize("size", [2, 3, 4, 5])
def test_eccentricities_clique(size: int):
    # Every node is connected to every other node
    a = Node((0, 0), (0, 0, 0))
    b = Node((1, 0), (0, 0, 0))
    c = Node((2, 0), (0, 0, 0))
    d = Node((3, 0), (0, 0, 0))
    e = Node((4, 0), (0, 0, 0))

    selected = [a, b, c, d, e][:size]
    conns = {
        node: {neighbor for neighbor in selected if neighbor != node}
        for node in selected
    }
    graph = ColorGraph(conns)
    assert len(graph.connections) == size

    # Distance is 1 for every node since they're all connected (except for nodes
    # to themselves)
    expected = {node: 1 for node in selected}

    assert graph.eccentricities() == expected


def test_filtering_bad_starting_nodes():
    connections = defaultdict[Node, set[Node]](set)
    nodes = [Node((index, 0), (index, 0, 0)) for index in range(1, 6)]
    for a, b in zip(nodes, nodes[1:]):
        connections[a].add(b)
    graph = ColorGraph(connections)

    assert graph.bad_starting_nodes(allowed_moves=1) == set(nodes)
    assert graph.bad_starting_nodes(allowed_moves=2) == set(nodes[:2] + nodes[3:])
    assert graph.bad_starting_nodes(allowed_moves=3) == set(nodes[:1] + nodes[4:])


def test_solver_simple():
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
    assert step.state.minimum_ceiling == 3


@pytest.mark.parametrize("size", range(1, 6))
@pytest.mark.parametrize("checkerboard", [True, False])
@pytest.mark.parametrize("cyclic", [True, False])
def test_solver_palindrome(size: int, checkerboard: bool, cyclic: bool):
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
    assert step.state.minimum_ceiling == (size + 1 if cyclic else size)


@pytest.mark.parametrize("size", range(1, 6))
def test_solver_change_current_node(size: int):
    # a -- b -- a -- c -- a -- ...

    nodes = [Node((0, 1), (0, 0, 0))]
    for i in range(size):
        nodes.extend([Node((i, 2), (i, 0, 0)), Node((i, 1), (0, 0, 0))])
    connections = {a: [b] for a, b in zip(nodes, nodes[1:])}
    graph = ColorGraph(connections)

    for index, step in enumerate(solve(graph)):
        ...

    print(f"change current node {size=} {index=:,}")
    assert step.state.minimum_ceiling == size
