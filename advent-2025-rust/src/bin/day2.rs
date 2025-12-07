fn count_patterns_within_range(start: &str, end: &str) -> usize {
    let min_possible = start.parse::<usize>().unwrap();
    let max_possible = end.parse::<usize>().unwrap();

    let mut pow10 = (10_usize).pow(start.len().div_ceil(2) as u32);

    let mut min_pattern = if start.len() % 2 == 1 {
        pow10 / 10
    } else {
        let mid = start.len() / 2;
        start[..mid].parse::<usize>().unwrap()
    };

    let mut goal = 0;

    loop {
        let to_check = min_pattern + pow10 * min_pattern;
        if to_check < min_possible {
            min_pattern += 1;
            continue;
        } else if to_check > max_possible {
            return goal;
        }
        goal += to_check;
        min_pattern += 1;

        if min_pattern >= pow10 {
            pow10 *= 100;
        }
    }
}

fn part1(text: &str) -> usize {
    let mut goal = 0;

    for range in text.trim().split(',') {
        let (start, end) = range.split_once('-').expect("Invalid range format");
        goal += count_patterns_within_range(start, end);
    }

    goal
}

fn part2(_text: &str) -> usize {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day2.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 1227775554);
    }

    //     #[rstest::rstest]
    //     #[case(TEXT1, 0)]
    //     fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //         assert_eq!(crate::part1(text), expected);
    //     }
}
