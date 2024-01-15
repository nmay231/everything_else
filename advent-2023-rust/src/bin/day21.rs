use std::collections::{HashMap, HashSet};

use advent_2023_rust::{Direc, UsizePoint, Zipper};
use indoc::indoc;
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

    fn new(target: Self::Target) -> Self {
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
    let mut points = HashMap::new();
    points.insert(entries[0].1, false);
    let mut visited = HashSet::new();

    let mut boundaries = vec![];
    let mut even = 0;
    let mut odd = 0;

    // TODO: I am trying to fix this so that each travel along the edge is
    // visited once instead of multiple times like it is now. This is so that I
    // can check if most times I enter the next subgrid on the corner and every
    // other time is on one point of the edge. If that's the case, it greatly
    // simplifies my problem. I don't think it is in the example, but I do with
    // the actual input. But we'll see.

    for step in 0..max_steps {
        let iter = points
            .into_iter()
            .cartesian_product(Direc::POWERS_OF_I.iter())
            .flat_map(|((point, hugging_edge), direc)| {
                if point.is_on_edge(grid_size) && !hugging_edge {
                    println!("{:?}", (&point, direc));
                }
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
            });
        points = HashMap::new();
        for (point, hugging_edge) in iter {
            // Prefer hugging edge instead of not
            *points.entry(point).or_insert(false) |= hugging_edge;
        }
        // .collect();

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
/// neighbors (only checking the boundaries). So I don't cache computation by
/// the exact form of its 8 neighbors, but by when the neighbors enter it.
///
/// Luckily, I think this is guaranteed to work since there is a border of `.`
/// meaning if you enter subgrid A, then subgrid B from A, I never have to worry
/// about going from B to A since it will always be faster to go to it from A
/// (this is not guaranteed in general).
///
/// Revisiting this, since the outline of the grid is empty space ('.'), when
/// you enter a subgrid by the corner, you always visit the next subgrid's by
/// the corner since there is no faster path than skirting around the edges. So,
/// the way to solve this is to keep track of the four cardinal directions from
/// the starting grid (since they enter on the sides), track how many grids are
/// fully subsumed by diagonal traversal and what parity they are, and finally
/// the progress into the partially visited grids in the big diamond around the
/// center.
///
/// For the fully subsumed grids, I just need to know the time it takes to get
/// to each corner, then how many grids are fully consumed, which is the
/// triangle number of `(steps_left_after_corner - 1) / (grid_x + grid_y - 1)`.
/// To get the grids on the big diamond (excluding the cardinal corners), you
/// find the remainder of that division and travel that far into the subgrid,
/// then times by the above number (since each subgrid is the same at that
/// point).

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
    let super_start = UsizePoint(super_grid_size.0 / 2, super_grid_size.1 / 2);
    super_grid[super_start.as_index(super_grid_size)]
        .as_mut()
        .unwrap()
        .push((0_usize, subgrid_start));
    let mut next_subgrid = HashMap::new();
    next_subgrid.insert(super_start.to_owned(), 0_usize);

    let mut new_entries = NewEntriesMap::new();
    // new_entries.insert(
    //     super_start.to_owned(),
    //     NewEntries(0, vec![(0, subgrid_start)], NewEntriesMap::new()),
    // );

    while next_subgrid.len() > 0 {
        let (subgrid_pos, entry_step) = next_subgrid
            .iter()
            .reduce(|a, b| if a.1 < b.1 { a } else { b })
            .unwrap();
        let subgrid =
            std::mem::replace(&mut super_grid[subgrid_pos.as_index(super_grid_size)], None);
        let mut entries = subgrid.expect("Revisiting a subgrid that was already accounted for");
        entries.sort_by_key(|x| x.0);

        let mut zipper = NewEntriesMapZipper::new(new_entries);
        let mut step_point: Option<()> = None;
        // We see if we already calculated the path for a similar set of entries
        for (step, point) in entries.iter() {
            match zipper.1.get(point) {
                Some(entry) => {
                    // if
                    // entry.0
                    todo!()
                }
                None => todo!(),
            }
            // if let Some(x) = zipper.1.get(point) {
            //     x
            // }
            // match zipper.child(point) {
            //     Ok(child) => zipper = child,
            //     Err(mut unchanged) => {
            //         // unchanged.1.insert(k, v);
            //         zipper = unchanged;
            //         step_point = Some((step, point));
            //         break;
            //     }
            // }
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
        new_entries = zipper.unzip();
    }
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day21.txt")?;

    println!("part 1 result = {:?}", part1(&text, 64));
    // println!("part 2 result = {:?}", part2(&text, 6));
    let text = indoc! {"
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ..........."};
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    let mut grid = text.replace('\n', "").chars().collect_vec();
    println!(
        "{:?}",
        (simulate_grid(&grid, grid_size, vec![(0, UsizePoint(5, 5))], 50))
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        assert!(true);
    }
}
