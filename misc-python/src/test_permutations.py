from itertools import permutations as reference_permutations
from itertools import zip_longest
from typing import Iterable, TypeVar

from hypothesis import given
from hypothesis.strategies import integers, lists

T = TypeVar("T")


def all_equal(a_list: Iterable[T], b_list: Iterable[T]) -> bool:
    """https://stackoverflow.com/questions/25216504/how-to-detect-that-two-python-iterators-yield-the-same-items"""
    return all(a == b for a, b in zip_longest(a_list, b_list, fillvalue=object()))


@given(lists(integers(), max_size=9))
def test_permutations_follow_reference_implementation(a_list):
    assert all_equal(reference_permutations(a_list), reference_permutations(a_list))


def test_permutations_empty():
    a_list = []
    assert all_equal(reference_permutations(a_list), reference_permutations(a_list))


def test_permutations_1():
    a_list = [1]
    assert all_equal(reference_permutations(a_list), reference_permutations(a_list))


def test_permutations_3():
    a_list = [1, 2, 3]
    assert all_equal(reference_permutations(a_list), reference_permutations(a_list))
