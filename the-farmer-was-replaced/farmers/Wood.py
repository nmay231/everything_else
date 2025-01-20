from farmers.utils import clear_grid, for_each


def _plant_tree_or_bush():  # type: () -> None
    if can_harvest():
        harvest()
        if (get_pos_x() + get_pos_y()) % 2 == 0:
            plant(Entities.Tree)
        else:
            plant(Entities.Bush)
    if get_water() < 0.13:
        use_item(Items.Water)


def infinite_trees():  # type: () -> None
    clear_grid(Grounds.Soil, Entities.Bush)

    while True:
        for_each(_plant_tree_or_bush)
