from collections import defaultdict

from utils import Edge, Point


def is_solved(grid_size: int, endpoints: list[Edge], edges: list[Edge]) -> bool:
    """
    width, height: size of the board of course
    n_paths: number of paths and endpoint pairs provided in endpoints
    endpoints: Pairs of endpoints (most likely not adjacent as the type Edge might suggest)
    edges: solution of Edge's that is being checked
    """
    if grid_size != len(endpoints) + len(edges):
        # There are not enough edges to cover the entire grid.
        return False

    graph = defaultdict[Point, set[Point]](set)
    for a, b in edges:
        graph[a].add(b)
        graph[b].add(a)

    for start, end in endpoints:
        while start != end:
            if len(graph[start]) == 1:
                old = start
                start = graph[start].pop()
                del graph[old]
                graph[start].difference_update([old])
            else:
                print("Invalid solution. More or less than the required connections")
                return False

        del graph[start]

    # The graph must be empty to be a valid solution
    assert not graph, "Logical error, the graph must be empty at the end of the solution"
    return True


# start = [
#     [0, 0, 0, 4, 0, 0, 0],
#     [0, 3, 0, 0, 2, 5, 0],
#     [0, 0, 0, 3, 1, 0, 0],
#     [0, 0, 0, 5, 0, 0, 0],
#     [0, 0, 0, 0, 0, 0, 0],
#     [0, 0, 1, 0, 0, 0, 0],
#     [2, 0, 0, 0, 4, 0, 0],
# ]
# solution = [
#     [2, 2, 2, 4, 4, 4, 4],
#     [2, 3, 2, 2, 2, 5, 4],
#     [2, 3, 3, 3, 1, 5, 4],
#     [2, 5, 5, 5, 1, 5, 4],
#     [2, 5, 1, 1, 1, 5, 4],
#     [2, 5, 1, 5, 5, 5, 4],
#     [2, 5, 5, 5, 4, 4, 4],
# ]

start = [
    [0, 0, 0, 1, 2, 0, 0],
    [0, 0, 0, 0, 3, 4, 0],
    [0, 0, 0, 0, 0, 0, 2],
    [0, 0, 0, 5, 0, 0, 6],
    [0, 0, 0, 0, 0, 0, 0],
    [0, 3, 6, 0, 0, 4, 0],
    [0, 1, 5, 0, 0, 0, 0],
]
solution = [
    [1, 1, 1, 1, 2, 2, 2],
    [1, 3, 3, 3, 3, 4, 2],
    [1, 3, 6, 6, 6, 4, 2],
    [1, 3, 6, 5, 6, 4, 6],
    [1, 3, 6, 5, 6, 4, 6],
    [1, 3, 6, 5, 6, 4, 6],
    [1, 1, 5, 5, 6, 6, 6],
]

start = [list(reversed(row)) for row in start]
solution = [list(reversed(row)) for row in solution]

endpoints_ = defaultdict(list)
for y, row in enumerate(start):
    for x, val in enumerate(row):
        if val:
            endpoints_[val].append((x, y))

endpoints = list[Edge]()
for i in range(1, 7):
    assert len(endpoints_[i]) == 2, "More or less than two endpoints per number found"
    endpoints.append(tuple(endpoints_[i]))

edges = list[Edge]()
for y, row in enumerate(solution):
    for x, val in enumerate(row):
        if y + 1 < len(solution) and val == solution[y + 1][x]:
            edges.append(((x, y), (x, y + 1)))

        if x + 1 < len(solution) and val == solution[y][x + 1]:
            edges.append(((x, y), (x + 1, y)))

print(is_solved(7 * 7, endpoints, edges))
