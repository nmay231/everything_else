use std::collections::HashSet;

use advent_2024_rust::{Direc, UsizePoint};

type Output = usize;

fn part1(text: &str) -> Output {
    let text = text.trim();
    let grid_size = &UsizePoint(text.find('\n').unwrap(), text.lines().count());
    let grid = text.chars().filter(|c| c != &'\n').collect::<Vec<_>>();
    let digits = "0123456789".chars().collect::<Vec<_>>();

    let mut total = 0;
    for (index, character) in grid.iter().enumerate() {
        if character != &'0' {
            continue;
        }

        let pos = UsizePoint::from_index(grid_size, index);
        let mut frontier = vec![(pos, 1_usize)];
        let mut peaks = HashSet::new();

        while let Some((pos, next_digit)) = frontier.pop() {
            for direc in Direc::POWERS_OF_I {
                let pos = match pos.next_point(&direc, grid_size) {
                    Some(pos) => pos,
                    None => continue,
                };

                if grid[pos.as_index(grid_size)] != digits[next_digit] {
                    continue;
                }

                if next_digit == 9 {
                    peaks.insert(pos);
                    continue;
                }
                frontier.push((pos, next_digit + 1));
            }
        }

        total += peaks.len();
    }

    total
}

fn part2(text: &str) -> Output {
    let text = text.trim();
    let grid_size = &UsizePoint(text.find('\n').unwrap(), text.lines().count());
    let grid = text.chars().filter(|c| c != &'\n').collect::<Vec<_>>();
    let digits = "0123456789".chars().collect::<Vec<_>>();

    let mut total = 0;
    for (index, character) in grid.iter().enumerate() {
        if character != &'0' {
            continue;
        }

        let pos = UsizePoint::from_index(grid_size, index);
        let mut frontier = vec![(pos, 1_usize)];

        while let Some((pos, next_digit)) = frontier.pop() {
            for direc in Direc::POWERS_OF_I {
                let pos = match pos.next_point(&direc, grid_size) {
                    Some(pos) => pos,
                    None => continue,
                };

                if grid[pos.as_index(grid_size)] != digits[next_digit] {
                    continue;
                }

                if next_digit == 9 {
                    total += 1;
                    continue;
                }
                frontier.push((pos, next_digit + 1));
            }
        }
    }

    total
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day10.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT1), 36);
    }

    #[test]
    fn part2_given_example() {
        assert_eq!(part2(TEXT1), 81);
    }
}
