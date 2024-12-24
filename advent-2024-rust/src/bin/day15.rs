use advent_2024_rust::{Direc, UsizePoint};

type Output = usize;

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Box,
}

fn grid_score(grid: &[Cell], grid_size: &UsizePoint) -> Output {
    grid.iter()
        .enumerate()
        .filter_map(|(index, cell)| {
            if cell != &Cell::Box {
                return None;
            }
            let position = UsizePoint::from_index(grid_size, index);
            return Some(100 * position.0 + position.1);
        })
        .sum()
}

fn part1(text: &str) -> Output {
    let mut grid = vec![];
    let mut player = None;
    let mut moves = String::new();
    let mut stage = 0_u8;

    for (line_index, line) in text.lines().enumerate() {
        match stage {
            2.. => unreachable!("There should only be two stages"),
            // Newline separator
            0 if line == "" => stage += 1,
            // Parse grid
            0 => {
                grid.extend(line.char_indices().map(|(index, c)| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::Box,
                    '@' => {
                        assert!(player.is_none(), "Friends are not allowed >:D");
                        player = Some(UsizePoint(line_index, index));
                        Cell::Empty
                    }
                    _ => panic!("Unexpected character '{}' at {}:{}", c, line_index, index),
                }));
            }
            // Parse moves
            1 => moves.push_str(line),
        }
    }

    let mut player = player.expect("player character '@' never found");
    assert_eq!(stage, 1);

    let width = text.find('\n').expect("there to be more than one line");
    let grid_size = &UsizePoint(width, grid.len() / width);

    for (i, mov) in moves.char_indices() {
        let direc = match mov {
            '^' => Direc::North,
            '>' => Direc::East,
            'v' => Direc::South,
            '<' => Direc::West,
            _ => panic!("Unexpected character '{}' at move index {}", mov, i),
        };

        let assume_outer_wall = "There should be a wall buffer around the edges of the grid";
        let mut next_step = player
            .next_point(&direc, grid_size)
            .expect(assume_outer_wall);

        match grid[next_step.as_index(grid_size)] {
            Cell::Wall => (),
            Cell::Empty => player = next_step,
            Cell::Box => {
                let step1 = next_step.clone();
                loop {
                    next_step = next_step
                        .next_point(&direc, grid_size)
                        .expect(assume_outer_wall);

                    let grid_i = next_step.as_index(grid_size);
                    match grid[grid_i] {
                        Cell::Wall => break,
                        Cell::Empty => {
                            grid[grid_i] = Cell::Box;
                            grid[step1.as_index(grid_size)] = Cell::Empty;
                            player = step1;
                            break;
                        }
                        Cell::Box => (),
                    }
                }
            }
        }
    }

    grid_score(&grid, grid_size)
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day15.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;
    use rstest::rstest;

    const TEXT1: &str = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    const TEXT2: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[rstest]
    #[case(TEXT1, 2028)]
    #[case(TEXT2, 10092)]
    fn part1_given_example(#[case] text: &str, #[case] expected: usize) {
        assert_eq!(part1(text), expected);
    }
}
