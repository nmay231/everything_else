type Output = usize;

fn find_horizontal_mirror_line(rows: &[&str], allowed_smudges: usize) -> Option<usize> {
    'outer: for row_i in 0..rows.len() - 1 {
        let mut smudges = allowed_smudges;

        let max_reach = std::cmp::min(row_i, (rows.len() - row_i).saturating_sub(2));
        for reach in 0..=max_reach {
            let (a, b) = (rows[row_i - reach], rows[row_i + reach + 1]);
            if a != b {
                if smudges == 0 {
                    continue 'outer; // This line is not the mirroring line, check the next one
                }
                let diffs = a
                    .chars()
                    .zip(b.chars())
                    .map(|(x1, x2)| if x1 == x2 { 0 } else { 1 })
                    .sum::<usize>();

                if diffs <= smudges {
                    smudges -= diffs;
                } else {
                    continue 'outer; // We went over our alloted number of smudges, check next mirror line
                }
            }
        }

        if smudges == 0 {
            return Some(row_i + 1);
        }
    }
    None
}

fn transpose(rows: &[&str]) -> Vec<String> {
    return (0..rows[0].len())
        .map(|col_i| {
            String::from_iter(
                rows.iter()
                    .map(|row| row.chars().nth(col_i).expect("rows to be the same length")),
            )
        })
        .collect::<Vec<_>>();
}

fn part1(text: &str) -> Output {
    let mut sum = 0;
    let blocks = text.trim().split("\n\n").collect::<Vec<_>>();

    for block in blocks {
        let lines = block.lines().collect::<Vec<_>>();
        let tmp = transpose(&lines);
        let transposed = tmp.iter().map(|string| string.as_str()).collect::<Vec<_>>();

        sum += find_horizontal_mirror_line(&transposed, 0)
            .or_else(|| find_horizontal_mirror_line(&lines, 0).and_then(|x| Some(100 * x)))
            .unwrap_or(0);
    }

    return sum;
}

fn part2(text: &str) -> Output {
    // TODO: I guess I do the same thing but allow exactly one difference in characters
    let mut sum = 0;
    let blocks = text.trim().split("\n\n").collect::<Vec<_>>();

    for block in blocks {
        let lines = block.lines().collect::<Vec<_>>();
        let tmp = transpose(&lines);
        let transposed = tmp.iter().map(|string| string.as_str()).collect::<Vec<_>>();

        sum += find_horizontal_mirror_line(&transposed, 1)
            .or_else(|| find_horizontal_mirror_line(&lines, 1).and_then(|x| Some(100 * x)))
            .unwrap_or(0);
    }

    return sum;
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day13.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    // https://www.reddit.com/r/adventofcode/comments/18hitog/2023_day_13_easy_additional_examples/
    const INPUT: &str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.
    ";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(INPUT.trim()), 709);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(INPUT.trim()), 1400);
    }
}
