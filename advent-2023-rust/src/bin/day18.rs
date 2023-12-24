use std::cmp::Ordering;
use std::vec;

use advent_2023_rust::{Direc, UsizePoint};
use itertools::Itertools;

type Output = usize;

fn flood_fill(grid: &mut [char], grid_size: &UsizePoint, point: &UsizePoint, to: char) {
    // TODO: There is a better algorithm that spirals and only leaves seeds when a boundary is encountered, but I don't feel like doing that right now.
    let from = grid[point.as_index(grid_size)];
    let mut seeds = vec![*point];
    while let Some(seed) = seeds.pop() {
        if grid[seed.as_index(grid_size)] != from {
            continue;
        }
        grid[seed.as_index(grid_size)] = to;

        for direc in Direc::POWERS_OF_I {
            match seed.next_point_steps(1, &direc, grid_size) {
                Some(point) => {
                    seeds.push(point);
                }
                None => (),
            }
        }
    }
}

enum InsideOut {
    Inside,
    Outside,
    Wall,
}

fn is_inside(grid: &[char], grid_size: &UsizePoint, point: &UsizePoint, walls: char) -> InsideOut {
    if grid[point.as_index(grid_size)] == walls {
        return InsideOut::Wall;
    }

    let mut point = *point;
    let mut loop_start = None;
    let mut along_wall = Direc::North;
    let mut rotations = 0;

    loop {
        match (
            loop_start,
            point.next_point_steps(1, &along_wall, grid_size),
        ) {
            (_, None) => return InsideOut::Outside,
            (None, Some(adjacent)) => {
                if grid[adjacent.as_index(grid_size)] != walls {
                    point = adjacent;
                } else {
                    loop_start = Some(point);
                    along_wall = along_wall.rotate(1);
                    rotations += 1;
                }
            }
            (Some(start), Some(adjacent)) => {
                if point == start && rotations % 4 == 0 && rotations != 0 {
                    // We looped
                    return InsideOut::Inside;
                } else if grid[adjacent.as_index(grid_size)] == walls {
                    along_wall = along_wall.rotate(1);
                    rotations += 1;
                } else {
                    point = adjacent;
                    rotations -= 1;
                    along_wall = along_wall.rotate(-1);
                }
            }
        }
    }
}

type Path = Vec<(UsizePoint, Direc)>;

fn parse_path(text: &str, is_part1: bool) -> (Path, UsizePoint, isize) {
    let mut point = (0isize, 0isize);
    let mut path = vec![];
    let mut origin = point.to_owned();
    let mut grid_size = point.to_owned();

    let mut prev_direc = " ";
    let mut rotation = 0isize;

    for line in text.lines() {
        let (direc, steps) = if is_part1 {
            let (direc, line) = line.split_once(' ').unwrap();
            let (steps, _color) = line.split_once(' ').unwrap();
            (direc, steps.parse::<isize>().unwrap())
        } else {
            let (_, color) = line.rsplit_once(' ').unwrap();
            let color = &color[2..color.len() - 1]; // Remove (# ... ) from around the color
                                                    // let steps = ;
            const DIRECTIONS: [&str; 4] = ["R", "D", "L", "U"];
            let direc = DIRECTIONS[color[color.len() - 1..].parse::<usize>().unwrap()];
            (
                direc,
                isize::from_str_radix(&color[..color.len() - 1], 16).unwrap(),
            )
        };

        match direc {
            "U" => {
                path.push((point, Direc::North));
                point.0 -= steps;
                origin.0 = std::cmp::min(origin.0, point.0);
            }
            "D" => {
                path.push((point, Direc::South));
                point.0 += steps;
                grid_size.0 = std::cmp::max(grid_size.0, point.0);
            }
            "L" => {
                path.push((point, Direc::West));
                point.1 -= steps;
                origin.1 = std::cmp::min(origin.1, point.1);
            }
            "R" => {
                path.push((point, Direc::East));
                point.1 += steps;
                grid_size.1 = std::cmp::max(grid_size.1, point.1);
            }
            _ => panic!("Unexpected direction in puzzle input: {direc}"),
        };

        rotation += match (prev_direc, direc) {
            (" ", _) => 0,
            ("U", "R") | ("R", "D") | ("D", "L") | ("L", "U") => -1,
            ("R", "U") | ("U", "L") | ("L", "D") | ("D", "R") => 1,
            _ => panic!("Unexpected change in direction from '{prev_direc}' to '{direc}'"),
        };
        prev_direc = direc;
    }

    assert_eq!(point, (0, 0));

    let grid_size = UsizePoint(
        (grid_size.0 - origin.0 + 1) as usize,
        (grid_size.1 - origin.1 + 1) as usize,
    );
    let path = path
        .into_iter()
        .map(|(point, direc)| {
            (
                UsizePoint((point.0 - origin.0) as usize, (point.1 - origin.1) as usize),
                direc,
            )
        })
        .collect::<Vec<_>>();

    let last_direc = path.last().unwrap().1;
    rotation += if last_direc.rotate(1) == path[0].1 {
        1
    } else {
        -1
    };
    assert_eq!(rotation.abs(), 4);

    (path, grid_size, rotation / 4)
}

