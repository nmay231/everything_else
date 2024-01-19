use std::collections::{HashMap, HashSet, VecDeque};

use advent_2023_rust::{Direc, UsizePoint, Zipper};
use indoc::indoc;
use itertools::Itertools;
use num_integer::Integer;

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

    if steps == 100 && false {
        let n_repeats = grid_size.0 / 11;
        // grid_size.debug_grid(&grid);
        for (i, row) in grid.chunks_exact(11).enumerate() {
            let i = i + 1;
            print!("{}", String::from_iter(row));
            if i % (11 * n_repeats) == 0 {
                println!("\n");
            } else if i % n_repeats == 0 {
                println!();
            } else {
                print!(" ");
            }
        }
        println!();
    }

    grid.iter().filter(|char| char == &&'O').count()
}

fn _part2_brute_force(text: &str, n_steps: usize) -> Output {
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());

    let tmp_grid = text.replace('\n', "").chars().collect_vec();
    let padding = n_steps.div_ceil(text.lines().count());
    let repeat = 2 * padding + 1;
    let top_row = text
        .replace('S', "T")
        .lines()
        .map(|row| row.repeat(repeat))
        .join("\n");
    let text = [top_row].iter().cycle().take(repeat).join("\n");
    let text = text.replacen('T', "t", padding * repeat + padding);
    let text = text.replacen('T', "S", 1);
    let text = text.replace(['T', 't'], ".");
    assert_eq!(text.chars().filter(|c| c == &'S').count(), 1);
    // println!("{}", text);
    assert!(text.find(|c| !['.', '#', 'S', '\n'].contains(&c)).is_none());

    part1(&text, n_steps)

    // assert_eq!(
    //     grid_size.0, grid_size.1,
    //     "Handling parity with rectangular grids is a pain"
    // );

    // tmp_grid[start.as_index(grid_size)] = '.';
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
                // if point.is_on_edge(grid_size) && !hugging_edge {
                //     println!("{:?}", (&point, direc));
                // }
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

fn grid_replace(grid: &mut [char], from: char, to: char) {
    for c in grid.iter_mut() {
        if c == &from {
            *c = to;
        }
    }
}

/// Returns (even, odd) counts of visited cells == 'O'
fn count_visits(grid: &[char], grid_size: &UsizePoint) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .fold((0, 0), |(even, odd), (index, c)| {
            let point = UsizePoint::from_index(grid_size, index);
            if c == &'O' {
                match (point.0 + point.1).is_even() {
                    true => (even + 1, odd),
                    false => (even, odd + 1),
                }
            } else {
                (even, odd)
            }
        })
}

fn find_exits(
    grid: &mut [char],
    grid_size: &UsizePoint,
    n_steps: usize,
    mut points: Vec<(usize, UsizePoint)>,
) -> (Vec<Vec<(usize, UsizePoint)>>, HashMap<UsizePoint, usize>) {
    let mut boundary_entries = vec![vec![]; 4];
    let mut diagonal_entries = [
        UsizePoint(0, 0),
        UsizePoint(0, grid_size.1 - 1),
        UsizePoint(grid_size.0 - 1, 0),
        UsizePoint(grid_size.0 - 1, grid_size.1 - 1),
    ]
    .into_iter()
    .map(|key| (key, usize::MAX))
    .collect::<HashMap<_, _>>();

    // let mut points = vec![(0_usize, start)];
    points.sort_by_key(|(step, _)| *step);
    // println!("{:?}", (expected_min, &points));
    let mut points = VecDeque::from_iter(points);

    loop {
        let len = points.len();
        let expected_min = points[0].0;

        for _ in 0..len {
            let Some((step, point)) = points.pop_front() else {
                unreachable!();
            };

            // println!("{:?}", (expected_min, step, &point));
            if grid[point.as_index(grid_size)] != '.' || step > n_steps {
                continue;
            } else if step != expected_min {
                points.push_back((step, point));
                continue;
            }

            // We only expect to increment by one
            // assert!(step >= expected_min);
            // expected_min = step;
            for direc in Direc::POWERS_OF_I {
                match point.next_point(&direc, grid_size) {
                    None => boundary_entries[direc.to_power_of_i()]
                        .push((step + 1, point.next_point_wrap(&direc, grid_size))),
                    Some(next_point) => points.push_back((step + 1, next_point)),
                }

                if let Some(steps_to) = diagonal_entries.get_mut(&point) {
                    *steps_to = std::cmp::min(*steps_to, step);
                }
            }

            grid[point.as_index(grid_size)] = 'O';
        }

        if points.len() == 0 {
            return (boundary_entries, diagonal_entries);
        }
        assert_eq!(points[0].0, expected_min + 1);
    }
    // (boundary_entries, diagonal_entries)
}

