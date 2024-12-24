use std::collections::{HashSet, VecDeque};

use advent_2024_rust::{Direc, UsizePoint};
use itertools::Itertools;

type Output = usize;

#[derive(Debug, PartialEq, Eq)]
enum CellPart1 {
    Wall,
    Empty,
    Box,
}

impl From<CellPart1> for bool {
    fn from(value: CellPart1) -> Self {
        value == CellPart1::Box
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CellPart2 {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
}

impl From<CellPart2> for bool {
    fn from(value: CellPart2) -> Self {
        value == CellPart2::BoxLeft
    }
}

// TODO: It probably would've made more sense to convert into a list of numbered
// weights, but it's not like there's a part 3 to this puzzle... Right?
fn grid_score(grid: &[bool], grid_size: &UsizePoint) -> Output {
    grid.iter()
        .enumerate()
        .filter_map(|(index, cell)| {
            if !cell {
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
                    '#' => CellPart1::Wall,
                    '.' => CellPart1::Empty,
                    'O' => CellPart1::Box,
                    '@' => {
                        assert!(player.is_none(), "Friends are not allowed >:D");
                        player = Some(UsizePoint(line_index, index));
                        CellPart1::Empty
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
            CellPart1::Wall => (),
            CellPart1::Empty => player = next_step,
            CellPart1::Box => {
                let step1 = next_step.clone();
                loop {
                    next_step = next_step
                        .next_point(&direc, grid_size)
                        .expect(assume_outer_wall);

                    let grid_i = next_step.as_index(grid_size);
                    match grid[grid_i] {
                        CellPart1::Wall => break,
                        CellPart1::Empty => {
                            grid[grid_i] = CellPart1::Box;
                            grid[step1.as_index(grid_size)] = CellPart1::Empty;
                            player = step1;
                            break;
                        }
                        CellPart1::Box => (),
                    }
                }
            }
        }
    }

    let grid: Vec<bool> = grid.into_iter().map(Into::into).collect();
    grid_score(&grid, grid_size)
}

fn _debug_print_grid_part2(
    grid: &[CellPart2],
    grid_size: &UsizePoint,
    player: &UsizePoint,
    highlights: &[UsizePoint],
) {
    let player = player.as_index(grid_size);
    let highlights = highlights
        .iter()
        .map(|p| p.as_index(grid_size))
        .collect::<HashSet<_>>();

    for (i, cell) in grid.iter().enumerate() {
        if i == player {
            print!("@");
            continue;
        }
        let (prefix, postfix) = if highlights.contains(&i) {
            ("\x1b[0;32m", "\x1b[0m")
        } else {
            ("", "")
        };

        let cell = match cell {
            CellPart2::Wall => '#',
            CellPart2::Empty => '.',
            CellPart2::BoxLeft => '{',
            CellPart2::BoxRight => '}',
        };
        print!("{}{}{}", prefix, cell, postfix);
        if (i + 1) % grid_size.1 == 0 {
            println!();
        }
    }
}

fn part2(text: &str) -> Output {
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
                grid.extend(line.char_indices().flat_map(|(index, c)| match c {
                    '#' => [CellPart2::Wall, CellPart2::Wall],
                    '.' => [CellPart2::Empty, CellPart2::Empty],
                    'O' => [CellPart2::BoxLeft, CellPart2::BoxRight],
                    '@' => {
                        assert!(player.is_none(), "Friends are not allowed >:D");
                        player = Some(UsizePoint(line_index, 2 * index));
                        [CellPart2::Empty, CellPart2::Empty]
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

    let width = 2 * text.find('\n').expect("there to be more than one line");
    let grid_size = &UsizePoint(grid.len() / width, width);

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

        match (direc, &grid[next_step.as_index(grid_size)]) {
            // Me telling a joke to my friends
            (_, CellPart2::Wall) => (),
            // How I feel inside
            (_, CellPart2::Empty) => player = next_step,
            // Impossible situations (Fine, I'll lay off the edgy jokes)
            (Direc::East, CellPart2::BoxRight) | (Direc::West, CellPart2::BoxLeft) => {
                unreachable!("Help! I'm stuck in a box (move '{}' at index {}", mov, i);
            }
            // Simple sideways motion
            (Direc::East | Direc::West, box_side) => {
                let step1 = next_step.clone();
                let mut n_boxes = 1;
                loop {
                    next_step = next_step
                        .next_point_steps(2, &direc, grid_size)
                        .expect(assume_outer_wall);

                    let grid_i = next_step.as_index(grid_size);
                    match &grid[grid_i] {
                        CellPart2::Wall => break,
                        CellPart2::Empty => {
                            // Bubble sort the empty cell from the end to where
                            // the player will be
                            let range: Box<dyn Iterator<Item = usize>> =
                                match grid_i < step1.as_index(grid_size) {
                                    true => Box::new(grid_i..grid_i + 2 * n_boxes),
                                    false => Box::new((grid_i - 2 * n_boxes..grid_i).rev()),
                                };

                            for i in range {
                                grid.swap(i, i + 1);
                            }
                            player = step1;
                            break;
                        }
                        new_box_side @ (CellPart2::BoxLeft | CellPart2::BoxRight) => {
                            n_boxes += 1;
                            assert_eq!(new_box_side, box_side); // Sanity check
                        }
                    }
                }
            }
            // Cascading vertical motion
            (Direc::North | Direc::South, CellPart2::BoxLeft | CellPart2::BoxRight) => {
                let step1 = next_step;
                let mut to_check = VecDeque::from([next_step]);
                let mut to_move = vec![];

                while let Some(pos) = to_check.pop_front() {
                    let is_box_left = match grid[pos.as_index(grid_size)] {
                        CellPart2::Wall => {
                            to_move = vec![];
                            break;
                        }
                        CellPart2::Empty => continue,
                        CellPart2::BoxLeft => true,
                        CellPart2::BoxRight => false,
                    };

                    let next_pos = pos.next_point(&direc, grid_size).expect(assume_outer_wall);
                    if is_box_left {
                        let right = pos.next_point(&Direc::East, grid_size).unwrap();
                        assert_eq!(CellPart2::BoxRight, grid[right.as_index(grid_size)]);
                        to_move.extend_from_slice(&[pos, right]);

                        to_check.extend([
                            next_pos,
                            next_pos
                                .next_point(&Direc::East, grid_size)
                                .expect(assume_outer_wall),
                        ]);
                    } else {
                        let left = pos.next_point(&Direc::West, grid_size).unwrap();
                        assert_eq!(CellPart2::BoxLeft, grid[left.as_index(grid_size)]);
                        to_move.extend_from_slice(&[pos, left]);

                        to_check.extend([
                            next_pos,
                            next_pos
                                .next_point(&Direc::West, grid_size)
                                .expect(assume_outer_wall),
                        ]);
                    }
                }

                to_move = to_move.into_iter().unique().collect();

                if to_move.len() > 0 {
                    player = step1;
                }

                // Swap boxes with empty positions starting from the far end
                while let Some(to_move) = to_move.pop() {
                    grid.swap(
                        to_move.as_index(grid_size),
                        to_move
                            .next_point(&direc, grid_size)
                            .unwrap()
                            .as_index(grid_size),
                    );
                }
            }
        }
    }

    let grid: Vec<bool> = grid.into_iter().map(Into::into).collect();
    grid_score(&grid, grid_size)
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day15.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
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

    #[rstest]
    // #[case(TEXT1, 2028)]
    #[case(TEXT2, 9021)]
    fn part2_given_example(#[case] text: &str, #[case] expected: usize) {
        assert_eq!(part2(text), expected);
    }
}
