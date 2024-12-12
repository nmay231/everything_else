use std::collections::{HashMap, HashSet};

use advent_2024_rust::UsizePoint;
use itertools::Itertools;

type Output = usize;

fn part1(text: &str) -> Output {
    let grid_size = &UsizePoint(text.find('\n').unwrap(), text.lines().count());
    let grid = text.trim().chars().filter(|c| c != &'\n').collect_vec();
    let mut antennas = HashMap::<char, Vec<UsizePoint>>::new();

    for (i, c) in grid.iter().enumerate() {
        if c == &'.' {
            continue;
        }
        antennas
            .entry(*c)
            .or_default()
            .push(UsizePoint::from_index(grid_size, i));
    }

    let mut antinodes = HashSet::new();
    for positions in antennas.into_values() {
        for vec in positions.into_iter().combinations(2) {
            let a = vec[0];
            let b = vec[1];
            let ab = &b.sub(&a);

            let a = a.isize().sub(ab);
            if a.within_grid(grid_size) {
                antinodes.insert(a);
            }

            let b = b.isize().add(ab);
            if b.within_grid(grid_size) {
                antinodes.insert(b);
            }
        }
    }

    return antinodes.len();
}

fn part2(text: &str) -> Output {
    let grid_size = &UsizePoint(text.find('\n').unwrap(), text.lines().count());
    let grid = text.trim().chars().filter(|c| c != &'\n').collect_vec();
    let mut antennas = HashMap::<char, Vec<UsizePoint>>::new();

    for (i, c) in grid.iter().enumerate() {
        if c == &'.' {
            continue;
        }
        antennas
            .entry(*c)
            .or_default()
            .push(UsizePoint::from_index(grid_size, i));
    }

    let mut antinodes = HashSet::new();
    for positions in antennas.into_values() {
        for vec in positions.into_iter().combinations(2) {
            let mut a = vec[0].isize();
            let mut b = vec[1].isize();
            let ab = &b.sub(&a);

            // Yes, we do count the antennas as antinodes now.
            while a.within_grid(grid_size) {
                antinodes.insert(a);
                a = a.sub(ab);
            }

            while b.within_grid(grid_size) {
                antinodes.insert(b);
                b = b.add(ab);
            }
        }
    }

    return antinodes.len();
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day8.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use indoc::indoc;

    const TEXT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT), 14)
    }

    #[test]
    fn part2_given_example() {
        assert_eq!(part2(TEXT), 34)
    }
}
