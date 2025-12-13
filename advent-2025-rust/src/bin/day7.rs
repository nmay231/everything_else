use std::collections::HashSet;

fn part1(text: &str) -> usize {
    let mut lines = text.lines();
    let mut indexes = HashSet::new();
    indexes.insert(
        lines
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .find_map(|(index, c)| if c != 'S' { None } else { Some(index) })
            .unwrap(),
    );

    let mut count = 0;

    for line in lines {
        for (index, char) in line.chars().enumerate() {
            match (indexes.contains(&index), char) {
                (false, _) | (true, '.') => continue,
                (true, '^') => {
                    indexes.remove(&index);
                    // NOTE: Based on the sample inputs, we can (or at least
                    // will) assume that there are no bounds checks needed.
                    indexes.insert(index - 1);
                    indexes.insert(index + 1);

                    count += 1;
                }
                (true, _) => unreachable!(),
            }
        }
    }

    return count;
}

fn part2(_text: &str) -> usize {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day7.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 21);
    }
}
