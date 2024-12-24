use std::collections::HashMap;
use std::usize;

use advent_2024_rust::{Direc, UsizePoint};
use itertools::Itertools;

type Output = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Goal,
}

fn part1(text: &str) -> Output {
    let grid_size = &UsizePoint(text.find('\n').unwrap(), text.lines().count());
    let mut player = None;
    let grid = text
        .chars()
        .filter(|c| c != &'\n')
        .enumerate()
        .map(|(i, c)| match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'E' => Cell::Goal,
            'S' => {
                assert!(player.is_none());
                player = Some(UsizePoint::from_index(grid_size, i));
                Cell::Empty
            }
            _ => panic!("Unexpected character '{}' at index {}", c, i),
        })
        .collect_vec();
    let player = player.expect("Player start wasn't found");

    // Sanity checks
    assert_eq!(grid_size.area(), grid.len());
    assert!(
        grid.iter().find(|cell| cell == &&Cell::Goal).is_some(),
        "Didn't find a goal in the grid"
    );
    assert_eq!(&grid[player.as_index(grid_size)], &Cell::Empty);

    let mut global_visited = HashMap::new();
    let mut current_depth = vec![(player, Direc::East, 0)];
    let mut next_depth = vec![];
    let boundaries = "Expected a wall before before leaving the bounds of the map";

    let mut min_score = None;
    for _max_expected_iterations in 0..grid_size.area() {
        while let Some((mut pos, ref direc, mut score)) = current_depth.pop() {
            let sides = [direc.rotate(1), direc.rotate(-1)];

            while &grid[pos.as_index(grid_size)] != &Cell::Wall {
                if &grid[pos.as_index(grid_size)] == &Cell::Goal {
                    min_score = Some(min_score.unwrap_or(usize::MAX).min(score));
                }

                if let Some(prev_cost) = global_visited.get(&(pos, *direc)) {
                    if *prev_cost <= score {
                        break;
                    }
                }
                global_visited.insert((pos, *direc), score);

                for side in &sides {
                    let adjacent = pos.next_point(side, grid_size).expect(boundaries);
                    if grid[adjacent.as_index(grid_size)] != Cell::Wall {
                        next_depth.push((adjacent, *side, score + 1001));
                    }
                }

                pos = pos.next_point(direc, grid_size).expect(boundaries);
                score += 1;
            }
        }

        if let Some(min) = min_score {
            return min;
        }
        assert!(
            next_depth.len() > 0,
            "Ran out of new paths before finding goal"
        );
        assert!(
            next_depth.len() < grid_size.area(),
            "Missed a place of exponential growth?"
        );

        current_depth = next_depth;
        next_depth = vec![];
    }

    unreachable!("Took longer than expected");
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day16.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    const TEXT2: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[rstest::rstest]
    #[case(TEXT1, 7036)]
    #[case(TEXT2, 11048)]
    fn part1_given_example(#[case] text: &str, #[case] expected: usize) {
        assert_eq!(part1(text), expected);
    }
}
