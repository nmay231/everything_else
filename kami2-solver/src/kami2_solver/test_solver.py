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
