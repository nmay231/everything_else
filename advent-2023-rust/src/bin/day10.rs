type Output = usize;

type Grid = Vec<String>;

/// (row, column)
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum Direc {
    North,
    East,
    South,
    West,
}

impl Direc {
    fn follow_path(&self, grid_char: char) -> Result<Self, String> {
        match (self, grid_char) {
            (Direc::North, '7') => Ok(Direc::West),
            (Direc::North, '|') => Ok(Direc::North),
            (Direc::North, 'F') => Ok(Direc::East),

            (Direc::East, 'J') => Ok(Direc::North),
            (Direc::East, '-') => Ok(Direc::East),
            (Direc::East, '7') => Ok(Direc::South),

            (Direc::South, 'L') => Ok(Direc::East),
            (Direc::South, '|') => Ok(Direc::South),
            (Direc::South, 'J') => Ok(Direc::West),

            (Direc::West, 'F') => Ok(Direc::South),
            (Direc::West, '-') => Ok(Direc::West),
            (Direc::West, 'L') => Ok(Direc::North),

            _ => Err(format!(
                "Expected a traversable terrain: {self:?}, {grid_char}"
            )),
        }
    }
}

fn part1(text: &str) -> Output {
    let grid = text.lines().map(String::from).collect::<Grid>();
    let grid_size = Point(grid.len(), grid[0].len());
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.find("S").map(|column| Point(i, column)))
        .unwrap();

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
    return 0;
}

fn part2(_text: &str) -> Output {
    // TODO: Keeping debug set up since I think I'll need it for part 2.

    // let mut debug_grid = vec![vec!['.'; grid_size.0]; grid_size.1];
    // debug_grid[start.0][start.1] = 'S';

    // let tmp = direc.follow_path(grid_char);
    // if tmp.is_err() {
    //     debug_grid[point.0][point.1] = 'O';
    //     println!("{direc:?}, {point:?}, {grid_char}");
    //     break;
    // }
    // let prev_direc = direc.to_owned();
    // direc = tmp.unwrap();
    // let tmp = point.next_point(&direc, &grid_size);
    // if tmp.is_none() {
    //     debug_grid[point.0][point.1] = 'O';
    //     println!("{prev_direc:?}, {point:?}, {grid_char}, {direc:?}");
    //     break;
    // }
    // point = tmp.unwrap();
    // debug_grid[point.0][point.1] = grid_char;
    0
    // for row in debug_grid {
    //     println!("{}", String::from_iter(row.iter()));
    // }
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
