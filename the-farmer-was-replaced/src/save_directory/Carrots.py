clear_grid(Grounds.Soil, Entities.Carrot)


def handle_tile():
    if can_harvest():
        harvest()
        plant(Entities.Carrot)
    if get_water() < 0.13:
        use_item(Items.Water)


while True:
    for_each(handle_tile)
