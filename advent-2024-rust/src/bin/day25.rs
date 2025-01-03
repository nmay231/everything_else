use itertools::Itertools;

fn part1(text: &str) -> usize {
    let mut locks = vec![];
    let mut keys = vec![];

    for block in text.lines().chunks(8).into_iter() {
        let mut block = block.peekable();
        let desired_char = match block.peek() {
            None => unreachable!("Expected at least one row in a block"),
            Some(lock) if lock.chars().all(|c| c == '#') => '#',
            Some(key) if key.chars().all(|c| c == '.') => '.',
            Some(row) => {
                unreachable!("Expected the top row to be either all '#' or '.': {}", row)
            }
        };
        let mut pins = vec![0; 5];

        for (row_i, row) in block.enumerate() {
            if row_i == 7 {
                assert_eq!(row, "");
                continue;
            }
            for (i, c) in row.chars().enumerate() {
                if c == desired_char {
                    pins[i] = row_i;
                }
            }
        }

        if desired_char == '#' {
            locks.push(pins);
        } else {
            keys.push(pins.into_iter().map(|pin| 5 - pin).collect_vec());
        }
    }

    let mut total = 0;
    for (key, lock) in keys.iter().cartesian_product(locks.iter()) {
        if key.iter().zip(lock).all(|(key, lock)| key + lock <= 5) {
            total += 1;
        }
    }
    return total;
}

fn part2(_text: &str) -> usize {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day25.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 3);
    }
}
