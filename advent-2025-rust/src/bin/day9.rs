fn part1(text: &str) -> usize {
    let mut red_tiles = vec![];

    for line in text.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let [a, b] = [a, b].map(|s| s.parse::<usize>().unwrap());
        red_tiles.push((a, b))
    }

    let mut max_area = 0;
    for (index, a) in red_tiles.iter().enumerate() {
        for b in red_tiles[index + 1..].iter() {
            max_area = max_area.max((1 + a.0.abs_diff(b.0)) * (1 + a.1.abs_diff(b.1)));
        }
    }

    return max_area;
}

fn part2(_text: &str) -> usize {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day9.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 50);
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
