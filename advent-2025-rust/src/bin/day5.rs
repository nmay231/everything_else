fn part1(text: &str) -> usize {
    // NOTE: I've often needed to break a boolean into a three stage state or
    // more, so I just default to a number now.
    let mut parse_state = 0;
    let mut ranges = vec![];
    let mut ids = vec![];

    for line in text.lines() {
        if parse_state == 0 && line == "" {
            parse_state = 1;
            // NOTE: The sorting and condensing of the ranges could go here and
            // then the ids could be processed as they are parsed, but this is
            // clearer.
            continue;
        }
        if parse_state == 0 {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            ranges.push(start..=end);
        } else {
            assert_eq!(parse_state, 1);
            ids.push(line.parse::<usize>().unwrap());
        }
    }

    ranges.sort_by_key(|range| *range.start());

    let mut next = 1;
    while next < ranges.len() {
        let prev = next - 1;
        if ranges[prev].end() >= &ranges[next].start().saturating_sub(1) {
            let start = *ranges[prev].start();
            let end = *ranges[prev].end().max(ranges[next].end());
            ranges.splice(prev..=next, [start..=end].into_iter());
        } else {
            next += 1;
        }
    }

    // TODO: Binary search if too slow (otherwise, what was the point of sorting
    // and merging the ranges... Not like I did that out of instinct...)

    let mut total = 0;
    'ids: for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                total += 1;
                continue 'ids;
            }
        }
    }

    return total;
}

fn part2(_text: &str) -> usize {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day5.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 3);
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
