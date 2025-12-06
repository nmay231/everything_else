"""
There's a puzzle-y challenge in the game Unnamed Space Idle where you place
different components in a grid and they affect each other in a certain way to
give you stats. I don't care to summarize it here--partially because it's
spoilers but mostly because I'm lazy--but if you're following along, you can see
I've only got three of the four bases unlocked, and base three only partially.

Unfortunately, I realized towards the end that I am not actually able to solve
this challenge using z3, and that's because z3 doesn't like variables in
exponents of equations (more info in the comment towards the bottom of the
file).

In the end, the work I've put towards this can still somewhat be put to good use
because the challenge in the game simply repurposes the Bases mechanic in a
creative way. The Bases mechanic itself is a little simpler and each base is
independent of the other. They still have the issue of boosters putting
variables in exponents, but I can resolve that by using the trick I used for
boosters in the challenge. In the comment towards the end of the file, I explain
why I can't really use that trick to resolve the other exponent problem I have
in the challenge.
"""

import textwrap
from dataclasses import dataclass, field

import z3

from utils.misc import is_uninterpreted_constant

type Z3AnyRef = (
    z3.PatternRef
    | z3.QuantifierRef
    | z3.BoolRef
    | z3.IntNumRef
    | z3.ArithRef
    | z3.RatNumRef
    | z3.AlgebraicNumRef
    | z3.BitVecNumRef
    | z3.BitVecRef
    | z3.ArrayRef
    | z3.DatatypeRef
    | z3.FPNumRef
    | z3.FPRef
    | z3.FiniteDomainNumRef
    | z3.FiniteDomainRef
    | z3.FPRMRef
    | z3.SeqRef
    | z3.CharRef
    | z3.ReRef
    | z3.ExprRef
)


Cell_ = z3.Datatype("Cell")
Cell_.declare("empty")
Cell_.declare("shield")
Cell_.declare("damage")
Cell_.declare("booster")
Cell_.declare("battle_mult")
Cell_.declare("b_reduction")

Cell: z3.DatatypeSortRef = Cell_.create()


def grid_from_string(grid_str: str, grid_name: str) -> list[list[Z3AnyRef]]:
    grid_bool = [
        [c == "1" for c in row]
        for row in textwrap.dedent(grid_str).strip().splitlines()
    ]
    result = [
        [
            z3.Const(f"{grid_name}[{rowi}][{coli}]", Cell) if exists else Cell.empty
            for coli, exists in enumerate(row)
        ]
        for rowi, row in enumerate(grid_bool)
    ]
    assert len(row_lengths := set(map(len, result))) == 1, (
        f"Expected grid to have the same number of elements per row: {row_lengths}"
    )
    return result


@dataclass
class Contribution:
    raw_shields: Z3AnyRef = z3.RealVal(0)
    raw_damage: Z3AnyRef = z3.RealVal(0)
    battle_mult: Z3AnyRef = z3.RealVal(0)
    n_boosters: Z3AnyRef = z3.RealVal(0)
    grid_constraints: list[z3.ExprRef] = field(default_factory=list)

    def __add__(self, other: object) -> "Contribution":
        if not isinstance(other, Contribution):
            raise TypeError
        return Contribution(
            raw_shields=self.raw_shields + other.raw_shields,
            raw_damage=self.raw_damage + other.raw_damage,
            battle_mult=self.battle_mult + other.battle_mult,
            n_boosters=self.n_boosters + other.n_boosters,
            grid_constraints=[*self.grid_constraints, *other.grid_constraints],
        )


def grid1_contribution() -> Contribution:
    grid = grid_from_string(
        """
        1101
        1111
        1111
        1011
        """,
        "grid1",
    )
    contr = Contribution()
    booster_strength = 8.21
    for rowi, row in enumerate(grid):
        for coli, cell in enumerate(row):
            boosters = []
            if rowi > 0:
                boosters.append(grid[rowi - 1][coli] == Cell.booster)
            if coli > 0:
                boosters.append(grid[rowi][coli - 1] == Cell.booster)
            if rowi + 1 < len(grid):
                boosters.append(grid[rowi + 1][coli] == Cell.booster)
            if coli + 1 < len(grid[0]):
                boosters.append(grid[rowi][coli + 1] == Cell.booster)
            boosters_count = z3.Sum(*boosters)

            for count in range(0, 5):
                contr.raw_shields += (
                    (cell == Cell.shield)
                    * 4000
                    * (boosters_count == count)
                    * booster_strength**count
                )
                contr.raw_damage += (
                    (cell == Cell.damage)
                    * 1099
                    * (boosters_count == count)
                    * booster_strength**count
                )

            if is_uninterpreted_constant(cell):
                contr.grid_constraints.append(
                    z3.Or(
                        *(
                            cell == type
                            for type in [Cell.booster, Cell.shield, Cell.damage]
                        )
                    )
                )
            contr.n_boosters += cell == Cell.booster

    return contr


