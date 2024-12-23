# Since only some of the words are needed in the final output, the decode()
# function first filters the list of words to what is needed before sorting the
# list of words and concatenating them. The words that will be selected are on
# lines that start with triangle numbers. The nth triangle numbers can be
# computed with `T_n := (n * (n + 1)) / 2`. Given the formula, we can solve for n
# in terms of T_n, `n := floor(sqrt(T_n * 2))`. On each line, we calculate the
# closest value of n assuming that x is a triangle number and then check it
# actually is. Then we sort the lines to put them in order and concat the words
# using spaces.


from pathlib import Path


def decode(message_file: str) -> str:
    """Returns the word on every line of message_file that starts with a
    triangle number.
    """
    text = Path(message_file).read_text().strip()
    lines = text.splitlines()
    filtered_lines = []

    for line in lines:
        x, word = line.split()
        x = int(x)
        n = int((x * 2) ** 0.5)
        if x == (n * (n + 1)) // 2:
            filtered_lines.append((x, word))

    filtered_lines.sort()
    return " ".join(word for _, word in filtered_lines)


print(decode("coding_qual_input.txt"))
