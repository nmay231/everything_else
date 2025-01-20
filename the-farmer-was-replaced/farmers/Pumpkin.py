from farmers.utils import clear_grid, for_each, lazy_to_pos

TILE_COUNT = get_world_size() ** 2


def _init_tile_pumpkin():  # type: () -> None
    plant(Entities.Pumpkin)


def infinite_pumpkins():  # type: () -> None
    clear_grid(Grounds.Soil, Entities.Pumpkin)

    while True:
        to_check = []
        for _ in range(TILE_COUNT):
            if get_entity_type() == None:
                to_check.append((get_pos_x(), get_pos_y()))
                plant(Entities.Pumpkin)
            elif not can_harvest():
                to_check.append((get_pos_x(), get_pos_y()))

            move(North)
            if get_pos_y() == 0:
                move(East)

            if get_water() < 0.25:
                use_item(Items.Water)

        while len(to_check) > 0:
            x, y = to_check.pop(0)
            lazy_to_pos(x, y)

            if can_harvest():
                continue

            # TODO: Do I want this much weird substance from fertilized/infected plants?
            while get_entity_type() != None and not can_harvest():
                use_item(Items.Fertilizer)

            if get_entity_type() == None:
                plant(Entities.Pumpkin)
            to_check.append((x, y))
        harvest()

        for_each(_init_tile_pumpkin)
