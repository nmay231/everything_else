# Required because functions from other files actually use
# global variables from the main file's scope
TILE_COUNT = get_world_size() ** 2

clear_grid(Grounds.Soil, Entities.Bush)


def plant_tree_or_bush():
    if can_harvest():
        harvest()
        if (get_pos_x() + get_pos_y()) % 2 == 0:
            plant(Entities.Tree)
        else:
            plant(Entities.Bush)
    if get_water() < 0.13 and num_items(Items.Water_Tank) > 0:
        use_item(Items.Water_Tank)


while True:
    restock_empty_tanks()
    for_each(plant_tree_or_bush)
