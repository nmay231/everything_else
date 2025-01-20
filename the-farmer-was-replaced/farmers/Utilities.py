from collections.abc import Callable  # noqa: F401


def lazy_to_pos(x, y):  # type: (int, int) -> None
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


def for_each(action):  # type: (Callable[[], Any]) -> None
    for _ in range(get_world_size() ** 2):
        action()
        move(North)
        if get_pos_y() == 0:
            move(East)


def clear_grid(ground=None, entity=None):  # type: (Ground | None, Entity | None) -> None
    def clear_tile():  # type: () -> None
        harvest()
        if ground != None and get_ground_type() != ground:
            till()
        if entity != None:
            plant(entity)

    for_each(clear_tile)
