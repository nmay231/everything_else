type Output = usize;

fn part1(text: &str) -> Output {
    let mut count = 0;
    for line in text.lines() {
        let levels: Result<Vec<isize>, _> =
            line.split_ascii_whitespace().map(|x| x.parse()).collect();
        let levels = levels.expect("Err");
        let diff = levels
            .iter()
            .skip(1)
            .zip(&levels)
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();

        if !(diff.iter().all(|x| x > &0) || diff.iter().all(|x| x < &0)) {
            continue;
        } else if !diff.iter().all(|x| x.abs_diff(0) <= 3) {
            continue;
        }

        count += 1;
    }

    return count;
}

fn part2(text: &str) -> Output {
    let mut count = 0;
    for (line_i, line) in text.lines().enumerate() {
        let levels: Result<Vec<isize>, _> =
            line.split_ascii_whitespace().map(|x| x.parse()).collect();
        let levels = levels.expect(&format!("Error on line {}", line_i));

        // I reverse the order since I suspect it's much more common to use all
        // the levels than to omit one. Of course, that's a guess and therefore
        // likely to be the root of all evil, aka premature optimization.
        for i in (0..=levels.len()).rev() {
            let levels = if i == levels.len() {
                &levels
            } else {
                &[&levels[0..i], &levels[i + 1..]].concat()
            };

            let diff = levels
                .iter()
                .skip(1)
                .zip(levels)
                .map(|(a, b)| a - b)
                .collect::<Vec<_>>();

            // On the topic of optimization, it's possible to run a few checks
            // to "early return" on certain sequences that are clearly
            // impossible to satisfy even with one omission. For example, a diff
            // >3 in the middle of the sequence would only get worse when
            // removing a neighbor (assuming the next diff doesn't offset it the
            // other way), e.g. `1 2 7 8 9 => -1 -5 -1 -1` is invalid. However,
            // I think of how likely that is to help compared to when it would
            // just take more time than the brute force with no complex logic.
            // Similar to the contiguous array vs linked-list (mis)optimization.
            if !(diff.iter().all(|x| x > &0) || diff.iter().all(|x| x < &0)) {
                continue;
            } else if !diff.iter().all(|x| x.abs_diff(0) <= 3) {
                continue;
            }

            count += 1;
            break;
        }
    }

    return count;
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day2.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn sample() {
        let text = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "#
        .trim();
        assert_eq!(2, part1(text));
        assert_eq!(4, part2(text));
    }
}
