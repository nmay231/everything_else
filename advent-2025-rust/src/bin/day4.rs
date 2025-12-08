use advent_2025_rust::{Direc, Point};

fn part1(text: &str) -> usize {
    let grid_size = &Point::<usize>::new_xy(text.find('\n').unwrap(), text.lines().count());
    let grid = text.chars().filter(|c| c != &'\n').collect::<Vec<_>>();

    let mut total = 0;

    for index in 0..grid.len() {
        if grid[index] != '@' {
            continue;
        }

        let point = Point::<usize>::from_index(grid_size, index);
        let mut neighbor_count = 0;

        'neighbors: for path in Direc::EIGHT_WAYS {
            let mut point = point;

            for direc in path {
                point = match point.next_point(direc, grid_size) {
                    Some(p) => p,
                    None => continue 'neighbors,
                };
            }

            if grid[point.as_index(grid_size)] == '@' {
                neighbor_count += 1;
            }
        }

        if neighbor_count < 4 {
            total += 1;
        }
    }

    total
}

fn part2(_text: &str) -> usize {
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

    const TEXT1: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 13);
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
