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


def format_phrase(counts: Counter[str]) -> str:
    phrase = (
        "this pangram tallies "
        + ", ".join(f"{{count_of_{let}}}" for let in ALPHABET[:-1])
        + ", and {count_of_z}."
    )

    return phrase.format_map(
        {f"count_of_{letter}": get_spoken_count(counts, letter) for letter in ALPHABET}
    )


def check_provided_example():
    example = "this pangram tallies five a's, one b, one c, two d's, twenty-eight e's, eight f's, six g's, eight h's, thirteen i's, one j, one k, three l's, two m's, eighteen n's, fifteen o's, two p's, one q, seven r's, twenty-five s's, twenty-two t's, four u's, four v's, nine w's, two x's, four y's, and one z."
    example_letter_counts = Counter(c for c in example if c.isalpha())
    reconstructed = format_phrase(example_letter_counts)

    assert example == reconstructed, "transcription error"


check_provided_example()  # Sanity check

z_letter_counts = {let: z3.Int(f"count_of_{let}") for let in ALPHABET}
spoken_words: dict[int, str] = {i: num2words(i) for i in range(1, 201)}

required = Counter(c for c in "this pangram tallies ..., and ... ." if c.isalpha())


def calculate_ceilings() -> Counter[str]:
    max_ = Counter[str]()
    for num in range(1, 100):
        for char, count in Counter(c for c in spoken_words[num] if c.isalpha()).items():
            max_[char] = max(max_[char], count)
    for char in ALPHABET:
        # Give a generous upper bound (plus one because z, e.g., is never used in spoken numbers)
        max_[char] = max_[char] * 26 + required[char] + 1
    max_["s"] += 26  # account for plural references, e.g. "five f's"
    return max_


ceilings = calculate_ceilings()
# print(max(ceilings.values()))
# print(ceilings)
# seconds_in_year = 365 * 24 * 60 * 60
# print(100**26 / seconds_in_year)
# print(math.prod(ceilings.values()) / seconds_in_year)
# exit()

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
    solver.add(
        # Again, +1 to reference the letter itself
        required[being_counted] + 1 <= final_letter_count,
        final_letter_count <= ceilings[being_counted],
    )


min_total_char_count = 21 + 26 * (3 + 1)  # e.g. "one a"
max_total_char_count = 21 + 26 * (12 + 2)  # e.g. "seventy-seven a's"
total_char_count = sum(z_letter_counts.values())

# These calculated bounds (125-385) are much better than the inferred bounds of
# 21-983, based on the individual constraints from `required` and `ceilings`
solver.add(
    min_total_char_count <= total_char_count,
    total_char_count <= max_total_char_count,
)

# print(
#     min_total_char_count,
#     max_total_char_count,
#     sum(ceilings.values()),
#     sum(required.values()),
# )
# exit()


provided_solution = Counter({'e': 28, 's': 25, 't': 22, 'n': 18, 'o': 15, 'i': 13, 'w': 9, 'h': 8, 'f': 8, 'r': 7, 'g': 6, 'a': 5, 'v': 4, 'y': 4, 'u': 4, 'l': 3, 'p': 2, 'm': 2, 'd': 2, 'x': 2, 'b': 1, 'c': 1, 'j': 1, 'k': 1, 'q': 1, 'z': 1})  # fmt: skip
if False:
    print("Checking provided solution")
    for let, c in provided_solution.items():
        solver.add(z_letter_counts[let] == c)
        assert c <= ceilings[let]
else:
    print("Disallowing provided solution")
    conjunction = z3.And(
        *(z_letter_counts[let] == c for let, c in provided_solution.items())
    )
    solver.add(z3.Not(conjunction))


while True:
    print("Solving...")
    start = time.time()
    is_sat = solver.check()
    print("is sat?", is_sat)
    duration = time.time() - start
    if is_sat != z3.sat:
        break

    model = solver.model()
    print("model =", model)
    print(f"Took {duration:.5f} seconds")

    final_counts = Counter(
        {letter: model.eval(z_letter_counts[letter]).as_long() for letter in ALPHABET}
    )
    print("Solution:", repr(format_phrase(final_counts).capitalize()))

    # Disallow the found solution, and keep going
    solver.add(z3.Or(*(model[count] != count for count in z_letter_counts.values())))

print("\nGracefully exiting\n")
