from farmers.Polyculture import infinite_polyculture

poly = simulate(
    "simulation",
    Unlocks,
    {Items.Hay: 100000, Items.Wood: 100000},
    {"WHICH": "polyculture"},
    -1,
    64,
)
quick_print("POLY TIME", poly)  # 4770.3

no_poly = simulate(
    "simulation",
    Unlocks,
    {Items.Hay: 100000, Items.Wood: 100000},
    {"WHICH": "individual"},
    -1,
    64,
)
quick_print("NO-POLY TIME", no_poly)  # 10356.74

# TODO: Simulations of difference strats for optimization, Dinos, Leaderboard (at least one of everything)
infinite_polyculture()


# TODO: Scratch work
# TILE_COUNT = get_world_size() ** 2

# # change_hat(Hats.Dinosaur_Hat)
# # move(North)
# lazy_to_pos(6, 4)


# def turn_left(input):  # type: (Direction) -> Direction
#     if input == North:
#         return West
#     elif input == East:
#         return North
#     elif input == South:
#         return East
#     else:
#         return South


# while True:
#     for _ in range(4 * (get_world_size() - 1)):
#         pass
