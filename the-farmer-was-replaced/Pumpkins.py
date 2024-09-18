TILE_COUNT = get_world_size() ** 2

stock_to(Items.Pumpkin_Seed, TILE_COUNT)
clear_grid(Grounds.Soil, Entities.Pumpkin)


def init_tile():
    plant(Entities.Pumpkin)


while True:
    to_check = []
    for _ in range(TILE_COUNT):
        if get_entity_type() == None or not can_harvest():
            to_check.append((get_pos_x(), get_pos_y()))
        move(North)
        if get_pos_y() == 0:
            move(East)

    while len(to_check) > 0:
        x, y = to_check.pop(0)
        to_pos(x, y)

        if get_entity_type() != None and not can_harvest():
            do_a_flip()

        if can_harvest():
            continue
        elif get_entity_type() == None:
            trade(Items.Pumpkin_Seed)
            plant(Entities.Pumpkin)
        to_check.append((x, y))
    harvest()

    stock_to(Items.Pumpkin_Seed, TILE_COUNT)
    for_each(init_tile)
