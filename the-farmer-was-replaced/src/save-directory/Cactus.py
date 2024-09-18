TILE_COUNT = get_world_size() ** 2

stock_to(Items.Cactus_Seed, TILE_COUNT)
clear_grid(Grounds.Soil, Entities.Cactus)


def init_tile():
    plant(Entities.Cactus)


def bubble_sort(forward, backward, get_pos):
    _bubble_sort(forward, backward, get_pos, 0, get_world_size() - 1)


def _bubble_sort(forward, backward, get_pos, start, end):
    while get_pos() > 0:
        move(backward)

    while start < end:
        for _ in range(end - start):
            if measure() > measure(forward):
                swap(forward)
            move(forward)

        end -= 1
        for _ in range(end - start):
            if measure() < measure(backward):
                swap(backward)
            move(backward)

        start += 1

    return


while True:
    for _ in range(get_world_size()):
        bubble_sort(North, South, get_pos_y)
        move(East)
    for _ in range(get_world_size()):
        bubble_sort(East, West, get_pos_x)
        move(North)
    harvest()
    stock_to(Items.Cactus_Seed, TILE_COUNT)
    for_each(init_tile)
