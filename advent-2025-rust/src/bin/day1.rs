const DIAL_SIZE: usize = 100;

fn part1(text: &str) -> usize {
    let mut dial = 50usize;
    let mut goal = 0;

    for line in text.lines() {
        let turns = line[1..].parse::<usize>().expect("Not a number!");
        match &line[..1] {
            "R" => dial = (dial + turns).rem_euclid(DIAL_SIZE),
            "L" => dial = (dial + DIAL_SIZE - turns.rem_euclid(DIAL_SIZE)).rem_euclid(DIAL_SIZE),
            _ => todo!(),
        }

        if dial == 0 {
            goal += 1;
        }
    }

    goal
}

fn part2(text: &str) -> usize {
    let mut dial = 50usize;
    let mut goal = 0;

    for line in text.lines() {
        let turns: usize = line[1..].parse::<usize>().expect("Not a number!");
        match &line[..1] {
            "R" => {
                dial += turns;
                goal += dial / DIAL_SIZE;
                dial = dial.rem_euclid(DIAL_SIZE)
            }
            "L" => {
                dial = (100 - dial).rem_euclid(DIAL_SIZE);
                dial += turns;
                goal += dial / DIAL_SIZE;
                dial = dial.rem_euclid(DIAL_SIZE);
                dial = (100 - dial).rem_euclid(DIAL_SIZE);
            }
            _ => todo!(),
        }
    }

    goal
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day1.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};
    // 50 - 68 = 100 - 18 = 82 (+1)
    // 82 - 30 = 52
    // 52 + 48 = 100 => 0 (+1)
    // 95
    // 95 + 60 = 55 (+1)
    // 55 - 55 = 0 (+1)
    // -1 => 99
    // 99 - 99 = 0 (+1)
    // 14
    // 14 - 82 = 14 + 18 = 32 (+1)

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 3);
    }

    #[test]
    fn part2_given_example() {
        assert_eq!(crate::part2(TEXT1), 6);
    }

    // Heh...
    const ALRIGHT: &str = indoc! {"
        R50
        R100
        R200
        R201
        R199
        R1
    "};

    const ALL_LEFT: &str = indoc! {"
        L50
        L100
        L200
        L201
        L199
        L1
    "};

    #[rstest::rstest]
    #[case(ALRIGHT, 8)]
    #[case(ALL_LEFT, 8)]
    fn part2_other_examples(#[case] text: &str, #[case] expected: usize) {
        assert_eq!(crate::part2(text), expected);
    }
}
