use std::collections::HashMap;
use std::ops::Deref;

use itertools::Itertools;
use num_integer::{ExtendedGcd, Integer};

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

    let seeds = map
        .keys()
        .map(Deref::deref)
        .filter(|str| str.ends_with('A'))
        .collect_vec();

    let mut offset_cycle_pairs = vec![];
    for mut seed in seeds {
        let mut visited = vec![(seed, 0)];
        for (turn_index, turn) in turns.iter().enumerate().cycle() {
            let branch = map.get(seed).unwrap();
            seed = if *turn { branch.0 } else { branch.1 };

            if let Some(index) = visited.iter().position(|x| x == &(seed, turn_index)) {
                assert_eq!((visited.len() - index) % turns.len(), 0);
                offset_cycle_pairs.push((
                    index as isize,
                    ((visited.len() - index) / turns.len()) as isize,
                ));
                break;
            }
            visited.push((seed, turn_index));
        }
    }
    // let offset_cycle_pairs = vec![
    //     (3, (20803 / turns.len()) as isize),
    //     (2, (13771 / turns.len()) as isize),
    //     (6, (17287 / turns.len()) as isize),
    //     (2, (19631 / turns.len()) as isize),
    //     (3, (23147 / turns.len()) as isize),
    //     (3, (17873 / turns.len()) as isize),
    // ];

    // TODO: Technically incorrect answer (in general, that is).
    let wrong_way = turns.len() as isize
        * offset_cycle_pairs
            .iter()
            .fold(1, |acc, (_offset, cycle_length)| acc.lcm(cycle_length));
    println!("The incorrectly correct answer: {}", wrong_way);

    // TODO: However, I can't even solve it CORRECTLY. And I'm so done right now...
    // https://brilliant.org/wiki/chinese-remainder-theorem/
    for (a, b) in offset_cycle_pairs.iter().tuple_combinations() {
        assert_eq!(a.1.gcd(&b.1), 1);
    }

    let mut x = 0;
    let cap_n = offset_cycle_pairs.iter().fold(1, |acc, (_, n_i)| acc * n_i);
    for (a_i, n_i) in offset_cycle_pairs {
        let y_i = cap_n / n_i;
        let ExtendedGcd { x: z_i, .. } = y_i.extended_gcd(&n_i);
        // let z_i = z_i.rem_euclid(n_i);
        assert_eq!((z_i * y_i).rem_euclid(n_i), 1);

        x += a_i * y_i * z_i;
        println!("{:?}", (a_i, y_i, z_i, a_i * y_i * z_i));
    }
    let x = x.rem_euclid(cap_n);
    assert!(x >= 0, "{x}");
    return x as usize * turns.len();
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
