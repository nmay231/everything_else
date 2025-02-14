from operator import add, mul, sub, truediv
from itertools import permutations, combinations_with_replacement

# I was struggling with this puzzle, so I decided it would make a good programming puzzle

all_ops = (add, mul, sub, truediv)
op_to_str = {add: "+", mul: "*", sub: "-", truediv: "/"}

# TODO: I thought I already finished this? Maybe I did and this old git stash with this comment block is completely irrelevant

# How could this fail? This fails because I forgot about commutativity.
# I should probably try out possibilities using a different method that takes into account order of operations and the fact you only get one paren grouping.
# Basically, I need to enumerate all states of a NSM
# I should work on this in a general fashion since this is related work to my other stuff

all_nums = (7, 7, 9, 6)


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
