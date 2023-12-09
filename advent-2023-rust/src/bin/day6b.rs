fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day6.txt")?;
    let mut lines = text.lines();
    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let dist = lines
        .next()
        .unwrap()
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let start = (1..time).find(|i| i * (time - i) > dist).unwrap();
    let end = (1..time).rfind(|i| i * (time - i) > dist).unwrap();
    let prod = end - start + 1;
    println!("prod = {}", prod);
    Ok(())
}
