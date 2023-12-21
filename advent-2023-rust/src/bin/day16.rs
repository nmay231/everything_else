use std::collections::HashSet;

use advent_2023_rust::{Direc, UsizePoint};

type Output = usize;

fn beeeeaaaammm(grid: &[char], grid_size: &UsizePoint) -> usize {
    // Vec<(CurrentPoint, FromDirection)>
    let mut beams = vec![(UsizePoint(0, 0), Direc::East)];
    let mut visited = HashSet::new();
    while let Some((mut point, mut direc)) = beams.pop() {
        loop {
            if visited.contains(&(point, direc)) {
                break;
            }
            visited.insert((point, direc));

            let index = point.as_index(grid_size);
            let char = grid[index];

            direc = match (char, direc) {
                ('-' | '.', Direc::East) | ('/', Direc::North) | ('\\', Direc::South) => {
                    Direc::East
                }
                ('-' | '.', Direc::West) | ('\\', Direc::North) | ('/', Direc::South) => {
                    Direc::West
                }
                ('|' | '.', Direc::North) | ('/', Direc::East) | ('\\', Direc::West) => {
                    Direc::North
                }
                ('|' | '.', Direc::South) | ('\\', Direc::East) | ('/', Direc::West) => {
                    Direc::South
                }

                ('|', Direc::East | Direc::West) => {
                    beams.push((point, Direc::North));
                    Direc::South
                }
                ('-', Direc::North | Direc::South) => {
                    beams.push((point, Direc::East));
                    Direc::West
                }

                _ => panic!("Unexpected character in grid: '{char}' at {point:?}"),
            };

            point = match point.next_point(&direc, grid_size) {
                Some(next) => next,
                // Hit wall
                None => break,
            };
        }
    }

    return visited
        .iter()
        .map(|(point, _)| *point)
        .collect::<HashSet<_>>()
        .len();
}

fn part1(text: &str) -> Output {
    let grid_size = UsizePoint(text.lines().count(), text.lines().next().unwrap().len());
    let grid = text.trim().replace('\n', "").chars().collect::<Vec<_>>();
    return beeeeaaaammm(&grid, &grid_size);
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day16.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use advent_2023_rust::UsizePoint;
    use indoc::indoc;

    use crate::beeeeaaaammm;

    #[test]
    fn given_example() {
        let input = indoc! {r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."#};
        assert_eq!(
            beeeeaaaammm(
                &input.replace('\n', "").chars().collect::<Vec<_>>(),
                &UsizePoint(10, 10)
            ),
            46
        );
    }
}
