fn parse_int(x: &str) -> Option<usize> {
    if x.len() == 0 {
        None
    } else {
        Some(x.parse().unwrap())
    }
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day5.txt")?;
    let mut lines = text.lines().peekable();
    let mut current = lines.next().unwrap()[7..]
        .split(" ")
        .filter_map(parse_int)
        .collect::<Vec<_>>();

    lines.next(); // Skip empty line

    while lines.peek() != None {
        lines.next(); // Skip header

        let mut maps = vec![];
        loop {
            let triple = match lines.next() {
                None => break,
                Some(x) => x.split(" ").filter_map(parse_int).collect::<Vec<_>>(),
            };

            if triple.len() == 0 {
                break;
            }

            if let [dest_start, source_start, count] = triple[..3] {
                maps.push((
                    (source_start..source_start + count),
                    (dest_start..dest_start + count),
                ))
            } else {
                panic!("Expected a number triple");
            }
        }

        current = current
            .iter()
            .map(|n| {
                for (source, dest) in maps.iter() {
                    if source.contains(n) {
                        return dest.start + n - source.start;
                    }
                }
                return *n;
            })
            .collect()
    }

    println!("lowest final value = {}", current.iter().min().unwrap());

    return Ok(());
}
