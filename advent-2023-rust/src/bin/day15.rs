type Output = usize;

fn part1(text: &str) -> Output {
    let mut sum = 0;
    for step in text.trim().split(',') {
        sum += step
            .as_bytes()
            .iter()
            .fold(0, |acc, byte| (acc + *byte as usize) * 17 % 256);
    }

    sum
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day15.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        assert!(true);
    }
}
