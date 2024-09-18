TILE_COUNT = get_world_size() ** 2
CURRENT_LOOP = "cactus"

stock_to(Items.Cactus_Seed, TILE_COUNT)
clear_grid(Grounds.Soil, Entities.Cactus)


def init_tile():
    plant(Entities.Cactus)


def bubble_sort(forward, backward, get_pos):
    while get_pos() > 0:
        move(backward)

    length = get_world_size() - 1
    while length > 0:
        for _ in range(length):
            if measure() > measure(forward):
                swap(forward)
            move(forward)

        for _ in range(length):
            if measure() < measure(backward):
                swap(backward)
            move(backward)
        move(forward)
        length -= 2


while CURRENT_LOOP == "cactus":
    for _ in range(get_world_size()):
        bubble_sort(North, South, get_pos_y)
        move(East)
    for _ in range(get_world_size()):
        bubble_sort(East, West, get_pos_x)
        move(North)
    harvest()
    stock_to(Items.Cactus_Seed, TILE_COUNT)
    for_each(init_tile)
