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

fn part1(text: &str) -> Output {
    let mut point = (0i32, 0i32);
    let mut path = vec![];
    let mut origin = point.to_owned();
    let mut grid_size = point.to_owned();

    let mut prev_direc = " ";
    let mut rotation = 0i32;

    for line in text.lines() {
        let (direc, line) = line.split_once(' ').unwrap();
        let (steps, _color) = line.split_once(' ').unwrap();
        let steps = steps.parse::<i32>().unwrap();
        let _color = &_color[2.._color.len() - 1]; // Remove (# ... ) from around the color

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
    // While the loop closes, you don't get 4 90-deg turns because the last turn is "implicit"
    assert_eq!(rotation.abs(), 3);

    let path = path
        .into_iter()
        .map(|(point, direc)| {
            (
                UsizePoint((point.0 - origin.0) as usize, (point.1 - origin.1) as usize),
                direc,
            )
        })
        .collect::<Vec<_>>();
    let grid_size = &UsizePoint(
        (grid_size.0 - origin.0 + 1) as usize,
        (grid_size.1 - origin.1 + 1) as usize,
    );
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

// TODO: Of course my first implementation is not gonna generalize. Fudge.
fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day18.txt")?;

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
