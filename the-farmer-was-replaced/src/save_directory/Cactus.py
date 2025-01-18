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


while True:
    for _ in range(get_world_size()):
        bubble_sort(North, South, get_pos_y)
        move(East)
    for _ in range(get_world_size()):
        bubble_sort(East, West, get_pos_x)
        move(North)
    harvest()
    for_each(init_tile)
