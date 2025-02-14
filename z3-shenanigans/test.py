# %%
import z3
from z3 import *

# %%

Pair = Datatype("Pair")
Pair.declare("init", ("first", IntSort()), ("second", IntSort()))
Pair.declare("nil")
Pair = Pair.create()
first, second = Pair.first, Pair.second

# %%
p1 = Pair.init(0, 0)
p2 = Pair.nil
p3 = Pair.init(-10, 15)

solver = Solver()
solver.add(first(p1) + second(p1) == first(p2) + second(p2))
solver.add(first(p2) - second(p2) == first(p3) - second(p3))

solver.check()

# %%

x, y = Reals("x y")
g = Goal()
g.add(x > 0, y > 0, x == y + 2)
print(g)

t = Then(Tactic("simplify"), Tactic("solve-eqs"))
print(t)
print(t(g))

# %%
# describe_tactics()
describe_probes()

# %%
t = Then("simplify", "normalize-bounds", "solve-eqs")

x, y, z = Ints("x y z")
g = Goal()
g.add(x > 10, y == x + 3, z > y)

r = t(g)
# r contains only one subgoal
print(r)
s = Solver()
s.add(r[0])
print(s.check())
# Model for the subgoal
print(s.model())
# Model for the original goal
print(r[0].convert_model(s.model()))

# %%
# 4x4 sudoku
# I pretended to generalize the size of the grid, but it's still pretty much hard-coded to 4x4
width = 4
board = [[Int(f"b_{j}_{i}") for i in range(width)] for j in range(width)]

exists = [
    And(1 <= board[j][i], board[j][i] <= 4) for i in range(width) for j in range(width)
]
distinct_rows = [Distinct(row) for row in board]
distinct_columns = [Distinct([row[col] for row in board]) for col in range(width)]

hori, vert = int(width**0.5), int(width**0.5)  # 2, 2
distinct_boxes = [
    Distinct([board[y][x] for x in range(x0, x0 + hori) for y in range(y0, y0 + vert)])
    for x0 in range(0, width, hori)
    for y0 in range(0, width, vert)
]

# start = [[1, 2, 3, 4],
#          [4, 3, 2, 1],
#          [3, 4, 1, 2],
#          [2, 1, 4, 3]]
start = [[1, 0, 0, 4], [0, 3, 2, 0], [3, 4, 0, 0], [0, 0, 0, 0]]
input_ = [
    board[j][i] == start[j][i] if start[j][i] else True
    for i in range(width)
    for j in range(width)
]

s = Solver()
s.add(*exists, *distinct_rows, *distinct_columns, *distinct_boxes, *input_)
print(s.check())
model = s.model()
print([[model.eval(board[j][i]) for i in range(width)] for j in range(width)])

# %%
# Numberlink
width = height = 7

s = Solver()
start = [
    [0, 0, 0, 4, 0, 0, 0],
    [0, 3, 0, 0, 2, 5, 0],
    [0, 0, 0, 3, 1, 0, 0],
    [0, 0, 0, 5, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0],
    [2, 0, 0, 0, 4, 0, 0],
]

# Focus on paths:

# x_v_i_p. v is the vertex in the grid lattice. i is the index the vertex takes of the path p.
s.add(*(Distinct() for _ in range(0)))

# I could try adding constraints in many different ways all at once and see what happens... It is a difficult problem in and of itself to pick which way/method would be best anyways, so why not try them all at the same time?

# So, I am mainly messing around with z3 to see what style of programming I should go with. That means I should be collecting working examples of different algorithms I devise and things like that.
#  I will say that I suspect I will not be able to use the same language structure for checking and solving, and therefore it would be better to have none of them shared.

# %%

# Next puzzle to solve with SAT is the pipes or netlink puzzle since everything is a local rule anyways.
