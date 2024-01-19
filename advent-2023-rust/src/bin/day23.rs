use std::collections::{HashMap, VecDeque};

use advent_2023_rust::{Direc, UsizePoint};
use itertools::Itertools;

type Output = usize;

fn part1(text: &str) -> Output {
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    let grid = text.trim().replace('\n', "").chars().collect_vec();

    let destination = UsizePoint(grid_size.0 - 1, grid_size.1 - 2);
    assert_eq!(grid[destination.as_index(grid_size)], '.');
    assert_eq!(destination.next_point(&Direc::South, grid_size), None);

    let mut paths = vec![(UsizePoint(0, 1), Direc::North, vec![])];
    let mut max_distance = 0;
    while let Some((mut point, mut from_direc, mut visited)) = paths.pop() {
        loop {
            let next = Direc::POWERS_OF_I
                .iter()
                .filter_map(|direc| {
                    if direc == &from_direc {
                        None
                    } else {
                        let next_point = point.next_point(direc, grid_size)?;
                        let char = grid[next_point.as_index(grid_size)];
                        match (char, direc) {
                            ('.', _)
                            | ('>', Direc::East)
                            | ('<', Direc::West)
                            | ('v', Direc::South)
                            | ('^', Direc::North) => Some((next_point, direc.rotate(2))),
                            ('#' | '>' | '<' | 'v' | '^', _) => None,
                            _ => {
                                unreachable!("Unexpected character at {:?}: '{}'", next_point, char)
                            }
                        }
                    }
                })
                .collect_vec();

            if next.len() == 0 {
                if point == destination {
                    max_distance = std::cmp::max(max_distance, visited.len());
                }
                break; // Dead-end
            } else {
                if next.iter().skip(1).any(|(point, _)| point == &destination) {
                    println!("{:?}", (visited.len(), &visited));
                }
                visited.push(point);
                (point, from_direc) = next[0];

                if next.len() > 1 {
                    paths.extend(
                        next.into_iter()
                            .skip(1)
                            .map(|(start, from_direc)| (start, from_direc, visited.clone())),
                    )
                }
            }
        }
    }
    max_distance
}

type VertexDistances = HashMap<UsizePoint, HashMap<UsizePoint, usize>>;
trait VertexDistancesTrait {
    fn insert_conn(&mut self, a: UsizePoint, b: UsizePoint, distance: usize);
}

impl VertexDistancesTrait for VertexDistances {
    fn insert_conn(&mut self, a: UsizePoint, b: UsizePoint, distance: usize) {
        self.entry(a).or_default().insert(b, distance);
        self.entry(b).or_default().insert(a, distance);
    }
}

fn part2(text: &str) -> Output {
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    let grid = text
        .trim()
        .replace('\n', "")
        .replace(['>', '<', 'v', '^'], ".")
        .chars()
        .collect_vec();

    let destination = UsizePoint(grid_size.0 - 1, grid_size.1 - 2);
    // Yes, I needed these assertions because I was tired
    assert_eq!(grid[destination.as_index(grid_size)], '.');
    assert_eq!(destination.next_point(&Direc::South, grid_size), None);

    let start = UsizePoint(0, 1);
    let mut vertex_distances = HashMap::new();
    let mut tmp = HashMap::new();
    tmp.insert(start, 0);
    vertex_distances.insert(start, tmp);

    let mut partial_conns = vec![(start, Direc::South)];

    while let Some((point, direc)) = partial_conns.pop() {
        let (mut previous, mut point) = (point, point.next_point(&direc, grid_size).unwrap());
        let start = previous.clone();
        let mut distance = 1;
        loop {
            let neighbors = Direc::POWERS_OF_I
                .iter()
                .filter_map(|direc| {
                    let next_point = point.next_point(direc, grid_size)?;
                    (grid[next_point.as_index(grid_size)] == '.' && next_point != previous)
                        .then_some((next_point, *direc))
                })
                .collect_vec();

            match &neighbors[..] {
                [] => {
                    assert_eq!(point, destination);
                    vertex_distances.insert_conn(start, point, distance);
                    break;
                }
                [(neighbor, _)] => {
                    (previous, point) = (point, *neighbor);
                    distance += 1;
                }
                adjacent => {
                    assert!(adjacent.len() < 4);

                    if !vertex_distances.contains_key(&point) {
                        partial_conns.extend(adjacent.iter().map(|(_, direc)| (point, *direc)));
                    }
                    vertex_distances.insert_conn(start, point, distance);
                    break;
                }
            }
        }
    }

    let mut paths = VecDeque::from([(start, vec![start], 0)]);
    let mut max = 0;

    while let Some((current, visited, distance)) = paths.pop_front() {
        let adjacent = vertex_distances
            .get(&current)
            .expect("every vertex to exist as a key");

        paths.extend(adjacent.iter().filter_map(|(adj, adj_dis)| {
            let distance = distance + adj_dis;
            if visited.contains(adj) {
                None
            } else if adj == &destination {
                max = std::cmp::max(max, distance);
                None
            } else {
                let mut visited = visited.clone();
                visited.push(current);
                Some((*adj, visited, distance))
            }
        }))
    }
    max
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day23.txt")?;

    println!("part 1 result = {:?}", part1(&text));
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
