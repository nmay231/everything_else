TILE_COUNT = get_world_size() ** 2
#clear()

# There is no point to harvesting groups larger than 4, with respect to points
# It might be better to go for large groups to avoid 

def dict_get(dict, key, default=None):
	if key in dict:
		return dict[key]
	else:
		return default

def replant():
	start = (get_pos_x(), get_pos_y())
	frontier = [start]
	visited = []
	while len(frontier) > 0:
		x, y = frontier.pop()
		# What is effeciency?
		to_pos(x, y)
		if get_entity_type() == Entities.Dinosaur:
			continue
		use_item(Items.Egg)
		neighbors = [
			(x + 1, y),
			(x - 1, y),
			(x, y + 1),
			(x, y - 1),
		]
		for neighbor in neighbors:
			nx, ny = neighbor
			if nx < 0 or ny < 0 or nx >= get_world_size() or ny >= get_world_size():
				continue
			if neighbor not in visited:
				frontier.append(neighbor)

def harvest_if_reasonable():
	center = measure()
	counts = {center: 1}
	for direc in [North, East, South, West]:
		key = measure(direc)
		counts[key] = 1 + dict_get(counts, key, 0)
	
	if len(counts) > 2 or counts[center] == 2:
		return
	elif counts[center] == 1:
		swap(North)
	# I have no less than a group of three of the same type
	# Three is less practical, but it's also useless to keep searching indefinitely
	# For now, I also skip 3, but I was thinking of just leaving it because it could
	# be a group of 4 that I simply can't see here.
	harvest()
	#stock_to(Items.Egg, TILE_COUNT)
	#replant()

def init():
	if get_entity_type() != Entities.Dinosaur:
		use_item(Items.Egg)

stock_to(Items.Egg, TILE_COUNT)
for_each(init)


while True:
	for_each(harvest_if_reasonable)
	stock_to(Items.Egg, TILE_COUNT)
	for_each(init)