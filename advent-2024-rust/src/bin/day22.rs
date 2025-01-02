#![feature(iter_map_windows)]
use std::collections::HashMap;

type Output = usize;

fn forward_one(mut input: usize) -> usize {
    // Calculate the result of multiplying the secret number by 64. Then, mix this
    // result into the secret number. Finally, prune the secret number.
    input = ((input * 64) ^ input) % 16777216;

    // Calculate the result of dividing the secret number by 32. Round the result
    // down to the nearest integer. Then, mix this result into the secret
    // number. Finally, prune the secret number.
    input = ((input / 32) ^ input) % 16777216;

    // Calculate the result of multiplying the secret number by
    // 2048. Then, mix this result into the secret number. Finally, prune the secret
    // number.
    input = ((input * 2048) ^ input) % 16777216;

    return input;
}

fn part1(text: &str) -> Output {
    let mut total = 0;

    for line in text.lines() {
        let mut n = line.parse::<usize>().unwrap();
        for _ in 0..2000 {
            n = forward_one(n);
        }
        total += n;
    }

    total
}

struct MonkeyBuyer {
    secret_number: usize,
}

impl MonkeyBuyer {
    fn new(secret_number: usize) -> Self {
        Self { secret_number }
    }
}

impl Iterator for MonkeyBuyer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.secret_number = forward_one(self.secret_number);
        Some(self.secret_number)
    }
}

fn part2(text: &str) -> Output {
    let mut global_sequence_increases = HashMap::new();
    for line in text.lines() {
        let n = line.parse::<usize>().unwrap();

        let sequence = [n]
            .into_iter()
            .chain(MonkeyBuyer::new(n))
            .take(2001)
            .map(|x| x % 10)
            .map_windows(|[a, b]| (*b as isize - *a as isize, *b))
            .map_windows(|[a, b, c, d]| ((a.0, b.0, c.0, d.0), d.1));

        let mut first_sell = HashMap::new();
        for (diffs, selling_price) in sequence {
            first_sell.entry(diffs).or_insert(selling_price);
        }

        for (diffs, selling_price) in first_sell.into_iter() {
            *global_sequence_increases.entry(diffs).or_default() += selling_price;
        }
    }

    global_sequence_increases.into_values().max().unwrap()
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day22.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        1
        10
        100
        2024
    "};

    const TEXT2: &str = indoc! {"
        1
        2
        3
        2024
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 37327623);
    }

    #[test]
    fn forward_20() {
        let mut result = vec![];
        let mut n = 123;
        for _ in 0..10 {
            n = crate::forward_one(n);
            result.push(n);
        }

        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_given_example() {
        assert_eq!(crate::part2(TEXT2), 23);
    }
}
