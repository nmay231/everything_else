use std::collections::{HashMap, HashSet};

use advent_2024_rust::{Direc, DisjointSet, DisjointSetWithCount, Eve, UsizePoint};

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
            .entry(regions.bookkeeping_eve_index(node))
            .or_default() += perimeter[node];
    }

    return new_perimeter
        .drain()
        .map(|(node, perimeter)| perimeter * regions.size_of_eve(node))
        .sum();
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct RegionEdge {
    pos: UsizePoint,
    adjacent_fence: Direc,
}

#[derive(Debug, Default)]
struct Region(HashSet<RegionEdge>, usize);
// NOTE: It was helpful to include the char representative to debug
// struct Region(HashSet<RegionEdge>, usize, char);

impl Eve for Region {
    fn init(_index: usize) -> Self {
        Self(HashSet::new(), 1) //, '\0')
    }

    fn merge(&self, other: &Self) -> Self {
        // TODO: This would be better if `merge()` used owned values instead,
        // but I'm too lazy to make that change right now
        Self(
            HashSet::from_iter(
                self.0
                    .iter()
                    .chain(other.0.iter())
                    .map(|region_edge| region_edge.to_owned()),
            ),
            self.1 + other.1,
            // self.2,
        )
    }
}

fn part2(text: &str) -> Output {
    let grid_size = &UsizePoint(text.find('\n').unwrap(), text.lines().count());
    let grid = text.chars().filter(|c| c != &'\n').collect::<Vec<_>>();

    let mut regions = DisjointSet::<Region>::new(grid.len());
    for (node, char) in grid.iter().enumerate() {
        let pos = UsizePoint::from_index(grid_size, node);

        for direc in Direc::POWERS_OF_I {
            match pos.next_point(&direc, grid_size) {
                Some(neighbor) => {
                    let neighbor = neighbor.as_index(grid_size);
                    if &grid[neighbor] != char {
                        regions.eve_mut(node).0.insert(RegionEdge {
                            pos,
                            adjacent_fence: direc,
                        });
                        // regions.eve_mut(node).2 = *char;
                    } else {
                        // regions.eve_mut(node).2 = *char;
                        regions.link(node, neighbor);
                    }
                }
                None => {
                    regions.eve_mut(node).0.insert(RegionEdge {
                        pos,
                        adjacent_fence: direc,
                    });
                }
            }
        }
    }

    let mut total = 0;
    for node in 0..grid.len() {
        if !regions.is_an_eve(node) {
            continue;
        }

        let Region(edges, area) = regions.eve(node);
        let mut perimeter = edges.len();
        for edge in edges {
            let RegionEdge {
                pos,
                adjacent_fence,
            } = edge;

            if let Some(pos) = pos.next_point(&adjacent_fence.rotate(1), grid_size) {
                if edges.contains(&RegionEdge {
                    pos,
                    adjacent_fence: *adjacent_fence,
                }) {
                    perimeter -= 1;
                }
            }
        }

        // println!("region: {:?}", (repr, area, perimeter, area * perimeter));

        total += area * perimeter;
    }

    return total;
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day12.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use indoc::indoc;
    use rstest::rstest;

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

    #[rstest]
    #[case::a(TEXT1, 140)]
    #[case::b(TEXT2, 1930)]
    fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
        assert_eq!(part1(text), expected);
    }

    #[rstest]
    #[case::a(TEXT1, 80)]
    #[case::b(TEXT2, 1206)]
    fn part2_given_examples(#[case] text: &str, #[case] expected: usize) {
        assert_eq!(part2(text), expected);
    }
}
