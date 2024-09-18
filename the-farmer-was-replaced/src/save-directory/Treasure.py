TILE_COUNT = get_world_size() ** 2


def init():
    clear()
    for _ in range(4):
        move(East)
        move(North)
    plant(Entities.Bush)
    while get_entity_type() == Entities.Bush:
        trade(Items.Fertilizer)
        use_item(Items.Fertilizer)


def opposite_direc(input):
    if input == North:
        return South
    elif input == East:
        return West
    elif input == South:
        return North
    elif input == West:
        return East


def turn_left(input):
    if input == North:
        return West
    elif input == East:
        return North
    elif input == South:
        return East
    elif input == West:
        return South


def turn_right(input):
    return turn_left(opposite_direc(input))


if get_entity_type() != Entities.Hedge:
    init()

while True:
    direc = North
    while get_entity_type() != Entities.Treasure:
        while not move(direc):
            direc = turn_right(direc)
        direc = turn_left(direc)
    harvest()
    init()