def grid2_contribution() -> Contribution:
    grid = grid_from_string(
        """
        11111
        11011
        11011
        11111
        """,
        "grid2",
    )
    contr = Contribution()
    booster_strength = 3.53
    for rowi, row in enumerate(grid):
        for coli, cell in enumerate(row):
            boosters = []
            if rowi > 0:
                boosters.append(grid[rowi - 1][coli] == Cell.booster)
            if coli > 0:
                boosters.append(grid[rowi][coli - 1] == Cell.booster)
            if rowi + 1 < len(grid):
                boosters.append(grid[rowi + 1][coli] == Cell.booster)
            if coli + 1 < len(grid[0]):
                boosters.append(grid[rowi][coli + 1] == Cell.booster)
            boosters_count = z3.Sum(*boosters)

            for count in range(0, 5):
                contr.raw_shields += (
                    (cell == Cell.shield)
                    * 3.88e4
                    * (boosters_count == count)
                    * booster_strength**count
                )
                contr.raw_damage += (
                    (cell == Cell.damage)
                    * 9700
                    * (boosters_count == count)
                    * booster_strength**count
                )
                contr.battle_mult += (
                    (cell == Cell.battle_mult)
                    * 1.98
                    * (boosters_count == count)
                    * booster_strength**count
                )

            if is_uninterpreted_constant(cell):
                contr.grid_constraints.extend(
                    cell != type for type in [Cell.empty, Cell.b_reduction]
                )
                contr.n_boosters += cell == Cell.booster

    return contr


def grid3_contribution() -> Contribution:
    grid = grid_from_string(
        """
        10101
        11111
        11011
        11111
        01110
        """,
        "grid3",
    )
    contr = Contribution()
    booster_strength = 1.97436
    for rowi, row in enumerate(grid):
        for coli, cell in enumerate(row):
            boosters = []
            if rowi > 0:
                boosters.append(grid[rowi - 1][coli] == Cell.booster)
            if coli > 0:
                boosters.append(grid[rowi][coli - 1] == Cell.booster)
            if rowi + 1 < len(grid):
                boosters.append(grid[rowi + 1][coli] == Cell.booster)
            if coli + 1 < len(grid[0]):
                boosters.append(grid[rowi][coli + 1] == Cell.booster)
            boosters_count = z3.Sum(*boosters)

            for count in range(0, 5):
                contr.raw_shields += (
                    (cell == Cell.shield)
                    * 4e5
                    * (boosters_count == count)
                    * booster_strength**count
                )
                contr.raw_damage += (
                    (cell == Cell.damage)
                    * 8.2e4
                    * (boosters_count == count)
                    * booster_strength**count
                )
                contr.battle_mult += (
                    (cell == Cell.battle_mult)
                    * 9.0
                    * (boosters_count == count)
                    * booster_strength**count
                )

                # TODO: The format of the equation suggests I'll need to track this in a separate variable in the future
                contr.n_boosters -= (
                    (cell == Cell.b_reduction)
                    * 2.0
                    * (boosters_count == count)
                    * booster_strength**count
                )

            if is_uninterpreted_constant(cell):
                contr.grid_constraints.append(cell != Cell.empty)
                contr.n_boosters += cell == Cell.booster

    return contr


solver = z3.Solver()

contributions = sum(
    [
        grid1_contribution(),
        grid2_contribution(),
        grid3_contribution(),
    ],
    start=Contribution(),
)

"""
TODO: I wrote all of this only to realize that Z3 doesn't like variables in
exponents. That's why I did the weird trick for each grid where I listed each
possible power for the booster strengths instead of the more obvious route of
putting into the exponent. I would either need to do something similar by
binning the exponents and creating a massive list of possible values, or turn
away from this method entirely.

I guess I could bin the number of boosters earlier by assuming the booster
reducers can only be boosted to integer values, but that doesn't help very much
I would assume...
"""

shields = (
    contributions.raw_shields
    * contributions.battle_mult
    / (z3.RealVal(1.25) ** contributions.n_boosters)
)
damage = (
    contributions.raw_damage
    * contributions.battle_mult
    / (z3.RealVal(1.25) ** contributions.n_boosters)
)

solver = z3.Solver()
solver.add(*contributions.grid_constraints)

print(result := solver.check(shields > 3.44e7, damage > 4.28e8))
if result == z3.sat:
    print(solver.model)
