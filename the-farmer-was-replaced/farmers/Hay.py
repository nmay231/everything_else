from farmers.Utilities import clear_grid, for_each


def infinite_hay():  # type: () -> None
    clear_grid(Grounds.Grassland)

    while True:
        for_each(harvest)
