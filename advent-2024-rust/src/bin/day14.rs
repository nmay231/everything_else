use std::io::{self, Write};
use std::str::FromStr;

use advent_2024_rust::{IsizePoint, UsizePoint};
use anyhow::Context;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type Output = usize;

fn parse_line(line: &str, line_index: usize) -> [isize; 4] {
    lazy_static! {
        static ref RE_ROBOT: Regex = Regex::from_str(r#"p=(\d+),(\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    };
    RE_ROBOT
        .captures(line)
        .with_context(|| format!("Error parsing line {}: `{}`", line_index, line))
        .unwrap()
        .extract()
        .1
        .map(|str| {
            str.parse::<isize>()
                .with_context(|| format!("Error parsing number in `{}`", line))
                .unwrap()
        })
}

fn part1(text: &str, grid_size: IsizePoint) -> Output {
    let quad_dividers = IsizePoint(grid_size.0 / 2, grid_size.1 / 2);

    let mut quadrants = [0; 4];
    for (index, line) in text.lines().enumerate() {
        let [x, y, dx, dy] = parse_line(line, index);
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

fn _wait() {
    io::stdout().flush().unwrap();
    let buf = &mut String::new();
    io::stdin().read_line(buf).unwrap();
}
fn _print_grid(grid: &[char], grid_size: &UsizePoint) {
    let grid = grid
        .chunks(grid_size.0)
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .join("\n");
    println!("\n\n{}", grid);
}

/// I was originally going to manually scan through large 10x10 renderings of
/// each grid, but I didn't find it because some sort of horizontal smearing is
/// occurring especially with larger grid. I don't have the free-time to debug
/// this, so here is the mess with all of the debugging code in it. In the end I
/// ended up using u/beebeep's suggestion in
/// https://www.reddit.com/r/adventofcode/comments/1hdw2m1/2024_day_14_part_2/.
/// It's neat idea to use compression size as a way to find a normal picture of
/// the christmas tree. I do like the overall idea of day14 part2 though; it's
/// kinda funny turning expectations on its head by requiring the "dumb"/simple
/// solution over any optimizations.
fn part2(text: &str) -> () {
    let grid_size = &UsizePoint(101, 103);
    let empty_grid = std::iter::repeat_n([0_u8, 0, 0], grid_size.area()).collect_vec();
    let white_color = [255_u8, 255, 255];

    let mut robots = text
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let [x, y, dx, dy] = parse_line(line, line_index);
            (UsizePoint(x as usize, y as usize), IsizePoint(dx, dy))
        })
        .collect_vec();

    let meta_size = &UsizePoint(1, 1).mul(10);
    let large_grid_size = grid_size.mul(meta_size.0);

    for meta_i in 0.. {
        if (meta_i + 1) * meta_size.area() < 7916 {
            for i in 0..meta_size.area() {
                // let offset = &UsizePoint::from_index(meta_size, i).piecewise_mul(grid_size);
                robots = robots
                    .into_iter()
                    .map(|(pos, delta)| {
                        // ten_by_ten[pos.add(offset).as_index(&large_grid_size)] = white_color;
                        let pos = UsizePoint(
                            (pos.0 as isize + delta.0).rem_euclid(grid_size.0 as isize) as usize,
                            (pos.1 as isize + delta.1).rem_euclid(grid_size.1 as isize) as usize,
                        );
                        (pos, delta)
                    })
                    .collect();
            }
            continue;
        }
        let mut ten_by_ten = empty_grid.repeat(meta_size.area());
        for i in 0..meta_size.area() {
            let offset = &UsizePoint::from_index(meta_size, i).piecewise_mul(grid_size);
            robots = robots
                .into_iter()
                .map(|(pos, delta)| {
                    ten_by_ten[pos.add(offset).as_index(
                        &large_grid_size, // I thought maybe there was an off-by-one error with
                                          // the grid size, but that doesn't fix the error.
                                          // &large_grid_size.add(&UsizePoint(0, 1))
                    )] = white_color;
                    let pos = UsizePoint(
                        (pos.0 as isize + delta.0).rem_euclid(grid_size.0 as isize) as usize,
                        (pos.1 as isize + delta.1).rem_euclid(grid_size.1 as isize) as usize,
                    );
                    (pos, delta)
                })
                .collect();
        }
        image::save_buffer(
            &format!(
                "./sample.png",
                // meta_size.0,
                // meta_size.1,
                // meta_i * meta_size.area(),
                // (meta_i + 1) * meta_size.area() - 1
            ),
            &ten_by_ten.as_flattened(),
            large_grid_size.0 as u32,
            large_grid_size.1 as u32,
            image::ExtendedColorType::Rgb8,
        )
        .unwrap();
        println!(
            "iterations: {}..={}",
            meta_i * meta_size.area(),
            (meta_i + 1) * meta_size.area() - 1
        );
        break;
    }
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day14.txt")?;

    println!("part 1 result = {:?}", part1(&text, IsizePoint(101, 103)));
    part2(&text);

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
