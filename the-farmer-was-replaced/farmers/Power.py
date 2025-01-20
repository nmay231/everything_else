from farmers.lib.measure_int import measure
from farmers.Utilities import clear_grid, lazy_to_pos

TILE_COUNT = get_world_size() ** 2


def infinite_power():  # type: () -> None
    clear_grid(Grounds.Soil)

    by_petal_count = []  # type: list[list[tuple[int, int]]]
    for _ in range(15 + 1):
        by_petal_count.append([])

    while True:
        for _ in range(TILE_COUNT):
            plant(Entities.Sunflower)
            by_petal_count[measure()].append((get_pos_x(), get_pos_y()))

            move(North)
            if get_pos_y() == 0:
                move(East)

        petals = 15
        for _ in range(TILE_COUNT):
            while len(by_petal_count[petals]) == 0:
                petals -= 1
            x, y = by_petal_count[petals].pop(0)
            lazy_to_pos(x, y)

            if get_water() < 0.13:
                use_item(Items.Water)

            while not can_harvest():
                use_item(Items.Fertilizer)

            harvest()
