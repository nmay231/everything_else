type Output = usize;

fn part1(text: &str) -> Output {
    let mut total = 0;

    'lines: for line in text.lines() {
        let (test_value, operands) = line
            .split_once(": ")
            // TODO: I wish there was a more convenient method to provide
            // formatted panic messages
            .unwrap_or_else(|| panic!("line did not contain colon separator: `{}`", line));
        let test_value: usize = test_value
            .parse()
            .unwrap_or_else(|err| panic!("{:?} was not a usize: {}", test_value, err));
        let operands = operands
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()
            .unwrap_or_else(|err| {
                panic!(
                    "expected {:?} to be space-separated ints: {}",
                    operands, err
                )
            });

        for mut index in 0..usize::pow(2, operands.len() as u32 - 1) {
            let mut partial = operands[0];
            for operand in operands.iter().skip(1) {
                let bit = index & 1;
                index = index >> 1;
                if bit == 0 {
                    partial += operand;
                } else {
                    partial *= operand;
                }
            }

            assert_eq!(index, 0);
            if partial == test_value {
                total += partial;
                continue 'lines;
            }
        }
    }

    total
}

fn part2(text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day7.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use itertools::Itertools;
    use rand::{Rng, SeedableRng};

    use crate::part1;

    const TEXT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};
    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT), 3749)
    }

    #[test]
    fn part1_random() {
        // This doesn't test against false positives, but it was enough to help
        // me fix the code.
        let mut rng = rand::rngs::StdRng::seed_from_u64(816542);
        let mut numbers =
            (&mut rng).sample_iter(rand::distributions::Uniform::new_inclusive(0_usize, 0xffff));

        let mut total_test_values = 0;
        let mut text = String::new();
        for _ in 0..30 {
            let mut test_value = numbers.next().unwrap();
            let mut max_allowed = test_value;
            let mut operands = vec![test_value.to_string()];

            for _ in 0..(0b111 & numbers.next().unwrap()) {
                let n = numbers.next().unwrap();
                let bit = n & 1;
                let n = n >> 1;

                // Technically, I could make the real code handle overflows, but
                // the AoC host is nice enough to prevent those situations, so
                // I'll be lazy in my solution (and keep it simple).
                if let Some(max) = max_allowed.checked_mul(n) {
                    max_allowed = max;
                } else {
                    break;
                }
                operands.push(n.to_string());

                if bit == 0 {
                    test_value += n;
                } else {
                    test_value *= n;
                };
            }

            total_test_values += test_value;
            text.push_str(&format!("{}: {}\n", test_value, operands.iter().join(" ")));
        }

        assert_eq!(part1(&text), total_test_values);
    }
}
