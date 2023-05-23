from operator import add, mul, sub, truediv
from itertools import permutations, combinations_with_replacement

# I was struggling with this puzzle, so I decided it would make a good programming puzzle

all_ops = (add, mul, sub, truediv)
op_to_str = {add: "+", mul: "*", sub: "-", truediv: "/"}
all_nums = (3, 3, 7, 1)


if __name__ == "__main__":
    # Here's my 'efficient' solution
    for a, b, c, d in permutations(all_nums):
        # print(a, b, c, d)
        for ops in combinations_with_replacement(all_ops, 3):
            for x, y, z in permutations(ops):
                try:
                    if z(y(x(a, b), c), d) == 10:
                        # print(a, b, c, d, x, y, z)
                        print(
                            f"(({a} {op_to_str[x]} {b}) {op_to_str[y]} {c}) {op_to_str[z]} {d}"
                        )
                except ZeroDivisionError:
                    pass
