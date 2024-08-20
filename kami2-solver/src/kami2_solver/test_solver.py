import pytest

from kami2_solver.solver import ColorGraph, Node


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
    new_graph = graph.recolor_node_and_merge(a, new_a.color)

    assert len(new_graph.connections) == 3
    assert new_graph.connections == {new_a: {c, d}, c: {new_a, d}, d: {new_a, c}}

    new_b = b.with_color((0, 0, 0))
    new_graph = graph.recolor_node_and_merge(b, new_b.color)

    assert len(new_graph.connections) == 2
    assert new_graph.connections == {new_b: {c}, c: {new_b}}

    new_c = c.with_color((0, 0, 0))
    new_graph = graph.recolor_node_and_merge(c, new_c.color)

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
