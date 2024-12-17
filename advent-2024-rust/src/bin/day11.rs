type Output = usize;

fn part1(text: &str) -> Output {
    let mut stones = text
        .trim()
        .split(' ')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|stone| match format!("{}", stone).as_str() {
                "0" => vec![1],
                text if text.len() % 2 == 0 => {
                    let (a, b) = text.split_at(text.len() / 2);
                    vec![a.parse().unwrap(), b.parse().unwrap()]
                }
                _ => vec![stone * 2024],
            })
            .collect();
    }

    stones.len()
}

fn part2(text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day11.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;

    const TEXT: &str = indoc! {"
        125 17
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT), 55312);
    }
}
