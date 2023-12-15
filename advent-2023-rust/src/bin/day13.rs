type Output = usize;

fn find_horizontal_mirror_line(rows: &[&str]) -> Option<usize> {
    for (index, pair) in rows.windows(2).enumerate() {
        if pair[0] == pair[1] {
            let mut is_mirror = true;
            for inner_index in 1..=std::cmp::min(index, (rows.len() - index).saturating_sub(2)) {
                if rows[index - inner_index] != rows[index + inner_index + 1] {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                return Some(index + 1);
            }
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

        sum += find_horizontal_mirror_line(&transposed)
            .or_else(|| find_horizontal_mirror_line(&lines).and_then(|x| Some(100 * x)))
            .unwrap_or(0);
    }

    return sum;
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day13.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::find_horizontal_mirror_line;

    #[test]
    fn small_mirror() {
        let input = "
##.###.
##.###.
        ";
        assert_eq!(
            find_horizontal_mirror_line(&input.trim().split("\n").collect::<Vec<_>>()),
            Some(1)
        );
    }

    #[test]
    fn small_not_mirror() {
        let input = "
##.#.#.
##.###.
        ";
        assert_eq!(
            find_horizontal_mirror_line(&input.trim().split("\n").collect::<Vec<_>>()),
            None
        );
    }

    #[test]
    fn small_mirror_extras_above() {
        let input = "
#.##.#.
##.###.
##.###.
        ";
        assert_eq!(
            find_horizontal_mirror_line(&input.trim().split("\n").collect::<Vec<_>>()),
            Some(2)
        );
    }

    #[test]
    fn small_mirror_extras_below() {
        let input = "
##.###.
##.###.
#.##.#.
        ";
        assert_eq!(
            find_horizontal_mirror_line(&input.trim().split("\n").collect::<Vec<_>>()),
            Some(1)
        );
    }

    #[test]
    fn not_mirror_extras_above() {
        let input = "
#.#..#.
#.##.#.
##.###.
##.###.
#.#..#.
        ";
        assert_eq!(
            find_horizontal_mirror_line(&input.trim().split("\n").collect::<Vec<_>>()),
            None
        );
    }

    #[test]
    fn not_mirror_extras_below() {
        let input = "
#.##.#.
##.###.
##.###.
#.#..#.
#.##.#.
        ";
        assert_eq!(
            find_horizontal_mirror_line(&input.trim().split("\n").collect::<Vec<_>>()),
            None
        );
    }
}
