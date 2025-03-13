import time
from collections import Counter

import z3
from num2words import num2words

ALPHABET = "abcdefghijklmnopqrstuvwxyz"


def get_spoken_count(counts: Counter[str], letter: str) -> str:
    count = counts[letter]

    if count == 1:
        return f"one {letter}"
    else:
        return f"{num2words(count)} {letter}'s"


def check_provided_example():
    example = "this pangram tallies five a's, one b, one c, two d's, twenty-eight e's, eight f's, six g's, eight h's, thirteen i's, one j, one k, three l's, two m's, eighteen n's, fifteen o's, two p's, one q, seven r's, twenty-five s's, twenty-two t's, four u's, four v's, nine w's, two x's, four y's, and one z."
    example_letter_counts = Counter(c for c in example if c.isalpha())

    phrase = (
        "this pangram tallies "
        + ", ".join("{count_of_" + let + "}" for let in ALPHABET[:-1])
        + ", and {count_of_z}."
    )

    reconstructed = phrase.format_map(
        {
            f"count_of_{letter}": get_spoken_count(example_letter_counts, letter)
            for letter in ALPHABET
        }
    )
    assert example == reconstructed, "transcription error"


check_provided_example()  # Sanity check

z_letter_counts = {let: z3.Int(f"count_of_{let}") for let in ALPHABET}
# z_number_counts = {i: z3.Int(f"count_of_{i}") for i in range(51)}
spoken_words: dict[int, str] = {i: num2words(i) for i in range(51)}

required = Counter(c for c in "this pangram tallies ..., and ... ." if c.isalpha())

# partial_sum: dict[str, z3.ArithRef] = {let: z3.IntVal(0) for let in ALPHABET}
solver = z3.Solver()
for being_counted, final_letter_count in z_letter_counts.items():
    partial_sum = z3.IntVal(
        required[being_counted] + 1
    )  # +1 to reference the letter itself

    for i, spoken in spoken_words.items():
        n_instances = spoken.count(being_counted)
        if n_instances == 0:
            continue

        for letter, z_letter in z_letter_counts.items():
            partial_sum += z3.If(z_letter == i, n_instances, 0)

    if being_counted == "s":
        # Account for plural counts, e.g. "five a's"
        partial_sum += sum(
            letter_count != 1 for letter_count in z_letter_counts.values()
        )

    solver.add(final_letter_count == partial_sum)

if False:
    print("Checking provided solution")
    solution = {'e': 28, 's': 25, 't': 22, 'n': 18, 'o': 15, 'i': 13, 'w': 9, 'h': 8, 'f': 8, 'r': 7, 'g': 6, 'a': 5, 'v': 4, 'y': 4, 'u': 4, 'l': 3, 'p': 2, 'm': 2, 'd': 2, 'x': 2, 'b': 1, 'c': 1, 'j': 1, 'k': 1, 'q': 1, 'z': 1}  # fmt: skip
    for let, c in solution.items():
        solver.add(z_letter_counts[let] == c)

print("Starting")
start = time.time()
solver.check()
duration = time.time() - start
print("Finished!")
print(solver.model())
print(f"Took in {duration:.5f} seconds")
