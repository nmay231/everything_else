import numpy as np


def fill_diagonal(arr: np.ndarray, diag_index: int, fill: int):
    """Given the `diag_index` (positive points down-left) fill the arr with `fill`"""
    assert diag_index >= 0  # TODO
    assert arr.shape[0] == arr.shape[1]  # Must be square
    len_ = len(arr)
    arr[np.arange(diag_index, len_), np.arange(len_ - diag_index)] = fill


def slope(n: int, spacing: int):
    """Return a n by n matrix with each lower-half diagonal filled by incrementing
    numbers with `spacing` gaps between each diagonal, e.g.

    >>> slope(6, 2)
    [[1. 0. 0. 0. 0. 0.]
     [0. 1. 0. 0. 0. 0.]
     [2. 0. 1. 0. 0. 0.]
     [0. 2. 0. 1. 0. 0.]
     [3. 0. 2. 0. 1. 0.]
     [0. 3. 0. 2. 0. 1.]]
    """
    arr = np.zeros((n, n))
    for fill, diag_index in zip(range(1, n + 1), range(0, n, spacing)):
        fill_diagonal(arr, diag_index, fill)
    return arr


def partitions_up_to(n: int) -> np.ndarray:
    """Returns the partition numbers `[p(1), ... p(n)]`"""
    sequence_of_differences = np.zeros(n)
    sequence_of_differences[0] = 1
    result = np.zeros(n)
    for i in range(1, int(n**0.5) + 1):
        sequence_of_differences = (
            slope(len(sequence_of_differences), i) @ sequence_of_differences
        )
        result[i**2 - 1 :] += sequence_of_differences
        sequence_of_differences = sequence_of_differences[: -(2 * i + 1)]
    return result


def euler_partitions(n: int) -> np.ndarray:
    """Returns the partition numbers `[p(0), ... p(n)]`"""
    indexes = np.zeros(n)
    for i, element in enumerate(pentagonal(n)):
        # +q_1 +q_2 -q_3 -q_4 +a_5 +a_6 -a_7 ...
        indexes[n - element] = 1 if not i & 2 else -1

    result = np.zeros(n + 1)
    result[0] = 1

    for i in range(1, n + 1):
        result[i] = result[:i].dot(indexes[-i:])

    return result


def pentagonal(n: int):
    """Generate *generalized* pentagonal numbers up to `pentagonal(i) <= n`"""
    pent = 0
    for i in range(n):
        for large in [True, False]:
            pent += 2 * i + 1 if large else i + 1
            if pent > n:
                return
            yield pent


def flops_euler_method(n: int):
    """How many flops to calculate partitions up to n using Euler's pentagonal method"""
    assert n > 1
    total_flops = 0
    flops_in_current_block = 1
    for index in range(1, n):
        for is_large_step in [True, False]:
            block_size = 2 * index - 1 if is_large_step else index

            if block_size >= n:
                return total_flops + flops_in_current_block * n
            n -= block_size

            total_flops += flops_in_current_block * block_size
            flops_in_current_block += 1


if __name__ == "__main__":
    # Then I could work on calculating the flops of each, probably by using rust and simply incrementing a number instead of trying to be smart about it, but then that means I actually have to look at the compiled output if I want to be completely accurate.
    assert [*pentagonal(40)] == [1, 2, 5, 7, 12, 15, 22, 26, 35, 40]

    mine = partitions_up_to(100)
    euler = euler_partitions(100)[1:]
    assert (mine == euler).all()