fn part1(path: Path, ref grid_size: UsizePoint, _rotate_inside: isize) -> Output {
    let mut grid = vec!['.'; grid_size.0 * grid_size.1];

    // Draw the outline
    for tmp in [&path, &path[..1]].concat().windows(2) {
        if let [(mut point, direc), (next_point, _)] = tmp {
            while point != *next_point {
                grid[point.as_index(grid_size)] = '#';
                point = point.next_point_steps(1, &direc, grid_size).unwrap();
            }
        }
    }

    // for row_i in 0..grid_size.0 {
    //     println!(
    //         "{}",
    //         String::from_iter(&grid[row_i * grid_size.1..(row_i + 1) * grid_size.1])
    //     );
    // }
    // println!();

    // Inverse fill
    while let Some((index, _)) = grid.iter().find_position(|char| **char == '.') {
        let point = UsizePoint(index / grid_size.1, index % grid_size.1);
        match is_inside(&grid, grid_size, &point, '#') {
            InsideOut::Wall => panic!("Just asserted the char is a '.'"),
            InsideOut::Outside => flood_fill(&mut grid, grid_size, &point, '_'),
            InsideOut::Inside => flood_fill(&mut grid, grid_size, &point, '#'),
        }
    }

    // for row_i in 0..grid_size.0 {
    //     println!(
    //         "{}",
    //         String::from_iter(&grid[row_i * grid_size.1..(row_i + 1) * grid_size.1])
    //     );
    // }
    // println!();
    return grid.iter().filter(|char| char == &&'#').count();
}

fn length_between(a: usize, b: usize) -> usize {
    a.abs_diff(b) + 1
}

fn furthest_point_along(direc: &Direc, a: &UsizePoint, b: &UsizePoint) -> UsizePoint {
    std::cmp::max_by(UsizePoint(a.0, b.1), UsizePoint(b.0, a.1), |a, b| {
        direc.cmp_points(a, b)
    })
}

