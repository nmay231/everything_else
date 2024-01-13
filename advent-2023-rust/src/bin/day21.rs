use std::collections::{HashMap, HashSet};

use advent_2023_rust::{Direc, UsizePoint, Zipper};
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

struct NewEntries(usize, Vec<(usize, UsizePoint)>, NewEntriesMap);
type NewEntriesMap = HashMap<UsizePoint, NewEntries>;

struct NewEntriesMapZipper(Vec<(UsizePoint, NewEntriesMap)>, NewEntriesMap);

impl Zipper for NewEntriesMapZipper {
    type Target = NewEntriesMap;
    type Index = UsizePoint;

    fn child(mut self, index: Self::Index) -> Result<Self, Self> {
        match self.1.get_mut(&index) {
            Some(new_entries) => {
                let map = std::mem::replace(&mut new_entries.2, NewEntriesMap::new());
                self.0.push((index.to_owned(), self.1));
                Ok(Self(self.0, map))
            }
            None => Err(self),
        }
    }

    fn parent(mut self) -> Result<Self, Self> {
        match self.0.pop() {
            Some((index, mut parent)) => {
                parent
                    .entry(index)
                    .or_insert_with(|| panic!("Should exist already"))
                    .2 = self.1;
                Ok(Self(self.0, parent))
            }
            None => Err(self),
        }
    }

    fn from(target: Self::Target) -> Self {
        NewEntriesMapZipper(vec![], target)
    }

    fn unwrap_target(self) -> Self::Target {
        self.1
    }
}

// fn find_exits(grid: &[char], grid_size: &UsizePoint, start: UsizePoint, direc: Option<Direc>) {
//     let mut exits = HashMap::new();
//     if let Some(direc) = direc {
//         exits.insert(direc, start.clone());
//     }
//
//     let mut points = HashSet::new();
//     points.insert(start);
//
//     for step in 0_usize.. {
//         points = points
//             .iter()
//             .flat_map(|point| {
//                 Direc::POWERS_OF_I.iter().flat_map(|direc| {
//                     point.next_point(direc, grid_size).and_then(|next_point| {
//                         (grid[next_point.as_index(grid_size)] == '.').then_some(next_point)
//                     })
//                 })
//             })
//             .collect();
//
//         let char = if step & 1 != steps & 1 { 'O' } else { '_' };
//         for point in &points {
//             grid[point.as_index(grid_size)] = char;
//         }
//     }
//
// }

fn simulate_grid(
    subgrid: &[char],
    grid_size: &UsizePoint,
    entries: Vec<(usize, UsizePoint)>,
    max_steps: usize,
) -> Vec<(usize, UsizePoint, Direc)> {
    assert_eq!(entries[0].0, 0);
    let mut points = HashSet::new();
    points.insert((entries[0].1, false));
    let mut visited = HashSet::new();

    let mut boundaries = vec![];
    let mut even = 0;
    let mut odd = 0;

    for step in 0..max_steps {
        points = points
            .iter()
            .cartesian_product(Direc::POWERS_OF_I.iter())
            .flat_map(|((point, hugging_edge), direc)| {
                point
                    .next_point(direc, grid_size)
                    .or_else(|| {
                        // When a point visits an edge, it ripples in both
                        // directions while hugging it. This is useless to
                        // track, therefore, we only count the first encounter
                        if !hugging_edge {
                            boundaries.push((step, point.to_owned(), direc.to_owned()));
                        }
                        None
                    })
                    .and_then(|next_point| {
                        if subgrid[next_point.as_index(grid_size)] == '.'
                            && !visited.contains(&next_point)
                        {
                            visited.insert(next_point.clone());
                            let still_hugging =
                                point.is_on_edge(grid_size) && next_point.is_on_edge(grid_size);
                            Some((next_point, still_hugging))
                        } else {
                            None
                        }
                    })
            })
            .collect();

        if step % 2 == 0 {
            even += points.len();
        } else {
            odd += points.len();
        }
    }
    boundaries
}

