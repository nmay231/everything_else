use std::cmp::Ordering;
use std::collections::HashMap;

use advent_2025_rust::{Direc, Point};
use itertools::Itertools;

fn part1(text: &str) -> usize {
    let mut red_tiles = vec![];

    for line in text.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let [a, b] = [a, b].map(|s| s.parse::<usize>().unwrap());
        red_tiles.push((a, b))
    }

    let mut max_area = 0;
    for (index, a) in red_tiles.iter().enumerate() {
        for b in red_tiles[index + 1..].iter() {
            max_area = max_area.max((1 + a.0.abs_diff(b.0)) * (1 + a.1.abs_diff(b.1)));
        }
    }

    return max_area;
}

fn part2(text: &str) -> usize {
    let mut red_tiles = vec![];

    for line in text.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let [x, y] = [x, y].map(|s| s.parse::<usize>().unwrap());
        red_tiles.push((x, y));
    }

    // ccw = counter-clockwise
    let mut ccw_turns = 0_isize;
    let mut prev_corner = red_tiles.last().unwrap();
    let mut prev_direc: Option<Direc> = None;

    for corner in red_tiles
        .iter()
        .chain(red_tiles.iter())
        .take(red_tiles.len() + 1)
    {
        let direc = match (prev_corner.0.cmp(&corner.0), prev_corner.1.cmp(&corner.1)) {
            (Ordering::Equal, Ordering::Less) => Direc::South,
            (Ordering::Equal, Ordering::Greater) => Direc::North,
            (Ordering::Less, Ordering::Equal) => Direc::East,
            (Ordering::Greater, Ordering::Equal) => Direc::West,
            _x => unreachable!("{:?}", (_x, &prev_corner, &corner)),
        };

        if let Some(prev_direc) = prev_direc {
            let tmp = (direc.to_power_of_i() as isize - prev_direc.to_power_of_i() as isize + 4)
                .rem_euclid(4);
            assert!([1, 3].contains(&tmp), "{}", tmp);
            ccw_turns += if tmp == 1 { 1 } else { -1 };
        }
        // println!("{}", ccw_turns);

        prev_direc = Some(direc);
        prev_corner = corner;
    }

    assert!([-4, 4].contains(&ccw_turns), "{}", ccw_turns);

    if ccw_turns == 4 {
        // Convert the loop to be defined clockwise, if not already
        red_tiles.reverse();
    }

    let mut x_regions = vec![0..=usize::MAX];
    let mut y_regions = vec![0..=usize::MAX];

    for tile in &red_tiles {
        for (coord, regions) in [(tile.0, &mut x_regions), (tile.1, &mut y_regions)] {
            let mut splice = vec![];
            let mut index = None;
            for (i, range) in regions.iter().enumerate() {
                if !range.contains(&coord) {
                    continue;
                }

                if range.start() < &coord {
                    splice.push(*range.start()..=coord);
                }
                splice.push(coord..=coord);
                if range.end() > &coord {
                    splice.push(coord..=*range.end());
                }
                index = Some(i);

                break;
            }
            assert!(splice.len() > 0);

            let index = index.unwrap();
            regions.splice(index..=index, splice.into_iter());
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RegionTiling {
        Unknown,
        Red,
        Green,
        Background,
    }

    // Too lazy to refactor the past part of the function right now...
    let red_tiles = red_tiles
        .into_iter()
        .map(|(x, y)| Point::new_xy(x, y))
        .collect_vec();

    let mut sparse_grid = vec![RegionTiling::Unknown; y_regions.len() * x_regions.len()];
    let sparse_grid_size = &Point::new_xy(x_regions.len(), y_regions.len());

    let mut prev_corner = red_tiles.last().unwrap();
    let mut prev_corner_coords = Point::new_xy(0, 0);

    // TODO: Refactor into a function because I need it for checking the largest area...
    for (regions, val_coord, index_coord) in [
        (&x_regions, prev_corner.x, &mut prev_corner_coords.x),
        (&y_regions, prev_corner.y, &mut prev_corner_coords.y),
    ] {
        for (i, range) in regions.iter().enumerate() {
            if range.contains(&val_coord) {
                *index_coord = i;
            }
        }
    }

    let mut insides = vec![];

    for corner in red_tiles.iter() {
        let direc = match prev_corner.cmp_direc_pair(corner) {
            (None, Some(direc)) => direc,
            (Some(direc), None) => direc,
            (None, None) => unreachable!(),
            (Some(_), Some(_)) => unreachable!(),
        };
        let mut corner_coords = prev_corner_coords.clone();

        while !(x_regions[corner_coords.x].contains(&corner.x)
            && y_regions[corner_coords.y].contains(&corner.y))
        {
            corner_coords = corner_coords.next_point(&direc, sparse_grid_size).unwrap();

            let coloring = &mut sparse_grid[corner_coords.as_index(sparse_grid_size)];
            assert_eq!(coloring, &RegionTiling::Unknown);
            *coloring = RegionTiling::Green;

            insides.push(
                corner_coords
                    .next_point(&direc.rotate(-1), sparse_grid_size)
                    .unwrap(),
            )
        }

        let coloring = &mut sparse_grid[corner_coords.as_index(sparse_grid_size)];
        assert_eq!(coloring, &RegionTiling::Unknown);
        *coloring = RegionTiling::Red;

        prev_corner = corner;
        prev_corner_coords = corner_coords;
    }

    assert_eq!(sparse_grid[0], RegionTiling::Unknown);

    let to_flood_fill = insides
        .into_iter()
        .map(|index| (index, RegionTiling::Green))
        .chain([(Point::new_xy(0_usize, 0), RegionTiling::Background)]);
    for (coord, fill) in to_flood_fill {
        if fill == RegionTiling::Background {
            // If these asserts fail, that means the insides are not fully
            // contained by the 'borders' we set up in the previous step, a
            // previous flood fill spilled into the outside, and that's bad
            assert_eq!(coord, Point::new_xy(0, 0));
            assert_eq!(sparse_grid[0], RegionTiling::Unknown);
        }

        let mut remaining = vec![coord];
        while let Some(coord) = remaining.pop() {
            let index = coord.as_index(sparse_grid_size);
            if sparse_grid[index] != RegionTiling::Unknown {
                // Already handled, or was incorrectly marked as an inside above
                continue;
            }

            sparse_grid[index] = fill;

            for direc in &Direc::POWERS_OF_I {
                if let Some(adjacent) = coord.next_point(direc, sparse_grid_size){
                    if sparse_grid[adjacent.as_index(sparse_grid_size)] == RegionTiling::Unknown {
                        remaining.push(adjacent);
                    }
                }
            }
        }
    }

    for region in sparse_grid.iter() {
        assert_ne!(region, &RegionTiling::Unknown, "Missed a spot");
    }

    let tile_coords = red_tiles

    {
        // let mut max_area = 0;
        // for (index, a) in red_tiles.iter().enumerate() {
        //     for b in red_tiles[index + 1..].iter() {
        //         let area = (1 + a.x.abs_diff(b.x)) * (1 + a.y.abs_diff(b.y));
        //         if area <= max_area {
        //             continue;
        //         }

        //         let [x_range, y_range] = [[a.x, b.x], [a.y, b.y]].map(|mut pair| {
        //             pair.sort();
        //             pair[0]..=pair[1]
        //         });

        //         let mut prev_corner = red_tiles.last().unwrap();
        //         for corner in red_tiles.iter() {
        //             if corner.x <= *x_range.start()
        //                 || *x_range.end() <= corner.x
        //                 || corner.y <= *y_range.start()
        //                 || *y_range.end() <= corner.y
        //             {
        //                 // Tile (corner) is not completely inside the rectangle and
        //                 // therefore cannot prevent it being completely made of red
        //                 // or green tiles.
        //                 prev_corner = corner;
        //                 continue;
        //             }

        //             prev_corner = corner;
        //         }
        //     }
        // }

        // let mut by_x = HashMap::new();
        // let mut by_y = HashMap::new();

        // let &(mut prev_x, mut prev_y) = red_tiles.last().unwrap();
        // for (x, y) in red_tiles {
        //     if prev_x == x {
        //         let by_x = by_x.entry(x).or_insert_with(|| vec![]);
        //         if y < prev_y {
        //             by_x.push((y, prev_y))
        //         } else {
        //             by_x.push((prev_y, y))
        //         }
        //     } else if prev_y == y {
        //         let by_y = by_y.entry(y).or_insert_with(|| vec![]);
        //         if x < prev_x {
        //             by_y.push((x, prev_x))
        //         } else {
        //             by_y.push((prev_x, x))
        //         }
        //     } else {
        //         unreachable!()
        //     }

        //     (prev_x, prev_y) = (x, y)
        // }
    }

    todo!();
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day9.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    /// ..............
    /// .......0XXX1..
    /// .......XXXXX..
    /// ..6XXXX7XXXX..
    /// ..XXXXXXXXXX..
    /// ..5XXXXXX4XX..
    /// .........XXX..
    /// .........3X2..
    /// ..............
    const TEXT1: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 50);
    }

    #[test]
    fn part2_given_example() {
        assert_eq!(crate::part2(TEXT1), 24);
    }

    const SQUARE_WITH_EXTRA_CORNERS: &str = indoc! {"
        0,0
        0,2
        0,4
        1,4
        6,4
        6,0
    "};
    /// ```txt
    /// .......
    /// .0XXX1.
    /// .XXXXX.
    /// .7X6XX.
    /// ...XXX.
    /// .4X5XX.
    /// .XXXXX.
    /// .3XXX2.
    /// .......
    /// ```
    const U_SHAPE: &str = indoc! {"
        1,1
        5,1
        5,7
        1,7
        1,5
        3,5
        3,3
        1,3
    "};

    /// ```txt
    /// .......
    /// .0XXX1.
    /// .XXXXX.
    /// .7X6XX.
    /// .4X5XX.
    /// .XXXXX.
    /// .3XXX2.
    /// .......
    /// ```
    const TIGHT_U_SHAPE: &str = indoc! {"
        1,1
        5,1
        5,6
        1,6
        1,4
        3,4
        3,3
        1,3
    "};

    /// ```txt
    /// .......
    /// .0XXX#.
    /// .X5X6X.
    /// .X43XX.
    /// .1X2XX.
    /// .8XX7X.
    /// .XXXXX.
    /// .9XXX#.
    /// .......
    /// ```
    const INTESTINE: &str = indoc! {"
        1,1
        1,4
        3,4
        3,3
        2,3
        2,2
        4,2
        4,5
        1,5
        1,7
        5,7
        5,1
    "};

    #[rstest::rstest]
    #[case(SQUARE_WITH_EXTRA_CORNERS, 5 * 7)]
    #[case(U_SHAPE, 3 * 5)]
    #[case(TIGHT_U_SHAPE, 5 * 6)]
    #[case(INTESTINE, 5 * 7)]
    fn part2_custom_examples(#[case] text: &str, #[case] expected: usize) {
        assert_eq!(crate::part2(text), expected);
    }
}
