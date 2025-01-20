from farmers.Utilities import clear_grid, for_each


def _handle_tile_carrot():
    if can_harvest():
        harvest()
        plant(Entities.Carrot)
    if get_water() < 0.13:
        use_item(Items.Water)


def infinite_carrots():
    clear_grid(Grounds.Soil, Entities.Carrot)

    while True:
        for_each(_handle_tile_carrot)
