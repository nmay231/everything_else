type Output = usize;

fn part1(text: &str) -> Output {
    let mut lines = text.lines().peekable();

    let mut load = text.lines().count();
    let row_length = lines.peek().unwrap().len();
    let mut loads = vec![load; row_length];

    let mut total_load = 0;
    for line in lines {
        load -= 1;
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => loads[col] = load,
                'O' => {
                    total_load += loads[col];
                    loads[col] -= 1;
                }
                '.' => (),
                _ => panic!("Unexpected char in puzzle input: '{char}'"),
            }
        }
    }

    total_load
}

struct Grid {
    grid: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(string: &str) -> Self {
        let rows = string.lines().count();
        let grid = string.trim().replace("\n", "").chars().collect::<Vec<_>>();
        let cols = grid.len() / rows;
        Self { grid, cols, rows }
    }
}

fn roll_vertically(grid: &mut Grid, north: bool) {
    let Grid {
        ref mut grid,
        ref rows,
        ref cols,
    } = grid;

    let mut roll = |ledges: &mut Vec<usize>, row: usize| {
        for col in 0..*cols {
            let index = row * cols + col;
            match grid[index] {
                '.' => (),
                '#' => match north {
                    true => ledges[col] = row + 1,
                    false => ledges[col] = row.saturating_sub(1),
                },
                'O' => {
                    grid[index] = '.';
                    grid[cols * ledges[col] + col] = 'O';
                    match north {
                        true => ledges[col] += 1,
                        false => ledges[col] = ledges[col].saturating_sub(1),
                    }
                }
                _ => panic!("Unexpected char in puzzle input: '{}'", grid[index]),
            }
        }
    };
    if north {
        let mut ledges = vec![0 as usize; *cols];
        for row in 0..*rows {
            roll(&mut ledges, row);
        }
    } else {
        let mut ledges = vec![rows - 1; *cols];
        for row in (0..*rows).rev() {
            roll(&mut ledges, row);
        }
    }
}

fn roll_horizontally(grid: &mut Grid, west: bool) {
    let Grid {
        ref mut grid,
        ref rows,
        ref cols,
    } = grid;

    let mut roll = |ledges: &mut Vec<usize>, col: usize| {
        for row in 0..*rows {
            let index = row * cols + col;
            match grid[index] {
                '.' => (),
                '#' => match west {
                    true => ledges[row] = col + 1,
                    false => ledges[row] = col.saturating_sub(1),
                },
                'O' => {
                    grid[index] = '.';
                    grid[cols * row + ledges[row]] = 'O';
                    match west {
                        true => ledges[row] += 1,
                        false => ledges[row] = ledges[row].saturating_sub(1),
                    }
                }
                _ => panic!("Unexpected char in puzzle input: '{}'", grid[index]),
            }
        }
    };
    if west {
        let mut ledges = vec![0 as usize; *rows];
        for col in 0..*cols {
            roll(&mut ledges, col);
        }
    } else {
        let mut ledges = vec![cols - 1; *rows];
        for col in (0..*cols).rev() {
            roll(&mut ledges, col);
        }
    }
}

fn _debug_grid(grid: &Grid) {
    for row in (0..grid.grid.len()).step_by(grid.cols) {
        println!("{}", String::from_iter(&grid.grid[row..row + grid.cols]));
    }
    println!();
}

fn count_load(grid: &str) -> usize {
    let mut load = grid.lines().count();
    return grid
        .lines()
        .map(|row| {
            let row_load = row.chars().filter(|char| *char == 'O').count() * load;
            load -= 1;
            return row_load;
        })
        .sum();
}

