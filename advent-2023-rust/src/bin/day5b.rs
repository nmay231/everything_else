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
        .collect::<Vec<_>>()
        .windows(2)
        .step_by(2)
        .map(|window| {
            if let [end, start] = window {
                return *start..*end;
            } else {
                panic!("Non-pair of windows");
            }
        })
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
            .flat_map(|seeds| {
                let mut seeds = seeds.to_owned();
                let mut matches = vec![];
                for (source, dest) in maps.iter() {
                    if source.start <= seeds.start && seeds.end <= source.end {
                        // Fully surrounds it
                        matches.push(
                            dest.start + seeds.start - source.start
                                ..dest.start + seeds.end - source.start,
                        );
                        break;
                    } else if source.contains(&seeds.start) {
                        // Only contains the start
                        matches.push(dest.start + seeds.start - source.start..dest.end);
                        seeds = dest.end..dest.start + seeds.end - source.start;
                    } else if seeds.end == 0 || dest.start + seeds.start < source.start {
                        println!("{:?},{:?},{:?},{:?}", seeds, source, dest, matches);
                        return matches;
                    } else if source.contains(&(seeds.end - 1)) {
                        // Only contains the end
                        // println!("{source:?}, {dest:?}, {seeds:?}");
                        // println!("{}", dest.start + seeds.end - source.start);
                        matches.push(dest.start..dest.start + seeds.end - source.start);
                        seeds = dest.start + seeds.start - source.start..dest.start;
                    }
                }
                if seeds.len() != 0 {
                    matches.push(seeds);
                }
                return matches;
            })
            .collect()
    }

    // 35355313 too high
    println!(
        "lowest final value = {}",
        current.iter().map(|range| range.start).min().unwrap()
    );

    return Ok(());
}
