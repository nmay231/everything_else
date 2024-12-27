use std::collections::{HashSet, VecDeque};

use advent_2024_rust::{Direc, UsizePoint};
use anyhow::Context;

type Output = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

fn part1(text: &str, grid_size: &UsizePoint) -> Output {
    let mut grid = vec![Cell::Empty; grid_size.area()];

    for (line_i, xy) in text.lines().enumerate().take(1024) {
        let (x, y) = xy
            .split_once(',')
            .with_context(|| format!("expected comma on line {}: '{}'", line_i, xy))
            .unwrap();
        let [x, y]: [usize; 2] = [x, y].map(|a| {
            a.parse()
                .with_context(|| format!("Failed to parse number on line {}: '{}'", line_i, a))
                .unwrap()
        });

        grid[UsizePoint(y, x).as_index(grid_size)] = Cell::Wall;
    }

    let goal = grid_size.sub(&UsizePoint(1, 1)).usize();
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::from([(UsizePoint(0, 0), 0)]);
    while let Some((point, steps)) = frontier.pop_front() {
        for direc in Direc::POWERS_OF_I {
            let Some(next) = point.next_point(&direc, grid_size) else {
                continue;
            };
            if next == goal {
                return 1 + steps;
            } else if grid[next.as_index(grid_size)] != Cell::Wall && !visited.contains(&next) {
                visited.insert(next);
                frontier.push_back((next, 1 + steps));
            }
        }
    }

    unreachable!("Failed to reach end point");
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day18.txt")?;

    println!("part 1 result = {:?}", part1(&text, &UsizePoint(71, 71)));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use indoc::indoc;
//
//     const TEXT1: &str = indoc! {"
//         asdf
//     "};
//
//     #[test]
//     fn part1_given_example() {
//         assert_eq!(crate::part1(TEXT1), 0);
//     }
//
//     #[rstest::rstest]
//     #[case(TEXT1, 0)]
//     fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
//         assert_eq!(crate::part1(text), expected);
//     }
// }
