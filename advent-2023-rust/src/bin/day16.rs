use std::collections::HashSet;

use advent_2023_rust::{Direc, UsizePoint};

type Output = usize;

fn beeeeaaaammm(grid: &[char], grid_size: &UsizePoint, start: (UsizePoint, Direc)) -> usize {
    // Vec<(CurrentPoint, FromDirection)>
    let mut beams = vec![start];
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
    return beeeeaaaammm(&grid, &grid_size, (UsizePoint(0, 0), Direc::East));
}

fn part2(text: &str) -> Output {
    let grid_size = UsizePoint(text.lines().count(), text.lines().next().unwrap().len());
    let grid = text.trim().replace('\n', "").chars().collect::<Vec<_>>();

    let mut max = 0;
    for col in 0..grid_size.1 {
        for (row, direc) in [(0, Direc::South), (grid_size.0 - 1, Direc::North)] {
            let amount = beeeeaaaammm(&grid, &grid_size, (UsizePoint(row, col), direc));
            assert!(amount > 1); // Sanity check
            max = std::cmp::max(max, amount);
        }
    }
    for row in 0..grid_size.0 {
        for (col, direc) in [(0, Direc::East), (grid_size.1 - 1, Direc::West)] {
            let amount = beeeeaaaammm(&grid, &grid_size, (UsizePoint(row, col), direc));
            assert!(amount > 1); // Sanity check
            max = std::cmp::max(max, amount);
        }
    }
    max
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day16.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use advent_2023_rust::{Direc, UsizePoint};
    use indoc::indoc;

    use crate::{beeeeaaaammm, part2};

    const INPUT: &str = indoc! {r#"
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

    #[test]
    fn first_example() {
        assert_eq!(
            beeeeaaaammm(
                &INPUT.replace('\n', "").chars().collect::<Vec<_>>(),
                &UsizePoint(10, 10),
                (UsizePoint(0, 0), Direc::East),
            ),
            46,
        );
    }

    #[test]
    fn best_example() {
        assert_eq!(
            beeeeaaaammm(
                &INPUT.replace('\n', "").chars().collect::<Vec<_>>(),
                &UsizePoint(10, 10),
                (UsizePoint(0, 3), Direc::South),
            ),
            51,
        );
    }

    #[test]
    fn can_find_best_example() {
        assert_eq!(part2(INPUT), 51);
    }
}
