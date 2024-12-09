use advent_2024_rust::{Direc, UsizePoint};

type Output = usize;

fn part1(text: &str) -> Output {
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    let grid = text.replace('\n', "").trim().chars().collect::<Vec<_>>();
    let mut total = 0;

    for (i, c) in grid.iter().enumerate() {
        if c != &'X' {
            continue;
        }

        let x_marks_the_spot = UsizePoint::from_index(grid_size, i);
        'direcs: for direcs in Direc::EIGHT_WAYS {
            let mut checking = x_marks_the_spot.clone();
            for next_char in ['M', 'A', 'S'] {
                for direc in direcs {
                    checking = match checking.next_point(direc, grid_size) {
                        None => continue 'direcs,
                        Some(x) => x,
                    }
                }
                if grid[checking.as_index(grid_size)] != next_char {
                    continue 'direcs;
                }
            }

            total += 1;
        }
    }
    total
}

fn part2(text: &str) -> Output {
    let grid_size = &UsizePoint(text.lines().count(), text.find('\n').unwrap());
    let grid = text.replace('\n', "").trim().chars().collect::<Vec<_>>();
    let mut total = 0;

    'next_char: for (i, c) in grid.iter().enumerate() {
        if c != &'A' {
            continue;
        }

        let corners = [
            [Direc::East, Direc::North],
            [Direc::West, Direc::North],
            [Direc::West, Direc::South],
            [Direc::East, Direc::South],
        ];

        let center = UsizePoint::from_index(grid_size, i);
        for i in 0..=1 {
            let opposing = [corners[i], corners[(i + 2) % 4]];
            let points = opposing
                .iter()
                .map(|[a, b]| {
                    center
                        .clone()
                        .next_point(a, grid_size)
                        .and_then(|horizontal| horizontal.next_point(b, grid_size))
                })
                .collect::<Option<Vec<_>>>();

            match points {
                None => continue 'next_char,
                Some(opposite) => {
                    if let [a, b] = opposite[0..2] {
                        let a = grid[a.as_index(grid_size)];
                        let b = grid[b.as_index(grid_size)];
                        if [a, b] != ['M', 'S'] && [b, a] != ['M', 'S'] {
                            continue 'next_char;
                        }
                    } else {
                        panic!("Only two items should be possible.")
                    }
                }
            }
        }
        total += 1;
    }

    total
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day4.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{part1, part2};

    const TEXT: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(TEXT), 18);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(TEXT), 9);
    }
}
