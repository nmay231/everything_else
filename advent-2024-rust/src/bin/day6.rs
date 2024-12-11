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

/// Pre-solve brainstorming: So, I have a couple ideas for how to solve this. 1)
/// Brute force. Just try every single position along the original path and
/// detect if the guard gets caught into a loop or leaves the grid. 2) I could
/// map out every possible turning spot next to an obstacle and detect any
/// pre-existing loops or setups that are one obstacle away from a loop. Then as
/// I travel through the normal path I see if an obstacle can redirect the guard
/// into one of those loops. I like this a lot less because setups that are one
/// away from a loop are much harder to detect, and the obstacle that creates
/// the loop must be the same one the redirects the guard from its original spot
/// to since you're only allowed to place one obstacle. That makes it basically
/// require the same logic as the brute force method while being more
/// complicated.
///
/// In fact, I had another "third" method that was not dissimilar to the second
/// but had a method of checking for potential loops by marking past movements
/// with arrows and detecting when walking over a trail of arrows going to the
/// right (because then you can place an obstacle there). However that still has
/// to share brute force logic for when certain cells are traversed up to 4 ways
/// in the same loop.
///
/// Overall, I think the only real optimization I can reasonably make is to
/// compress the grid into a set of columns and rows with the obstacles as
/// positions in order. That way you can avoid traversing every cell and move in
/// leaps and bounds.
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
