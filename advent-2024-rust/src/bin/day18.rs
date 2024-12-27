#![feature(assert_matches)]
use std::assert_matches::assert_matches;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

use advent_2024_rust::{Direc, UsizePoint};
use anyhow::Context;
use itertools::Itertools;

type Output = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

fn part1(text: &str, grid_size: &UsizePoint, n_glitches: usize) -> Output {
    let mut grid = vec![Cell::Empty; grid_size.area()];

    for (line_i, xy) in text.lines().enumerate().take(n_glitches) {
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

fn part2(text: &str, grid_size: &UsizePoint) -> String {
    let glitches = text
        .lines()
        .enumerate()
        .map(|(line_i, xy)| {
            let (x, y) = xy
                .split_once(',')
                .with_context(|| format!("expected comma on line {}: '{}'", line_i, xy))
                .unwrap();
            let [x, y]: [usize; 2] = [x, y].map(|a| {
                a.parse()
                    .with_context(|| format!("Failed to parse number on line {}: '{}'", line_i, a))
                    .unwrap()
            });

            UsizePoint(y, x)
        })
        .collect_vec();

    // Yes, this is the lazy way since I could technically track when the
    // first diagonally/orthogonally continuous path of walls is made when
    // the glitches are placed, but... This also works
    let indexes = (0..glitches.len()).collect_vec();
    let index = indexes.binary_search_by(|index| {
        let mut grid = vec![Cell::Empty; grid_size.area()];
        for point in glitches.iter().take(*index) {
            grid[point.as_index(grid_size)] = Cell::Wall;
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
                    return Ordering::Less;
                } else if grid[next.as_index(grid_size)] != Cell::Wall && !visited.contains(&next) {
                    visited.insert(next);
                    frontier.push_back((next, 1 + steps));
                }
            }
        }

        return Ordering::Greater;
    });

    assert_matches!(index, Err(_)); // Using assert matches to get debug printing if false
    let final_blocker = glitches[index.unwrap_err() - 1];
    return format!("{},{}", final_blocker.1, final_blocker.0);
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day18.txt")?;

    println!(
        "part 1 result = {:?}",
        part1(&text, &UsizePoint(71, 71), 3010)
    );
    println!("part 2 result = {:?}", part2(&text, &UsizePoint(71, 71)));

    Ok(())
}

#[cfg(test)]
mod tests {
    use advent_2024_rust::UsizePoint;
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn part1_given_examples() {
        assert_eq!(crate::part1(TEXT1, &UsizePoint(7, 7), 12), 22);
    }

    #[test]
    fn part2_given_examples() {
        assert_eq!(crate::part2(TEXT1, &UsizePoint(7, 7)), "6,1");
    }
}
