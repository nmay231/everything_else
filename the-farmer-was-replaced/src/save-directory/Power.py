TILE_COUNT = get_world_size() ** 2

clear_grid(Grounds.Soil)


by_petal_count = []
for _ in range(15 + 1):
    by_petal_count.append([])


while True:
    stock_to(Items.Sunflower_Seed, TILE_COUNT)
    stock_to(Items.Fertilizer, 3 * TILE_COUNT)
    restock_empty_tanks()

    for _ in range(TILE_COUNT):
        plant(Entities.Sunflower)
        by_petal_count[measure()].append((get_pos_x(), get_pos_y()))

        move(North)
        if get_pos_y() == 0:
            move(East)
    beginning = False

    for _ in range(TILE_COUNT):
        for x in range(9):
            petals = 15 - x
            if len(by_petal_count[petals]) > 0:
                break
        x, y = by_petal_count[petals].pop(0)
        to_pos(x, y)
        if not can_harvest():
            while not can_harvest():
                use_item(Items.Fertilizer)
            while get_water() < 0.50 and num_items(Items.Water_Tank) > 0:
                use_item(Items.Water_Tank)

        harvest()
