type Output = usize;

const DEBUGGING: bool = false;
macro_rules! debug_println {
    () => {
        if DEBUGGING {
            print!("\n")
        }
    };
    ($($arg:tt)*) => {{
        if DEBUGGING {
            println!("{}", format!($($arg)*))
        }
    }};
}

/// (row, column)
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn next_point(&self, direc: &Direc, grid_size: &Point) -> Option<Point> {
        match direc {
            Direc::North => {
                if self.0 > 0 {
                    Some(Point(self.0 - 1, self.1))
                } else {
                    None
                }
            }
            Direc::East => {
                if self.1 + 1 < grid_size.1 {
                    Some(Point(self.0, self.1 + 1))
                } else {
                    None
                }
            }
            Direc::South => {
                if self.0 + 1 < grid_size.0 {
                    Some(Point(self.0 + 1, self.1))
                } else {
                    None
                }
            }
            Direc::West => {
                if self.1 > 0 {
                    Some(Point(self.0, self.1 - 1))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direc {
    North,
    East,
    South,
    West,
}

impl Direc {
    fn follow_path(&self, grid_char: char) -> Result<Self, String> {
        self.follow_path_with_rotation(grid_char)
            .and_then(|(direc, _)| Ok(direc))
    }

    fn follow_path_with_rotation(&self, grid_char: char) -> Result<(Self, i32), String> {
        match (self, grid_char) {
            (Direc::North, '7') => Ok((Direc::West, 1)),
            (Direc::North, '|') => Ok((Direc::North, 0)),
            (Direc::North, 'F') => Ok((Direc::East, -1)),

            (Direc::East, 'J') => Ok((Direc::North, 1)),
            (Direc::East, '-') => Ok((Direc::East, 0)),
            (Direc::East, '7') => Ok((Direc::South, -1)),

            (Direc::South, 'L') => Ok((Direc::East, 1)),
            (Direc::South, '|') => Ok((Direc::South, 0)),
            (Direc::South, 'J') => Ok((Direc::West, -1)),

            (Direc::West, 'F') => Ok((Direc::South, 1)),
            (Direc::West, '-') => Ok((Direc::West, 0)),
            (Direc::West, 'L') => Ok((Direc::North, -1)),

            _ => Err(format!(
                "Expected a traversable terrain: {self:?}, {grid_char}"
            )),
        }
    }

    const POWERS_OF_I: [Direc; 4] = [Direc::East, Direc::North, Direc::West, Direc::South];

    fn rotate(&self, rotation_counter_clockwise: i32) -> Self {
        return Direc::POWERS_OF_I[((rotation_counter_clockwise
            + Direc::POWERS_OF_I
                .iter()
                .enumerate()
                .find_map(|(i, x)| if x == self { Some(i as i32) } else { None })
                .unwrap()) as usize)
            .rem_euclid(4)];
    }
}

fn part1(text: &str) -> Output {
    let grid = text.lines().map(String::from).collect::<Vec<_>>();
    let grid_size = Point(grid.len(), grid[0].len());
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.find("S").map(|column| Point(i, column)))
        .unwrap();

    // Find the starting direction of the pipe loop
    // TODO: Funny enough this code doesn't work in general. It only works because the first direc is north which does connect to the starting point in the main example. This is fixed in part2
    let (mut direc, mut point) = vec![Direc::North, Direc::East, Direc::South, Direc::West]
        .iter()
        .filter_map(|direc| {
            start
                .next_point(direc, &grid_size)
                .and_then(|point| Some((*direc, point)))
        })
        .next()
        .unwrap();

    for steps in 0 as usize.. {
        let grid_char = grid[point.0].chars().nth(point.1).unwrap();
        if grid_char == 'S' {
            return (steps + 1) / 2;
        }
        direc = direc.follow_path(grid_char).unwrap();
        point = point.next_point(&direc, &grid_size).unwrap();
    }
    unreachable!("Looping path did not actually loop!");
}

fn part2(text: &str) -> Output {
    let grid = text
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let grid_size = Point(grid.len(), grid[0].len());
    let original_start = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, char)| **char == 'S')
                .map(|(column, _)| Point(i, column))
        })
        .unwrap();

    // We find a point connected to the start and the direc from the start to that point
    let (mut direc, mut point) = Direc::POWERS_OF_I
        .iter()
        .filter_map(|direc| {
            original_start
                .next_point(direc, &grid_size)
                .and_then(|point| {
                    direc
                        .follow_path(grid[point.0][point.1])
                        .and(Ok((*direc, point)))
                        .ok()
                })
        })
        .next()
        .unwrap();

    let start_direc = direc.clone();
    let start_point = point.clone();

    let mut empty_grid = vec![vec!['.'; grid_size.0]; grid_size.1];
    let mut rotations = 0;
    let mut loop_length = 1;
    for steps in 0 as usize.. {
        let grid_char = grid[point.0][point.1];
        if grid_char == 'S' {
            loop_length = steps + 1;
            break;
        }
        empty_grid[point.0][point.1] = grid_char;

        let rot;
        (direc, rot) = direc.follow_path_with_rotation(grid_char).unwrap();
        point = point.next_point(&direc, &grid_size).unwrap();

        rotations += rot;
    }
    assert!(loop_length != 1);

    let start_char = "LF7J-|"
        .chars()
        .find(|char| {
            direc
                .follow_path(*char)
                .map(|next_direc| point.next_point(&next_direc, &grid_size) == Some(start_point))
                == Ok(true)
        })
        .unwrap();

    empty_grid[original_start.0][original_start.1] = start_char;
    let mut grid = empty_grid;

    for row in &grid {
        debug_println!("{}", String::from_iter(row.iter()));
    }
    debug_println!();

    assert!(
        rotations.abs() == 4,
        "Loop must be topologically isomorphic to a circle (or square in this case)"
    );
    let look_inside = start_direc.rotate(rotations / 4);

    direc = start_direc;
    point = start_point;
    let mut rotations = 0;
    let mut total_cells_inside = 0;
    for steps in 0..loop_length {
        debug_println!(
            "{}; {:?}; {:?}; {}",
            steps,
            direc,
            point,
            grid[point.0][point.1]
        );
        let mut looks_inward = rotations % 4 == 0;

        let rot;
        (direc, rot) = direc
            .follow_path_with_rotation(grid[point.0][point.1])
            .unwrap();
        rotations += rot;

        looks_inward = looks_inward || rotations % 4 == 0;

        if looks_inward {
            let mut tmp_point = point.to_owned();
            for number_inside in 0.. {
                tmp_point = tmp_point.next_point(&look_inside, &grid_size).unwrap();
                if grid[tmp_point.0][tmp_point.1] != '.' {
                    total_cells_inside += number_inside;
                    break;
                } else if grid[tmp_point.0][tmp_point.1] == 'O' {
                    panic!("Visiting cells twice");
                }
                grid[tmp_point.0][tmp_point.1] = 'O';
            }
        }

        point = point.next_point(&direc, &grid_size).unwrap();
    }

    for row in &grid {
        debug_println!(
            "{}",
            String::from_iter(row.iter().map(|c| if *c == 'O' { 'O' } else { '.' }))
        );
    }

    return total_cells_inside;
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day10.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        assert!(true);
    }
}
