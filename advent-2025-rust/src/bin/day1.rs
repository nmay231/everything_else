fn part1(text: &str) -> usize {
    let mut dial = 50usize;
    let mut goal = 0;

    for line in text.lines() {
        let turns = line[1..].parse::<usize>().expect("Not a number!");
        match &line[..1] {
            "R" => dial = (dial + turns).rem_euclid(100),
            "L" => dial = (dial + 100 - turns.rem_euclid(100)).rem_euclid(100),
            _ => todo!(),
        }

        if dial == 0 {
            goal += 1;
        }
    }

    goal
}

fn part2(_text: &str) -> usize {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day1.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use indoc::indoc;
//
//     const TEXT1: &str = indoc! {"
//         asdf
//     "};
//
//     #[test]
//     fn part1_given_example() {
//         assert_eq!(crate::part1(TEXT1), 0);
//     }
//
//     #[rstest::rstest]
//     #[case(TEXT1, 0)]
//     fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
//         assert_eq!(crate::part1(text), expected);
//     }
// }
