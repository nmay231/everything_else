use std::collections::HashMap;

use advent_2024_rust::{Direc, DisjointSetWithCount, UsizePoint};

type Output = usize;

fn part1(text: &str) -> Output {
    let grid_size = &UsizePoint(text.find('\n').unwrap(), text.lines().count());
    let grid = text.chars().filter(|c| c != &'\n').collect::<Vec<_>>();
    let mut regions = DisjointSetWithCount::new(grid.len());
    let mut perimeter = vec![0_usize; grid.len()];

    for (node, char) in grid.iter().enumerate() {
        let pos = UsizePoint::from_index(grid_size, node);

        for direc in Direc::POWERS_OF_I {
            match pos.next_point(&direc, grid_size) {
                Some(neighbor) => {
                    let neighbor = neighbor.as_index(grid_size);
                    if &grid[neighbor] != char {
                        perimeter[node] += 1;
                        continue;
                    }
                    regions.link(node, neighbor);
                }
                None => {
                    perimeter[node] += 1;
                    continue;
                }
            }
        }
    }

    let mut new_perimeter = HashMap::<usize, usize>::new();
    for node in 0..grid.len() {
        *new_perimeter
            .entry(regions.bookkeeping_eve(node))
            .or_default() += perimeter[node];
    }

    return new_perimeter
        .drain()
        .map(|(node, perimeter)| perimeter * regions.size_of_eve(node))
        .sum();
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day12.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    const TEXT2: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn part1_given_example1() {
        assert_eq!(part1(TEXT1), 140);
    }

    #[test]
    fn part1_given_example2() {
        assert_eq!(part1(TEXT2), 1930);
    }
}
