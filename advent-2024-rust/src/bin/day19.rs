use std::collections::VecDeque;

use itertools::Itertools;

type Output = usize;

fn part1(text: &str) -> Output {
    let towels = text.split_once('\n').expect("Missing towels").0.split(", ");

    let characters = ['w', 'u', 'b', 'r', 'g'];
    let mut str_len = 1;
    let mut combinations = vec![""];
    while combinations.len() * 4 < characters.len().pow(str_len) {
        for (prefix, next_char) in combinations.iter().cartesian_product(characters) {}
    }

    let mut total = 0;
    'patterns: for pattern in text.lines().skip(2) {
        println!("pattern: {}", pattern);
        let pattern = pattern.chars().collect_vec();
        let mut frontier = VecDeque::from([(&pattern[..], &trie)]);

        while let Some((pattern, current_towel)) = frontier.pop_back() {
            match (pattern.first(), current_towel.is_terminal) {
                // Perfect match
                (None, true) => {
                    total += 1;
                    continue 'patterns;
                }
                // Leftover towel
                (None, false) => continue,
                // Need another towel to continue the pattern
                (Some(char), is_terminal) => {
                    if is_terminal {
                        frontier.push_back((pattern, &trie));
                    }
                    match current_towel.map.get(char) {
                        None => continue,
                        Some(child) => {
                            frontier.push_back((&pattern[1..], child));
                        }
                    }
                }
            }
        }
    }

    total
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day19.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        bwu
        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 7);
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
