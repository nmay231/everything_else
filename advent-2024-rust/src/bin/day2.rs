type Output = usize;

fn part1(text: &str) -> Output {
    let mut count = 0;
    for line in text.lines() {
        let levels: Result<Vec<isize>, _> =
            line.split_ascii_whitespace().map(|x| x.parse()).collect();
        let levels = levels.expect("Err");
        let diff = levels
            .iter()
            .skip(1)
            .zip(&levels)
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();

        if !(diff.iter().all(|x| x > &0) || diff.iter().all(|x| x < &0)) {
            continue;
        } else if !diff.iter().all(|x| x.abs_diff(0) <= 3) {
            continue;
        }

        count += 1;
    }

    return count;
}

fn part2(_text: &str) -> Output {
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
    #[test]
    fn sample() {
        assert!(true);
    }
}
