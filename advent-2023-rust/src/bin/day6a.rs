fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day6.txt")?;
    let mut lines = text.lines();
    let times = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|str| str.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|str| str.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let mut prod = 1;
    for (time, dist) in times.into_iter().zip(distances) {
        let start = (1..time).find(|i| i * (time - i) > dist).unwrap();
        let end = (1..time).rfind(|i| i * (time - i) > dist).unwrap();
        prod *= end - start + 1;
    }

    println!("prod = {}", prod);
    Ok(())
}
