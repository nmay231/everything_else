from typing import Generator

# Source: https://www.youtube.com/watch?v=jUM_Dpt6yu0
# This is going to (mostly) be implemented from memory


def permutations(arg: list) -> Generator[tuple, None, None]:
    indices = list(range(len(arg)))
    for perm in _permute_indices(indices):
        yield tuple(arg[i] for i in perm)


def _permute_indices(indices: list[int]) -> Generator[tuple, None, None]:
    if len(indices) <= 1:
        yield tuple(indices)
        return

    for i, index in enumerate(indices):
        for perm in _permute_indices(indices[:i] + indices[i + 1 :]):
            yield (index,) + perm


"""/
123
132
213
231
312
321
"""

"""/
1234
1243
1324
1342
1423
1432
2...
"""
