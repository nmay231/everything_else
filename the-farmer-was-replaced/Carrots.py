TILE_COUNT = get_world_size() ** 2
stock_to(Items.Carrot_Seed, TILE_COUNT)
clear_grid(Grounds.Soil, Entities.Carrots)


def handle_tile():
	if can_harvest():
		harvest()
		plant(Entities.Carrots)
	elif num_items(Items.Water_Tank) > 0:
		use_item(Items.Water_Tank)


while True:
	stock_to(Items.Carrot_Seed, TILE_COUNT)
	restock_empty_tanks()
	for_each(handle_tile)

	