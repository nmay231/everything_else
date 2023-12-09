use std::collections::HashMap;

type Result = usize;

fn part1(text: &str) -> Result {
    let mut lines = text.lines();
    let turns = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| char == 'L')
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    lines.next(); // Skip gap
    for line in lines {
        map.insert(&line[..3], (&line[7..10], &line[12..15]));
    }

    let mut pos = "AAA";
    for (steps, left) in turns.iter().cycle().enumerate() {
        let pair = map.get(pos).unwrap();
        pos = if *left { pair.0 } else { pair.1 };
        if pos == "ZZZ" {
            return steps + 1;
        }
    }
    return 0;
}

fn part2(text: &str) -> Result {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day8.txt")?;

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
