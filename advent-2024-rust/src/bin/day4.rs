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
    0
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

    use crate::part1;

    #[test]
    fn test_part1_example() {
        let text = indoc! {"
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
        assert_eq!(part1(text), 18);
    }
}