/// My first strategy is going to be caching the resulting walk-outs by when you
/// walked in (ignoring useless walk-ins; places you would have already walked).
/// So I have a grid square 26501365/131 long and wide and every cell is the
/// input grid, a sub-grid. Then I have a map for when a grid is entered by it's
/// neighbors (only checking the boundaries). So I don't cache computation by the
/// exact form of its 8 neighbors, but by when the neighbors enter it.
///
/// Luckily, I think this is guaranteed to work since there is a border of `.`
/// meaning if you enter subgrid A, then subgrid B from A, I never have to worry
/// about going from B to A since it will always be faster to go to it from A
/// (this is not guaranteed in general).
///
/// Definitely have to test with the small grid first to see if it's feasible.

fn part2(text: &str, n_steps: usize) -> Output {
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    let mut grid = text.replace('\n', "").chars().collect_vec();
    let subgrid_start = UsizePoint::from_index(
        grid_size,
        grid.iter()
            .enumerate()
            .find_map(|(i, char)| (*char == 'S').then_some(i))
            .unwrap(),
    );
    grid[subgrid_start.as_index(grid_size)] = '.';

    let super_grid_size = &UsizePoint(
        (grid_size.0 / n_steps) * 2 + 1,
        (grid_size.1 / n_steps) * 2 + 1,
    );
    let mut super_grid = vec![Some(vec![]); super_grid_size.0 * super_grid_size.1];
    // let mut next_sub_grids = vec![(
    //     0_usize,
    //     UsizePoint(super_grid_size.0 / 2, super_grid_size.1 / 2),
    // )];
    let super_start = UsizePoint(super_grid_size.0 / 2, super_grid_size.1 / 2);
    super_grid[super_start.as_index(super_grid_size)]
        .as_mut()
        .unwrap()
        .push((0_usize, subgrid_start));
    let mut next_subgrid = HashMap::new();
    next_subgrid.insert(super_start.to_owned(), 0_usize);

    let mut new_entries = NewEntriesMap::new();
    new_entries.insert(
        super_start.to_owned(),
        NewEntries(0, vec![(0, subgrid_start)], NewEntriesMap::new()),
    );

    while next_subgrid.len() > 0 {
        let (subgrid_pos, entry_step) = next_subgrid
            .iter()
            .reduce(|a, b| if a.1 < b.1 { a } else { b })
            .unwrap();
        let subgrid =
            std::mem::replace(&mut super_grid[subgrid_pos.as_index(super_grid_size)], None);
        let mut entries = subgrid.expect("Revisiting a subgrid that was already accounted for");
        entries.sort_by_key(|x| x.0);

        // We see if we already calculated the path for a similar set of entries
        for (step, point) in entries {
            // TODO: Continue here. First I should not use new_entries, then I
            // should use it and figure out how since it's a bit convoluted.
            // Then I need to update simulate_grid to track even/odd (only
            // necessary each time since the fringes of the exploration will
            // only fill part of the grid; but too complicated to optimize that).
            // if new_entries.
        }

        let exits = simulate_grid(&grid, grid_size, entries, n_steps - *entry_step);
        for (step_offset, edge_point, direc) in exits {
            assert_eq!(edge_point.next_point(&direc, grid_size), None);
            let entry = edge_point.next_point_wrap(&direc, grid_size);

            let adjacent_subgrid_pos = subgrid_pos
                .next_point(&direc, super_grid_size)
                .expect("Super grid was not made big enough for number of iteration steps");

            match &mut super_grid[adjacent_subgrid_pos.as_index(super_grid_size)] {
                Some(entries) => entries.push((0, entry)),
                // The adjacent subgrid was already handled
                None => continue,
            }
        }
        // next_sub_grid.ex
    }
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day21.txt")?;

    println!("part 1 result = {:?}", part1(&text, 64));
    println!("part 2 result = {:?}", part2(&text, 6));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        assert!(true);
    }
}
