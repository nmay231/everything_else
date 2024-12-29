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

fn generate_distances(text: &str) -> (Point<usize>, HashMap<Point<usize>, usize>) {
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

    return (*grid_size, distances);
}

fn part1(text: &str, floor: usize) -> Output {
    let floor = floor + 2; // It takes 2 picoseconds to glitch through a wall
    let (ref grid_size, distances) = generate_distances(text);

    let mut total = 0;
    for (start, start_steps) in distances.iter() {
        for direc in Direc::POWERS_OF_I {
            let Some(pos) = start.next_point_steps(&2, &direc, grid_size) else {
                continue;
            };
            match distances.get(&pos) {
                Some(dest_steps) if *dest_steps >= start_steps + floor => total += 1,
                _ => (),
            }
        }
    }
    total
}

fn part2(text: &str, floor: usize) -> Output {
    let (ref grid_size, distances) = generate_distances(text);

    let mut total = 0;
    for (start, start_steps) in distances.iter() {
        for distance_left in -20..=20_i32 {
            let (glitch_steps, horizontal) = if distance_left < 0 {
                ((-distance_left) as usize, Direc::West)
            } else {
                (distance_left as usize, Direc::East)
            };

            if glitch_steps == 20 {
                let mut pos = start.clone();
                for x in 1..=glitch_steps {
                    pos = match pos.next_point(&horizontal, grid_size) {
                        Some(pos) => pos,
                        None => continue,
                    };

                    if let Some(dest_steps) = distances.get(&pos) {
                        if *dest_steps >= start_steps + floor + x {
                            total += 1;
                        }
                    }
                }
            } else {
                let Some(pos) = start.next_point_steps(&glitch_steps, &horizontal, grid_size)
                else {
                    continue;
                };

                for vertical in [Direc::North, Direc::South] {
                    let mut pos = pos;
                    for x in 1..=(20 - glitch_steps) {
                        pos = match pos.next_point(&vertical, grid_size) {
                            Some(pos) => pos,
                            None => continue,
                        };

                        if let Some(dest_steps) = distances.get(&pos) {
                            if *dest_steps >= start_steps + floor + x + glitch_steps {
                                total += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    total
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day20.txt")?;

    println!("part 1 result = {:?}", part1(&text, 100));
    println!("part 2 result = {:?}", part2(&text, 100));

    Ok(())
}

#[cfg(test)]
mod tests {

    use indoc::indoc;
    use itertools::Itertools;

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
        for floor in 1..=70 {
            let expected = amounts.iter().filter(|x| **x >= floor).count();
            assert_eq!(crate::part1(TEXT1, floor), expected, "floor={}", floor);
        }
    }

    #[test]
    fn part2_given_example() {
        // (Benefit, Count)
        let glitches = [
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ];
        // Note: The above list only lists 50+
        for floor in (50..=80).rev() {
            let expected = glitches
                .into_iter()
                .filter_map(|(picoseconds_saved, count)| {
                    (picoseconds_saved >= floor).then_some(count)
                })
                .sum();
            assert_eq!(crate::part2(TEXT1, floor), expected, "floor={}", floor);
        }
    }
}
