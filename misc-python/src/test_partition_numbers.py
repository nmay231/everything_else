from src.partition_numbers import (
    brute_force_partitions,
    euler_partitions,
    partitions_of_rank,
    partitions_up_to,
    pentagonal,
)


def test_pentagonal():
    assert [*pentagonal(40)] == [1, 2, 5, 7, 12, 15, 22, 26, 35, 40]


def test_counting_partitions():
    mine = partitions_up_to(100)
    euler = euler_partitions(100)[1:]  # Skip p(0)
    assert (mine == euler).all()


def test_brute_force_partitions():
    p_n = euler_partitions(50)
    for n in range(50 + 1):
        parts = brute_force_partitions(n)
        assert p_n[n] == len(parts)
        assert all(sum(part) == n for part in parts)


def test_partitions_by_rank():
    p_n = euler_partitions(50)[1:]  # Skip p(0)
    for n in range(1, 50 + 1):
        parts = brute_force_partitions(n)
        assert p_n[n - 1] == len(parts)
        assert all(sum(part) == n for part in parts)

        for rank_i in range(int(n**0.5)):
            count_of_parts = partitions_of_rank(rank_i + 1, 50)[n - 1]

            parts_of_rank_i = [
                part
                for part in parts
                if len(part) > rank_i
                and part[rank_i] > rank_i
                and (len(part) == rank_i + 1 or part[rank_i + 1] <= rank_i + 1)
            ]
            assert count_of_parts == len(parts_of_rank_i)


def test_partitions_by_rank_and_elbow_width_equivalence():
    p_n = euler_partitions(50)[1:]  # Skip p(0)
    for n in range(1, 50 + 1):
        parts = brute_force_partitions(n)
        assert p_n[n - 1] == len(parts)
        assert all(sum(part) == n for part in parts)

        for width in range(1, int(n**0.5) + 1):
            rank_sum = 0
            for rank_i in range(width, int(n**0.5) + 1):
                rank_sum += partitions_of_rank(rank_i, 50)[n - 1]
            # of_rank = partitions_of_rank(width, 50)

            tmp = [
                part
                for part in parts
                if part[-1] >= width
                and len(part) >= width
                and part[0] == part[width - 1]
            ]
            if rank_sum != len(tmp):
                print(f"{(n, width, len(tmp), tmp, rank_sum)=}")

            assert rank_sum == len(tmp)
