use std::collections::HashMap;

use advent_2024_rust::{Direc, Point};
use itertools::Itertools;

type Output = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Goal,
}

fn part1(text: &str, min_picosecs: usize) -> Output {
    let min_picosecs = min_picosecs + 2; // It takes 2 picoseconds to glitch through a wall
    let text = text.trim();
    let grid_size = &Point::<usize>::new_xy(text.find('\n').unwrap(), text.lines().count());
    let mut player = None;
    let grid = text
        .chars()
        .filter(|c| c != &'\n')
        .enumerate()
        .map(|(index, char)| match char {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'S' => {
                assert!(player.is_none(), "found more than one player");
                player = Some(Point::from_index(grid_size, index));
                Cell::Empty
            }
            'E' => Cell::Goal,
            _ => unreachable!("Unexpected char '{}' at index {}", char, index),
        })
        .collect_vec();

    let player = player.expect("A player wasn't found");

    let mut distances = HashMap::new();
    let mut frontier = vec![(player, 0)];
    let surrounding_walls = "Passed over the outer wall unexpectedly";

    while let Some((pos, steps)) = frontier.pop() {
        if distances.get(&pos).is_some() {
            continue;
        }
        distances.insert(pos, steps);

        for direc in Direc::POWERS_OF_I {
            let pos = pos.next_point(&direc, grid_size).expect(surrounding_walls);
            match &grid[pos.as_index(grid_size)] {
                Cell::Wall => continue,
                Cell::Empty => frontier.push((pos, steps + 1)),
                Cell::Goal => {
                    distances.entry(pos).or_insert(steps + 1);
                }
            }
        }
    }

    assert_eq!(
        distances.len(),
        // 2 extra for the start and end
        2 + text.chars().filter(|c| c == &'.').count()
    );

    let mut total = 0;
    for (start, start_steps) in distances.iter() {
        for direc in Direc::POWERS_OF_I {
            let Some(pos) = start.next_point_steps(&2, &direc, grid_size) else {
                continue;
            };
            match distances.get(&pos) {
                Some(dest_steps) if *dest_steps >= start_steps + min_picosecs => total += 1,
                _ => (),
            }
        }
    }
    total
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day20.txt")?;

    println!("part 1 result = {:?}", part1(&text, 100));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {

    use indoc::indoc;
    use itertools::Itertools;

    use crate::part1;

    const TEXT1: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn part1_given_example() {
        let amounts = [
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]
        .into_iter()
        .flat_map(|(picos_saves, quantity)| std::iter::repeat_n(picos_saves, quantity))
        .collect_vec();
        for floor in 1..=75 {
            let expected = amounts.iter().filter(|x| **x >= floor).count();
            assert_eq!(part1(TEXT1, floor), expected, "floor={}", floor);
        }
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
