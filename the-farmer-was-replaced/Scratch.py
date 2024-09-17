TILE_COUNT = get_world_size() ** 2

stock_to(Items.Cactus_Seed, TILE_COUNT)
clear_grid(Grounds.Soil, Entities.Cactus)

def init_tile():
	plant(Entities.Cactus)

def neighbor(foward, backward):
	move(foward)
	measurement = measure()
	move(backward)
	return measurement
	
while True:
	move(South)
	while not can_harvest():
		do_a_flip()
	to_pos(0, 0)
	
	harvested = False
	prev_col = None
	for x in range(get_world_size()):
		column = []
		for y in range(get_world_size()):
			current = measure()
			column.append((True, current))
			if y > 0:
				prev_possible, prev_measure = column[y - 1]
				if prev_measure > current:
					column[y - 1] = (False, prev_measure)
					column[y] = (False, current)
					continue
			if x > 0:
				prev_possible, prev_measure = prev_col[y]
				if prev_measure > current:
					column[y] = (False, current)
					# No need to update previous column
				elif prev_possible:
					harvest()
					harvested = True
					break
			move(North)
		move(East)
		
		if harvested:
			break
		prev_col = column
	
	if not harvested:
		for y in range(get_world_size()):
			possible, _ = column[y]
			if possible:
				to_pos(get_world_size() - 1, y)
				harvest()
				break
	
	stock_to(Items.Cactus_Seed, TILE_COUNT)
	for_each(init_tile)
	