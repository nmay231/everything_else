type Result = ();

fn part1(text: &str) -> Result {}
fn part2(text: &str) -> Result {}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/dayDAY_NUMBER.txt")?;

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
