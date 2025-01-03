use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

type Output = usize;

enum Operator {
    And,
    Or,
    Xor,
}

fn part1(text: &str) -> Output {
    let mut stage = 0_usize;
    let mut wire_values = HashMap::new();
    let mut gates = VecDeque::new();

    // Note: I am only using for_each because I realized that it de-nests the
    // match statement by a level compared to a regular for-loop
    text.lines().for_each(|line| match (stage, line) {
        (2.., _) => unreachable!("There are only two stages in the input file"),
        (0, "") => stage += 1,
        (0, line) => {
            let (gate, bit) = line.split_once(": ").unwrap();
            wire_values.insert(gate, bit == "1");
        }
        (1, line) => {
            let (a, op, b, _arrow, c) = line.split(' ').collect_tuple().unwrap();
            let op = match op {
                "AND" => Operator::And,
                "OR" => Operator::Or,
                "XOR" => Operator::Xor,
                _ => unreachable!("Unknown operator: {:?}", op),
            };
            gates.push_back((a, b, op, c));
        }
    });

    // TODO: Assuming that all gates eventually get values
    while let Some((a, b, op, c)) = gates.pop_front() {
        match (wire_values.get(a), wire_values.get(b)) {
            (Some(a), Some(b)) => {
                wire_values.insert(
                    c,
                    match op {
                        Operator::And => a & b,
                        Operator::Or => a | b,
                        Operator::Xor => a ^ b,
                    },
                );
            }
            _ => gates.push_back((a, b, op, c)),
        }
    }

    let mut result = 0;
    for (wire, value) in wire_values.into_iter() {
        if !wire.starts_with('z') {
            continue;
        }
        let bit_shift = wire[1..].parse::<usize>().unwrap();
        result |= (value as usize) << bit_shift;
    }

    return result;
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day24.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    "};

    const TEXT2: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[rstest::rstest]
    #[case(TEXT1, 0b100)]
    #[case(TEXT2, 0b0011111101000)]
    fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
        assert_eq!(crate::part1(text), expected);
    }
}
