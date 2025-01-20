def _init_treasure():  # type: () -> None
    clear()
    for _ in range(4):
        move(East)
        move(North)

    plant(Entities.Bush)
    n_substance = get_world_size() * num_unlocked(Unlocks.Mazes)
    use_item(Items.Weird_Substance, n_substance)


# TODO: Put into Utilities?
def _opposite_direc_treasure(input):  # type: (Direction) -> Direction
    if input == North:
        return South
    elif input == East:
        return West
    elif input == South:
        return North
    else:
        return East


def _turn_left_treasure(input):  # type: (Direction) -> Direction
    if input == North:
        return West
    elif input == East:
        return North
    elif input == South:
        return East
    else:
        return South


def _turn_right_treasure(input):  # type: (Direction) -> Direction
    return _turn_left_treasure(_opposite_direc_treasure(input))


def infinite_treasure():  # type: () -> None
    if get_entity_type() != Entities.Hedge:
        _init_treasure()

    # TODO: Reuse mazes that have a low circumference (aka, a low maximum eccentricity)
    while True:
        direc = North
        while get_entity_type() != Entities.Treasure:
            while not move(direc):
                direc = _turn_right_treasure(direc)
            direc = _turn_left_treasure(direc)
        harvest()
        _init_treasure()
