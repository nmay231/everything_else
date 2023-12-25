use std::collections::HashSet;

use advent_2023_rust::{Direc, UsizePoint};
use itertools::Itertools;

type Output = usize;

fn part1(text: &str, steps: usize) -> Output {
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    let mut grid = text.replace('\n', "").chars().collect_vec();

    let start = UsizePoint::from_index(
        grid_size,
        grid.iter()
            .enumerate()
            .find_map(|(i, char)| (*char == 'S').then_some(i))
            .unwrap(),
    );
    grid[start.as_index(grid_size)] = '.';

    let mut points = HashSet::new();
    points.insert(start);

    for step in 0..steps {
        points = points
            .iter()
            .flat_map(|point| {
                Direc::POWERS_OF_I.iter().flat_map(|direc| {
                    point.next_point(direc, grid_size).and_then(|next_point| {
                        (grid[next_point.as_index(grid_size)] == '.').then_some(next_point)
                    })
                })
            })
            .collect();

        let char = if step & 1 != steps & 1 { 'O' } else { '_' };
        for point in &points {
            grid[point.as_index(grid_size)] = char;
        }
    }

    // grid_size.debug_grid(&grid);

    grid.iter().filter(|char| char == &&'O').count()
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day21.txt")?;

    println!("part 1 result = {:?}", part1(&text, 64));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        assert!(true);
    }
}
