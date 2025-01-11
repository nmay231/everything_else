clear_grid(Grounds.Soil, Entities.Bush)


def plant_tree_or_bush():
    if can_harvest():
        harvest()
        if (get_pos_x() + get_pos_y()) % 2 == 0:
            plant(Entities.Tree)
        else:
            plant(Entities.Bush)
    if get_water() < 0.13:
        use_item(Items.Water)


while True:
    for_each(plant_tree_or_bush)