fn part2(text: &str) -> Output {
    let mut grid = Grid::new(text);
    let mut history = vec![];
    let mut non_repeat_length = 0;
    let mut cycle_length = 0;
    for step in 0.. {
        roll_vertically(&mut grid, true);
        roll_horizontally(&mut grid, true);
        roll_vertically(&mut grid, false);
        roll_horizontally(&mut grid, false);
        if let Some(prev) =
            history
                .iter()
                .enumerate()
                .find_map(|(i, prev)| if prev == &grid.grid { Some(i) } else { None })
        {
            println!("==================== END {prev} ====================");
            non_repeat_length = prev;
            cycle_length = step - prev;
            break;
        }
        history.push(grid.grid.to_owned());
    }

    let index = (1_000_000_000 - non_repeat_length - 1) % cycle_length + non_repeat_length;
    return count_load(
        &history[index]
            .chunks(grid.cols)
            .map(String::from_iter)
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day14.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{part2, roll_horizontally, roll_vertically, Grid};

    #[test]
    fn rock_and_roll() {
        let tmp: Vec<&str> = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....

            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....

            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O

            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O"}
        .split("\n\n")
        .collect();

        let input = tmp[0];
        let after_1_cycle = tmp[1].replace("\n", "").chars().collect::<Vec<char>>();
        let after_2_cycle = tmp[2].replace("\n", "").chars().collect::<Vec<char>>();
        let after_3_cycle = tmp[3].replace("\n", "").chars().collect::<Vec<char>>();
        let mut grid = Grid::new(input);

        roll_vertically(&mut grid, true);
        roll_horizontally(&mut grid, true);
        roll_vertically(&mut grid, false);
        roll_horizontally(&mut grid, false);
        assert_eq!(grid.grid, after_1_cycle);

        roll_vertically(&mut grid, true);
        roll_horizontally(&mut grid, true);
        roll_vertically(&mut grid, false);
        roll_horizontally(&mut grid, false);
        assert_eq!(grid.grid, after_2_cycle);

        roll_vertically(&mut grid, true);
        roll_horizontally(&mut grid, true);
        roll_vertically(&mut grid, false);
        roll_horizontally(&mut grid, false);
        assert_eq!(grid.grid, after_3_cycle);
    }

    #[test]
    fn part2_cycles_in_2_offset_0() {
        // The X is the assumed ending position
        let input = indoc! {"
            ............................
            ...........................#
            .#..........................
            ............................
            ............................
            ............................
            ............................
            ............................
            ............................
            ............................
            ....................#.......
            ...................#O.......
            ............................
            .........................#..
            ..#.................#.......
            ...........................X"};
        assert_eq!(part2(&input.replace("X", ".")), 1);
    }

    #[test]
    fn part2_cycles_in_2_offset_1() {
        let input = indoc! {"
            ............................
            ...........................#
            .#..........................
            ............................
            ............................
            ............................
            ............................
            ..............#.............
            ........#...................
            .............#..............
            ............#O.#............
            .............#..............
            ............................
            ........................X#..
            ..#......#..................
            ............................"};
        assert_eq!(part2(&input.replace("X", ".")), 3);
    }

    #[test]
    fn part2_cycles_in_3_offset_0() {
        let input = indoc! {"
            ............................
            .................#..........
            ........#...................
            ...............#............
            ..........#.................
            ..............#.............
            ............#..X#...........
            .............#..............
            ..................#.........
            ...........#................
            ...............#............
            .........#..................
            ............................
            ............................
            .........#..................
            ..............O............."};
        assert_eq!(part2(&input.replace("X", ".")), 10);
    }

    #[test]
    fn part2_cycles_in_3_offset_1() {
        let input = indoc! {"
            ............................
            .................#..........
            ........#...................
            ...............#............
            ..........#.................
            ..............#.............
            ............#...#...........
            .............#..............
            ..................#.........
            ...........#................
            ..............X#............
            .........#..................
            ............................
            ............................
            .........#..................
            ...............#...........O"};
        assert_eq!(part2(&input.replace("X", ".")), 6);
    }

    #[test]
    fn part2_cycles_in_3_offset_2() {
        let input = indoc! {"
            ............................
            .................#..........
            ........#...................
            ...............#............
            ..........#.................
            ..............#.............
            ............#...#...........
            .............#..............
            .................X#.........
            ...........#................
            ...............#............
            .........#..................
            ............................
            ............................
            .........#...............#..
            ...............#........#O.."};
        assert_eq!(part2(&input.replace("X", ".")), 8);
    }
}
