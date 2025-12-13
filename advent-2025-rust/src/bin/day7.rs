use std::collections::{HashMap, HashSet};

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

fn part2(text: &str) -> usize {
    let mut lines = text.lines();
    let mut universe_counts = HashMap::new();
    universe_counts.insert(
        lines
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .find_map(|(index, c)| if c != 'S' { None } else { Some(index) })
            .unwrap(),
        1,
    );

    for line in lines {
        for (index, char) in line.chars().enumerate() {
            match (universe_counts.get(&index), char) {
                (None, _) | (Some(_), '.') => continue,
                (Some(n_universes), '^') => {
                    let n_universes = *n_universes;

                    universe_counts.remove(&index);
                    *universe_counts.entry(index - 1).or_default() += n_universes;
                    *universe_counts.entry(index + 1).or_default() += n_universes;
                }
                (Some(_), _) => unreachable!(),
            }
        }
    }

    return universe_counts.into_values().sum::<usize>();
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

    #[test]
    fn part2_given_example() {
        assert_eq!(crate::part2(TEXT1), 40);
    }
}
