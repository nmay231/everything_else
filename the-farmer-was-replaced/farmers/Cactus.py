from collections.abc import Callable  # noqa: F401

from farmers.Utilities import clear_grid, for_each


def _init_cell_cactus():
    plant(Entities.Cactus)


def _bubble_sort_cactus(forward, backward, get_pos):
    while get_pos() > 0:
        move(backward)

    left = 0  # Inclusive
    right = get_world_size() - 1  # Inclusive

    for _iterations in range(get_world_size()):
        prev_right, right = right, left
        for index in range(left, prev_right):
            if measure() > measure(forward):
                right = index
                swap(forward)
            move(forward)

        prev_left, left = left, right
        for index in range(prev_right, prev_left, -1):
            if measure() < measure(backward):
                left = index
                swap(backward)
            move(backward)

        if left >= right:
            return

        for _ in range(prev_left, left):
            move(forward)


def gen_cactus():
    for _ in range(get_world_size()):
        _bubble_sort_cactus(North, South, get_pos_y)
        move(East)
    for _ in range(get_world_size()):
        _bubble_sort_cactus(East, West, get_pos_x)
        move(North)
    harvest()
    for_each(_init_cell_cactus)


def infinite_cactus():
    clear_grid(Grounds.Soil, Entities.Cactus)

    while True:
        gen_cactus()
