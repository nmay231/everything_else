fn max_joltage(bank: &str, to_turn_on: usize) -> usize {
    if to_turn_on == 0 {
        return 0;
    }
    let (index, first_char) = bank[0..bank.len() + 1 - to_turn_on]
        .chars()
        .enumerate()
        .max_by_key(|(index, ch)| (*ch, bank.len() - index))
        .expect("there to be at least two batteries per bank");
    if to_turn_on < 2 {
        return first_char.to_digit(10).unwrap() as usize;
    }
    let max = format!(
        "{}{}",
        first_char,
        max_joltage(&bank[index + 1..], to_turn_on - 1)
    );

    return max.parse::<usize>().unwrap();
}

fn part1(text: &str) -> usize {
    let mut goal = 0;

    for line in text.lines() {
        goal += max_joltage(line, 2);
    }

    goal
}

fn part2(text: &str) -> usize {
    let mut goal = 0;

    for line in text.lines() {
        goal += max_joltage(line, 12);
    }

    goal
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

    #[test]
    fn part2_given_example() {
        assert_eq!(crate::part2(TEXT1), 3121910778619);
    }
}
