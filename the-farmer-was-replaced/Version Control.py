dance_state = None
def dance(_):
	# Simple, stateless, columnwise movement
	move(North)
	if get_pos_y() == 0:
		move(East)
	return None

dance_state = True
def dance(go_up):
	# Zig zag columnwise (uses a state machine)
	y = get_pos_y()
	size = get_world_size()

	if go_up:
		if y == size - 1:
			move(East)
			go_up = False
		else:
			move(North)
	else:
		if y == 0:
			move(East)
			go_up = True
		else:
			move(South)
	return go_up

dance_state = dance(dance_state)