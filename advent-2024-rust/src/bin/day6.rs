use advent_2024_rust::{Direc, UsizePoint};
use itertools::Itertools;

type Output = usize;
const VISITED: char = 'X';
const WALL: char = '#';
const EMPTY: char = '.';
const GUARD: char = '^';

fn part1(text: &str) -> Output {
    let grid_size = &UsizePoint(text.find('\n').unwrap(), text.lines().count());
    let mut grid = text.chars().filter(|c| c != &'\n').collect::<Vec<_>>();
    let guard = grid
        .iter()
        .find_position(|c| c == &&GUARD)
        .expect("No guard (^) found")
        .0;
    grid[guard] = VISITED;

    let mut guard = UsizePoint::from_index(grid_size, guard);
    let mut direc = Direc::North;

    while let Some(ahead) = guard.next_point(&direc, grid_size) {
        // TODO: I didn't realize until I started writing the match statement,
        // but this has the same problem that doing `use Enum::*;` does.
        // Deleting the WALL constant, for example, could become a logic bug if
        // missed in a large refactor (or more likely incorrectly refactored).
        match grid[ahead.as_index(grid_size)] {
            EMPTY | VISITED => {
                guard = ahead;
                grid[guard.as_index(grid_size)] = VISITED;
            }
            WALL => direc = direc.rotate(-1),
            c => panic!("Unexpected char found '{}'", c),
        }
    }

    grid.into_iter().filter(|c| c == &VISITED).count()
}

fn part2(text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day6.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::part1;

    const TEXT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT), 41);
    }
}
