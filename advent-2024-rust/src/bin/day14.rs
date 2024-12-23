use std::str::FromStr;

use advent_2024_rust::IsizePoint;
use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;

type Output = usize;

fn part1(text: &str, grid_size: IsizePoint) -> Output {
    lazy_static! {
        static ref RE_ROBOT: Regex = Regex::from_str(r#"p=(\d+),(\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    };
    let quad_dividers = IsizePoint(grid_size.0 / 2, grid_size.1 / 2);

    let mut quadrants = [0; 4];
    for (index, line) in text.lines().enumerate() {
        let [x, y, dx, dy] = RE_ROBOT
            .captures(line)
            .with_context(|| format!("Error parsing line {index}: `{line}`"))
            .unwrap()
            .extract()
            .1
            .map(|str| {
                str.parse::<isize>()
                    .with_context(|| format!("Error parsing number in `{line}`"))
                    .unwrap()
            });
        let x = (x + 100 * dx).rem_euclid(grid_size.0);
        let y = (y + 100 * dy).rem_euclid(grid_size.1);

        if x == quad_dividers.0 || y == quad_dividers.1 {
            continue;
        }

        let mut which = 0;
        if x > quad_dividers.0 {
            which += 1;
        }
        if y > quad_dividers.1 {
            which += 2;
        }
        quadrants[which] += 1;
    }

    quadrants.into_iter().product()
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day14.txt")?;

    println!("part 1 result = {:?}", part1(&text, IsizePoint(101, 103)));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use advent_2024_rust::IsizePoint;
    use indoc::indoc;

    const TEXT: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT, IsizePoint(11, 7)), 12);
    }
}
