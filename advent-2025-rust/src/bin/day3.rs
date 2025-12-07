fn part1(text: &str) -> usize {
    let mut goal = 0;

    for line in text.lines() {
        let (index, first_char) = line[..line.len() - 1]
            .chars()
            .enumerate()
            .max_by_key(|(index, ch)| (*ch, line.len() - index))
            .expect("there to be at least two batteries per bank");
        let second_char = line[index + 1..].chars().max().unwrap();

        let s = first_char.to_digit(10).unwrap() * 10 + second_char.to_digit(10).unwrap();
        goal += s as usize;
    }

    goal
}

fn part2(_text: &str) -> usize {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day3.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 357);
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
