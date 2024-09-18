# These utility functions actually use global variables from the main file's
# scope, but I also define it here to get type checking happy
TILE_COUNT = 0


def stock_to(item, count):
    trade(item, max(0, count - num_items(item)))


def to_pos(x, y):
    size = get_world_size()
    if get_pos_x() != x:
        if (size - get_pos_x() + x) % size < (size - x + get_pos_x()) % size:
            direc = East
        else:
            direc = West
        while get_pos_x() != x:
            move(direc)
    if get_pos_y() != y:
        if (size - get_pos_y() + y) % size < (size - y + get_pos_y()) % size:
            direc = North
        else:
            direc = South
        while get_pos_y() != y:
            move(direc)


def for_each(action):
    for _ in range(get_world_size() ** 2):
        action()
        move(North)
        if get_pos_y() == 0:
            move(East)


def clear_grid(ground=None, entity=None):
    def clear_tile():
        harvest()
        if ground != None and get_ground_type() != ground:
            till()
        if entity != None:
            plant(entity)

    for_each(clear_tile)


def restock_empty_tanks():
    if num_items(Items.Water_Tank) < TILE_COUNT:
        stock_to(Items.Empty_Tank, 2 * TILE_COUNT)
