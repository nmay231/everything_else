from dataclasses import dataclass

from hypothesis import given
from hypothesis import strategies as st


@dataclass
class State:
    row: list[int]
    index: int


state = State([], 0)


def get_world_size():
    return len(state.row)


def move(direction):
    assert 0 <= state.index + direction < len(state.row)
    state.index += direction


def measure(direction=0):
    assert 0 <= state.index + direction < len(state.row)
    return state.row[state.index + direction]


def swap(direction):
    assert 0 <= state.index + direction < len(state.row)
    state.row[state.index], state.row[state.index + direction] = (
        state.row[state.index + direction],
        state.row[state.index],
    )


# Copied from Cactus.py (because otherwise I would need to mock all the global builtins)
def bubble_sort(forward, backward, get_pos):
    while get_pos() > 0:
        move(backward)

    length = get_world_size() - 1
    while length > 0:
        for _ in range(length):
            if measure() > measure(forward):
                swap(forward)
            move(forward)

        for _ in range(length):
            if measure() < measure(backward):
                swap(backward)
            move(backward)
        move(forward)
        length -= 2


@given(
    row=st.lists(st.integers(min_value=0, max_value=9), min_size=1),
    start_index=st.integers(min_value=0),
)
def test_bubblesort(row, start_index):
    state.row = row[:]
    state.index = start_index % len(row)
    # state.row = [1, 1, 0]
    # state.index = 0
    bubble_sort(1, -1, lambda: state.index)

    assert state.row == sorted(row)


def test_other(mocker):
    mocker.patch("Cactus.CURRENT_LOOP", None)
    from .save_directory.Cactus import bubble_sort

    print(bubble_sort)


# Scrap this. I think I should just add a main.py for running each crop thing
# and refactor everything to do that. I should focus on work, but I can also add
# credits real quick to 2hol-names before I do that.
