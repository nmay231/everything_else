type Output = ();

fn part1(text: &str) -> Output {}

fn part2(_text: &str) -> Output {}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day%DAY_NUMBER%.txt")?;

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