fn part2(text: &str, n_steps: usize) -> Output {
    // Parts that we count individually
    //       1
    //      313
    //     33133
    //    3321233
    //   332212233
    //  33222122233
    // 1111110111111
    //  33222122233
    //   332212233
    //    3321233
    //     33133
    //      313
    //       1
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    assert_eq!(
        grid_size.0, grid_size.1,
        "Handling parity with rectangular grids is a pain"
    );

    let mut grid = text.replace('\n', "").chars().collect_vec();
    let start = UsizePoint::from_index(
        grid_size,
        grid.iter()
            .enumerate()
            .find_map(|(i, char)| (*char == 'S').then_some(i))
            .unwrap(),
    );
    grid[start.as_index(grid_size)] = '.';
    // Part 0
    let (boundary_entries, diagonal_entries) =
        find_exits(&mut grid, grid_size, n_steps, vec![(0_usize, start)]);

    // Technically not the max if part of the grid is missed, but then it
    // doesn't need to be accurate.
    let (max_even, max_odd) = count_visits(&grid, grid_size);
    grid_replace(&mut grid, 'O', '.');

    // (step, "entry into grid", "number of these grids", ("# cells an even step away from origin",
    // "ditto, but odd"))
    let mut entries_to_grids = vec![(0, start, 1, (max_even, max_odd))];
    // println!("part0: {:?}", (entries_to_grids));
    // let mut saved = entries_to_grids.clone();

    // println!("{}", "part0 finished");

    // Part 1
    for (direc, mut entries) in Direc::POWERS_OF_I.iter().zip(boundary_entries) {
        // println!("checking outer{:?}", ());
        let mut known_entry_patterns = vec![];
        let mut _count = 0;
        while entries.len() > 0 {
            _count += 1;
            let (mut step, point) = entries[0];
            // println!("{:?}", (_count, step, known_entry_patterns.len()));

            let were_going_in_circles =
                known_entry_patterns
                    .iter()
                    .rfind(|(known, _): &&(Vec<_>, _)| {
                        let mut known = known.to_owned();
                        known.sort_by_key(|(step, _)| *step);
                        let mut entries = entries.clone();
                        entries.sort_by_key(|(step, _)| *step);

                        let (known_min, entry_min) = (known[0].0, entries[0].0);

                        entries
                            .iter()
                            .zip_longest(known.iter())
                            .map(|x| match x {
                                itertools::EitherOrBoth::Left(_) => false,
                                itertools::EitherOrBoth::Right(_) => false,
                                itertools::EitherOrBoth::Both(entry, known) => {
                                    (entry.0 - entry_min, entry.1) == (known.0 - known_min, known.1)
                                }
                            })
                            .all_equal_value()
                            .is_ok_and(|x| x == true)
                    });

            if let Some((_, result_entries)) = were_going_in_circles {
                let steps_left = (n_steps - step).saturating_sub(1);
                let per_grid = grid_size.0;

                let n_full_grids = steps_left / per_grid;
                let n_full_grids = n_full_grids.saturating_sub(1);
                // let mut entry_step = steps + 2;
                if n_full_grids > 0 {
                    assert_eq!(
                        result_entries, &entries,
                        "The entry pattern should only cycle back to itself"
                    );

                    entries_to_grids.extend([
                        (step, point, n_full_grids.div_ceil(2), (max_even, max_odd)),
                        (
                            step + per_grid,
                            point,
                            n_full_grids / 2,
                            (max_even, max_odd),
                        ),
                    ]);

                    entries = entries
                        .into_iter()
                        .map(|(step, point)| (step + n_full_grids * per_grid, point))
                        .collect();
                    step += n_full_grids * per_grid;
                    // No need to update point because of the assertion above.
                }
            }

            // let (steps, point) = entries[0];
            // let need_to_flip_parity = (steps % 2) != ((point.0 + point.1) % 2);

            let original_entries = entries.clone();
            let (boundary_entries, _) = find_exits(&mut grid, grid_size, n_steps, entries);
            let (even, odd) = count_visits(&grid, grid_size);
            grid_replace(&mut grid, 'O', '.');

            entries_to_grids.push((step, point, 1, (even, odd)));
            // if need_to_flip_parity {
            //     total_even += odd;
            //     total_odd += even;
            // } else {
            //     total_even += even;
            //     total_odd += odd;
            // }

            // TODO: Is there a better way than just cloning?
            entries = boundary_entries[direc.to_power_of_i()].clone();
            // let boundary_entries = boundary_entries[direc.to_power_of_i()];
            // entries = boundary_entries;
            // let entries = boundary_entries
            //     .into_iter()
            //     .nth(direc.to_power_of_i())
            //     .unwrap();
            // entries = *boundary_entries.index(direc.to_power_of_i());

            known_entry_patterns.push((original_entries, entries.clone()))
        }
        // println!("{}", "part1 (1/4) finished");

        // println!(
        //     "part1: {:?}",
        //     (
        //         direc,
        //         count,
        //         "grids traversed",
        //         &entries_to_grids[saved.len()..],
        //     )
        // );
        // saved = entries_to_grids.clone();
    }

    // println!("{:?}", (diagonal_entries));
    for (exit_corner, steps) in diagonal_entries {
        if steps == usize::MAX {
            continue;
        }
        let to_grid_edge = Direc::POWERS_OF_I
            .iter()
            .circular_tuple_windows()
            .find(|(a, b)| {
                exit_corner.next_point(a, grid_size).is_none()
                    && exit_corner.next_point(b, grid_size).is_none()
            })
            .expect("A corner should have two grid edges");
        let entry_corner = exit_corner
            .next_point_wrap(to_grid_edge.0, grid_size)
            .next_point_wrap(to_grid_edge.1, grid_size);

        // Part 2
        let steps_left = (n_steps - steps).saturating_sub(1);
        let per_grid = grid_size.0;

        let n_full_grids = steps_left / per_grid;
        let n_full_grids = n_full_grids.saturating_sub(1);
        let mut entry_step = steps + 2;

        for in_diagonal_slice in 1..=n_full_grids {
            entries_to_grids.push((
                entry_step,
                entry_corner,
                in_diagonal_slice,
                (max_even, max_odd),
            ));
            // entries_to_grids.extend(
            //     [(entry_step, entry_corner, (max_even, max_odd))]
            //         .iter()
            //         .cycle()
            //         .take(in_diagonal_slice),
            // );
            entry_step += per_grid;
        }
        // println!("{}", "part2 (1/4) finished");

        // println!(
        //     "part2: {:?}",
        //     (&exit_corner, &entries_to_grids[saved.len()..],)
        // );
        // saved = entries_to_grids.clone();

        // Part 3: Inner diagonal slice
        // let entry_step = n_steps - last_leg;
        let (_, diagonal_entries) = find_exits(
            &mut grid,
            grid_size,
            n_steps,
            vec![(entry_step, entry_corner)],
        );
        let (even, odd) = count_visits(&grid, grid_size);
        grid_replace(&mut grid, 'O', '.');

        entries_to_grids.push((entry_step, entry_corner, n_full_grids + 1, (even, odd)));
        // entries_to_grids.extend(
        //     [(entry_step, entry_corner, (even, odd))]
        //         .iter()
        //         .cycle()
        //         .take(n_full_grids + 1),
        // );

        // Part 3: Outer diagonal slice (if it exists)
        let a_side_corner = exit_corner.next_point_wrap(to_grid_edge.0, grid_size);
        let entry_step = *diagonal_entries.get(&a_side_corner).unwrap();
        if entry_step != usize::MAX {
            let entry_step = entry_step + 1;
            find_exits(
                &mut grid,
                grid_size,
                n_steps,
                vec![(entry_step, entry_corner)],
            );
            let (even, odd) = count_visits(&grid, grid_size);
            grid_replace(&mut grid, 'O', '.');

            entries_to_grids.push((entry_step, entry_corner, n_full_grids + 2, (even, odd)));
            // entries_to_grids.extend(
            //     [(entry_step, entry_corner, (even, odd))]
            //         .iter()
            //         .cycle()
            //         .take(n_full_grids + 2),
            // );
        }

        // let (even, odd) = (even * (n_grids + 1), odd * (n_grids + 1));
        // if n_grids % 2 == 0 {
        //     total_even += even;
        //     total_odd += odd;
        // } else {
        //     total_even += odd;
        //     total_odd += even;
        // }

        // println!(
        //     "part3: {:?}",
        //     (&exit_corner, &entries_to_grids[saved.len()..],)
        // );
        // saved = entries_to_grids.clone();
        // println!("{}", "part3 (1/4) finished");
    }

    let mut result = 0;

    for (step, entry_point, count, (even, odd)) in entries_to_grids {
        // Yes, count could have been multiplied before, but this is easier to debug
        result += count
            * if (step + entry_point.0 + entry_point.1 + n_steps) % 2 == 0 {
                even
            } else {
                odd
            }
    }
    result

    // println!("even/odd: {:?}", (total_even, total_odd));
    // if (n_steps + start_parity) % 2 == 0 {
    //     total_even
    // } else {
    //     total_odd
    // }

    // Can't do this since I can cross boundary without touching corner
    // if diagonal_entries.values().any(|step| step == &n_steps) {
    //     println!("{:?}", ());
    // }

    // for step_to_
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day21.txt")?;

    println!("part 1 result = {:?}", part1(&text, 64));
    println!("part 2 result = {:?}", part2(&text, 26501365));
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
    // let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    // let mut grid = text.replace('\n', "").chars().collect_vec();
    // println!(
    //     "{:?}",
    //     (simulate_grid(&grid, grid_size, vec![(0, UsizePoint(5, 5))], 50))
    // );

    // In exactly 6 steps, he can still reach 16 garden plots.
    // In exactly 10 steps, he can reach any of 50 garden plots.
    // In exactly 50 steps, he can reach 1594 garden plots.
    // In exactly 100 steps, he can reach 6536 garden plots.
    // In exactly 500 steps, he can reach 167004 garden plots.
    // In exactly 1000 steps, he can reach 668697 garden plots.
    // In exactly 5000 steps, he can reach 16733044 garden plots.

    for n_steps in [6, 10, 50, 100, 500, 1000, 5000] {
        // println!("part1: {}", part1(text, n_steps));
        println!(
            "In exactly {} steps, he can reach {} garden plots",
            n_steps,
            part2(text, n_steps)
        );
        // println!("But in reality it's: {}", _part2_brute_force(text, n_steps));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        assert!(true);
    }
}