// TODO: Of course my first implementation was not gonna generalize. Fudge.
fn part2(mut path: Path, ref _grid_size: UsizePoint, rotate_inside: isize) -> Output {
    let rotate_inside = rotate_inside as i32;
    let mut area: usize = 0;
    let mut neg_area: usize = 0;

    'outer: while path.len() > 4 {
        println!("path.len()={}", path.len());

        for (index, (prev, a, b, c, d, next)) in
            path.clone().iter().circular_tuple_windows().enumerate()
        {
            if (a.1.rotate(1) == b.1 && b.1.rotate(1) == c.1)
                || (a.1.rotate(-1) == b.1 && b.1.rotate(-1) == c.1)
            {
                let nub_neck = std::cmp::max_by(a.0, d.0, |x, y| a.1.cmp_points(x, y));
                let nub_head = std::cmp::max_by_key(b.0, c.0, |x| nub_neck.manhattan_distance(x));
                let diameter = nub_neck.manhattan_distance(&nub_head);

                // Search for points that visit inside of this box we're attempting to shrink
                for (point, _) in path.iter().cycle().skip(index + 6).take(path.len() - 6) {
                    let diff_sum =
                        nub_neck.manhattan_distance(point) + nub_head.manhattan_distance(point);
                    if diff_sum <= diameter {
                        assert_eq!(diff_sum, diameter, "They should always be equal or greater, but I never use `==` as a check in loop");
                        // There are points inside the box, skip for now
                        continue 'outer;
                    }
                }

                let nub_area =
                    length_between(nub_neck.0, nub_head.0) * length_between(nub_neck.1, nub_head.1);
                let dividing_edge_area;

                // Hopefully this example helps with the following match statement
                // (prev.1 == b.1, d.1 == b.1, a.1 == Direc::North and a.0 is more north than d.0)
                // => (true, false, Ordering::Greater)
                // .......b>--c....    .......*****....
                // .......|...V....    .......*****....
                // .......|...|....    .......*****....
                // .......^...|.... => .......*****.... => area += 20
                // prev>--a...|....    prev>------c'....
                // ...........|....    ...........V....
                // ...next---<d....    ...next---<d....

                // Remove excess points
                match (prev.1 == b.1, d.1 == b.1, a.1.cmp_points(&a.0, &d.0)) {
                    (true, true, Ordering::Equal) => {
                        // Remove a, b, c, d
                        path.splice(index + 1..index + 5, []);
                        // Do not double count the edge from a to d
                        dividing_edge_area = a.0.manhattan_distance(&d.0) + 1;
                    }
                    (true, false, Ordering::Equal) => {
                        // Remove a, b, c, d
                        path.splice(index + 1..index + 5, []);
                        // Do not double count the edge from a to next
                        dividing_edge_area = a.0.manhattan_distance(&next.0) + 1;
                    }
                    (false, true, Ordering::Equal) => {
                        // Remove prev..d, replace prev with (prev.0, b.1)
                        path.splice(index..index + 5, [(prev.0, b.1)]);
                        dividing_edge_area = prev.0.manhattan_distance(&d.0) + 1;
                    }
                    // etc.
                    (false, false, Ordering::Equal) => {
                        path.splice(index..index + 5, [(prev.0, b.1)]);
                        dividing_edge_area = prev.0.manhattan_distance(&next.0) + 1;
                    }

                    (_, true, Ordering::Less) => {
                        let new_b0 = furthest_point_along(&a.1, &a.0, &d.0);
                        path.splice(index + 2..index + 5, [(new_b0, b.1)]);
                        dividing_edge_area = new_b0.manhattan_distance(&d.0) + 1;
                    }
                    (_, false, Ordering::Less) => {
                        let new_b0 = furthest_point_along(&a.1, &a.0, &d.0);
                        path.splice(index + 2..index + 5, [(new_b0, b.1)]);
                        dividing_edge_area = new_b0.manhattan_distance(&next.0) + 1;
                    }

                    (true, _, Ordering::Greater) => {
                        let new_c0 = furthest_point_along(&a.1, &a.0, &d.0);
                        path.splice(index + 1..index + 4, [(new_c0, c.1)]);
                        dividing_edge_area = a.0.manhattan_distance(&new_c0) + 1;
                    }
                    (false, _, Ordering::Greater) => {
                        let new_c0 = furthest_point_along(&a.1, &a.0, &d.0);
                        path.splice(
                            index..index + 4,
                            [(prev.0, prev.1.rotate(2)), (new_c0, c.1)],
                        );
                        dividing_edge_area = prev.0.manhattan_distance(&new_c0) + 1;
                    }
                }
                if a.1.rotate(rotate_inside) == b.1 {
                    // Turning in means the nub is an out-ty; aka positive area
                    area += nub_area - dividing_edge_area;
                } else {
                    //                N = nub_area;  P=perimeter; E=dividing_edge; 2=shared corners; new_area += 6
                    // #####.......   #####.......   #####.......   #####.......   #####.......    #####.......
                    // #####.......   #####.......   #####.......   #####.......   #####.......    #####.......
                    // #B#<A....... = #NNNN....... - #PPPP....... + ####E....... - ####2....... => #####.......
                    // #V..........   #NNNN.......   #P..P.......   ##..E.......   ##..........    ##***.......
                    // ##.......^>#   #NNNN....###   #P..P....###   ##..E....###   ##.......###    ##***....###
                    // #C>######D##   #NNNN#######   #PPPP#######   ####E#######   ####2#######    ############
                    // ############   ############   ############   ############   ############    ############
                    let perimeter_area = 2 * (nub_neck.manhattan_distance(&nub_head));
                    neg_area += nub_area - perimeter_area + dividing_edge_area - 2;
                }
                for (a, b) in path.iter().circular_tuple_windows() {
                    assert!(a.0 .0 == b.0 .0 || a.0 .1 == b.0 .1);
                    assert!(a.1 == b.1.rotate(1) || a.1 == b.1.rotate(-1));
                }
                continue 'outer; // We continue 'outer instead of break to detect infinite loops
            }
        }

        unreachable!(
            "A finite path should have some consecutive pairs of repeated turns (aka a nub)"
        );
    }

    let (corner1, corner2) = (path[0].0, path[2].0);
    area += length_between(corner1.0, corner2.0) * length_between(corner1.1, corner2.1);
    return area - neg_area;
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day18.txt")?;

    let (path, grid_size, rotate_inside) = parse_path(&text, true);
    println!(
        "part 1 result = {:?}",
        part1(path, grid_size, rotate_inside)
    );
    let (path, grid_size, rotate_inside) = parse_path(&text, false);
    println!(
        "part 2 result = {:?}",
        part2(path, grid_size, rotate_inside)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rstest::*;

    enum TestWhich {
        Part1,
        Part2,
        // TODO: Useless
        // BothEqual,
    }

    use crate::{parse_path, part1, part2};

    #[rstest]
    #[case::true_true_equal("R1;U1;R1;D1;R1;D1;L3;U1", 10)]
    #[case::true_false_equal("R1;U1;R2;D1;L1;D1;L2;U1", 10)]
    #[case::false_true_equal("L1;U1;R2;D1;R1;D1;L2;U1", 10)]
    #[case::false_false_equal("L1;U1;R3;D1;L1;D1;L1;U1", 10)]
    #[case::true_true_less("R1;U2;R1;D1;R1;D2;L3;U1", 13)]
    #[case::false_true_less("L1;U2;R1;D1;R1;D2;L1;U1", 10)]
    #[case::true_false_less("R1;U2;R2;D1;L1;D2;L2;U1", 12)]
    #[case::false_false_less("L1;U2;R3;D1;L2;D1", 10)]
    #[case::true_true_greater("L1;U1;L1;D2;L1;D1;R3;U2", 13)]
    #[case::false_true_greater("L1;U1;L1;D2;R1;D1;R1;U2", 10)]
    #[case::true_false_greater("R1;U1;L2;D2;L1;D1;R2;U2", 12)]
    #[case::false_false_greater("R2;U1;L3;D2;R1;U1", 10)]
    #[case::fake_negative_area("R1;D1;R1;U1;R1;D2;L3;U2", 12)]
    #[case::actual_negative_area("R1;D2;R2;U2;R1;D3;L4;U3", 18)]
    #[case::first_example("R6;D5;L2;D2;R2;D2;L5;U2;L1;U2;R2;U3;L2;U2", 62)]
    fn test_part1_and_part2(
        #[case] instructions: &str,
        #[case] expected: usize,
        #[values(TestWhich::Part1, TestWhich::Part2)] test_which: TestWhich,
    ) {
        // Transform compacted instructions into part1-compatible text and parse
        let text = &instructions
            .split(';')
            .map(|step| [&step[..1], " ", &step[1..], " (#fake_color_here)"].join(""))
            .join("\n");
        let (path, grid_size, rotate_inside) = parse_path(text, true);

        match test_which {
            TestWhich::Part1 => assert_eq!(part1(path, grid_size, rotate_inside), expected),
            TestWhich::Part2 => assert_eq!(part2(path, grid_size, rotate_inside), expected),
            // TestWhich::BothEqual => assert_eq!(
            //     part1(path.to_owned(), grid_size.to_owned(), rotate_inside),
            //     part2(path, grid_size, rotate_inside),
            // ),
        }
    }
}
