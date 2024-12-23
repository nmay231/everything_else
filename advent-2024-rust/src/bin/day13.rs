use std::str::FromStr;

use advent_2024_rust::{CoinChange, UsizePoint};
use anyhow::{Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type Output = usize;

fn parse_group(button_a: &str, button_b: &str, prize: &str) -> Result<[UsizePoint; 3]> {
    lazy_static! {
        static ref RE_BUTTON: Regex = Regex::from_str(r#"Button (A|B): X\+(\d+), Y\+(\d+)"#)
            .expect("the regex should compile");
        static ref RE_PRIZE: Regex =
            Regex::from_str(r#"Prize: X=(\d+), Y=(\d+)"#).expect("the regex should compile");
    }

    let [name, x, y] = RE_BUTTON
        .captures(button_a)
        .with_context(|| format!("Couldn't parse button_a info in `{}`", button_a))?
        .extract()
        .1;
    assert_eq!(name, "A");
    let a = UsizePoint(
        x.parse().context("Couldn't parse button A's x value")?,
        y.parse().context("Couldn't parse button A's y value")?,
    );

    let [name, x, y] = RE_BUTTON
        .captures(button_b)
        .with_context(|| format!("Couldn't parse button_b info in `{}`", button_b))?
        .extract()
        .1;
    assert_eq!(name, "B");
    let b = UsizePoint(
        x.parse().context("Couldn't parse button B's x value")?,
        y.parse().context("Couldn't parse button B's y value")?,
    );

    let [x, y] = RE_PRIZE
        .captures(prize)
        .with_context(|| format!("Couldn't parse prize info in `{}`", prize))?
        .extract()
        .1;
    let prize = UsizePoint(
        x.parse().context("Couldn't parse the prize x value")?,
        y.parse().context("Couldn't parse the prize y value")?,
    );

    Ok([a, b, prize])
}

fn part1(text: &str) -> Output {
    let mut total = 0;

    // We chain("") to add a fake trailing newline
    for (index, (button_a, button_b, prize, _)) in text.lines().chain([""]).tuples().enumerate() {
        let [a, b, prize] = parse_group(button_a, button_b, prize)
            .with_context(|| {
                format!(
                    "Failed to parse group {} (around line {})",
                    index,
                    index * 4
                )
            })
            .unwrap();

        // I am confusing myself with the idea that pressing button A could be
        // better than button B three times if A has better movement. I guess
        // that could still be true. E.g. button A moves (4, 4) and B moves (3,
        // 3), then prioritizing A over B where possible works for any prize at
        // (N, N), N > 12. However, this code gives the same result as comparing
        // every possible situation. I think that was intentional in how the
        // test data was generated, but I'm too tired to articulate the
        // situation where B over A is always good so that I can add assertions
        // for that. Whatever...
        let [mut iter_xs, mut iter_ys] =
            [(a.0, b.0, prize.0), (a.1, b.1, prize.1)].map(|(a, b, prize)| {
                CoinChange::new(&[b, a], prize).map(|ba| {
                    if let [b, a] = ba[0..2] {
                        return (b, a);
                    } else {
                        panic!("duplication glitch found!");
                    }
                })
            });

        let (mut xs, mut ys) = match (iter_xs.next(), iter_ys.next()) {
            (Some(xs), Some(ys)) => (xs, ys),
            _ => continue,
        };

        while xs != ys {
            if xs.0 > ys.0 {
                let Some(new_xs) = iter_xs.next() else { break };
                xs = new_xs;
            } else {
                let Some(new_ys) = iter_ys.next() else { break };
                ys = new_ys;
            }
        }
        if xs == ys {
            // xs = (b, a)
            total += xs.0 + xs.1 * 3;
        }
    }

    total
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day13.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;

    const TEXT: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT), 480);
    }
}
