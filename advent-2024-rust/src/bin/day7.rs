use anyhow::{Context, Result};
use itertools::Itertools;

type Output = usize;

fn parse_line(line: &str) -> Result<(usize, Vec<usize>)> {
    let (test_value, operands) = line
        .split_once(": ")
        .with_context(|| format!("line did not contain colon separator: `{}`", line))?;
    let test_value: usize = test_value
        .parse()
        .with_context(|| format!("{:?} was not a usize", test_value))?;
    let operands = operands
        .split(' ')
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .with_context(|| format!("expected {:?} to be space-separated ints", operands))?;

    return Ok((test_value, operands));
}

fn part1(text: &str) -> Output {
    let mut total = 0;

    'lines: for line in text.lines() {
        let (test_value, operands) = parse_line(line).unwrap();

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

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Mul,
    Concat,
}

fn part2(text: &str) -> Output {
    let mut total = 0;

    'lines: for line in text.lines() {
        let (test_value, operands) = parse_line(line).unwrap();

        let all_operations = (1..operands.len())
            .map(|_| [Op::Plus, Op::Mul, Op::Concat].iter())
            .multi_cartesian_product();

        let mut x = 0;
        for operations in all_operations {
            assert_eq!(operations.len(), operands.len() - 1);
            x += 1;

            let mut partial = operands[0];

            for (op, num) in operations.iter().zip(operands.iter().skip(1)) {
                match op {
                    Op::Plus => partial += *num,
                    Op::Mul => partial *= *num,
                    Op::Concat => {
                        // TODO: Test which concat method is faster
                        // partial = format!("{}{}", partial, num).parse().unwrap();
                        let amount = num.to_string().len();
                        partial = partial * usize::pow(10, amount as u32) + *num;
                    }
                }
            }
            if partial == test_value {
                total += test_value;
                continue 'lines;
            }
        }

        assert_eq!(x, usize::pow(3, operands.len() as u32 - 1));
    }
    return total;
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

    use crate::{part1, part2};

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

    #[test]
    fn part2_given_example() {
        assert_eq!(part2(TEXT), 11387);
    }
}
