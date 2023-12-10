use std::collections::HashMap;
use std::ops::Deref;

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

fn gcd(a: Result, b: Result) -> Result {
    let mut max = a;
    let mut min = b;
    if min > max {
        (min, max) = (max, min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        (max, min) = (min, res);
    }
}

// Unfortunately, I peeked at the subreddit so the solution was a bit spoiled for me, even though it isn't the right solution given the problem in general.
// I don't know how to feel about it since I can't solve on my own anymore. I'm not gonna look at the subreddit anymore until I've solved the whole advent, or at least as much as I'm willing to solve.
// Based on a user's comment in the following thread, I at least know the way I would solve it is to use the Chinese remainder theorem.
// I vaguely remember it, but I never learned it in full. Here's my chance I guess...
// https://www.reddit.com/r/adventofcode/comments/18dfpub/2023_day_8_part_2_why_is_spoiler_correct/
fn part2(text: &str) -> Result {
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

    return map
        .keys()
        .map(Deref::deref)
        .filter(|pos| pos.ends_with("A"))
        .map(|start| {
            let mut pos = start;
            let mut visited = HashMap::new();
            visited.insert((0, pos), 0);

            for (steps, (i, left)) in turns.iter().enumerate().cycle().enumerate() {
                let pair = map.get(pos).unwrap();
                pos = if *left { pair.0 } else { pair.1 };

                if pos.ends_with("Z") {
                    println!("end pos={pos}");
                    return 0;
                }
                let key = (i + 1, pos);
                if visited.contains_key(&key) {
                    println!("{visited:?}; {pos}");
                    panic!();
                }
                visited.insert(key, steps + 1);

                if pos == start {
                    // TODO: This is not always true, I need the visited array, but also,
                    return steps + 1;
                }
            }
            return 1;
        })
        .map(|steps| {
            // println!("{steps}");
            steps
        })
        .fold(1 as usize, |acc, steps| steps / gcd(acc, steps) * acc);
    // .fold(1 as usize, |acc, (modulus, offset)| {
    //     steps / gcd(acc, steps) * acc
    // });
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
