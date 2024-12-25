use itertools::Itertools;

type Output = String;

fn interpret_combo_operand(operand: usize, registers: [usize; 3]) -> usize {
    match operand {
        // Combo operands 0 through 3 represent literal values 0 through 3.
        0 | 1 | 2 | 3 => operand,
        // Combo operand 4 represents the value of register A.
        // Combo operand 5 represents the value of register B.
        // Combo operand 6 represents the value of register C.
        4 | 5 | 6 => registers[operand - 4],
        // Combo operand 7 is reserved and will not appear in valid programs.
        7 => panic!("Part 2?"),
        8.. => unreachable!(),
    }
}

fn part1(text: &str) -> Output {
    let mut registers = [0_usize; 3];
    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;

    let mut subtext = text;
    for reg_i in 0..3 {
        let start = 2 + subtext.find(": ").unwrap();
        let end = subtext.find('\n').unwrap();
        registers[reg_i] = subtext[start..end].parse().unwrap();
        subtext = &subtext[end + 1..];
    }

    let (_, instructions) = text.trim().split_once("\n\n").unwrap();
    let instructions = instructions.strip_prefix("Program: ").unwrap();
    let instructions = instructions
        .split(',')
        .map(|d| d.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut head = 0;
    let mut out = vec![];
    loop {
        if head >= instructions.len() {
            break;
        }
        assert!(head < instructions.len() - 1);

        let opcode = instructions[head];
        let operand = instructions[head + 1];

        // Condensed summary
        // 0, 6, 7: copy a modified A into A, B, or C respectively.
        // 1, 4: XOR either the literal operand or C (respectively) into B
        // 2: truncate the combo operand and move into B
        // 5: print out combo operator
        // 3: Jump to operand if A != 0
        match opcode {
            // 0, 6, 7: copy a modified A into A, B, or C respectively.
            0 => registers[A] /= 2_usize.pow(interpret_combo_operand(operand, registers) as u32),
            6 => {
                registers[B] =
                    registers[A] / 2_usize.pow(interpret_combo_operand(operand, registers) as u32);
            }
            7 => {
                registers[C] =
                    registers[A] / 2_usize.pow(interpret_combo_operand(operand, registers) as u32);
            }
            // 1, 4: XOR either the literal operand or C (respectively) into B
            1 => registers[B] ^= operand,
            4 => registers[B] ^= registers[C],
            // 2: truncate the combo operand and move into B
            2 => registers[B] = 0b111 & interpret_combo_operand(operand, registers),
            // 5: print out combo operator
            5 => out.push(0b111 & interpret_combo_operand(operand, registers)),
            // 3: Jump to operand if A != 0
            3 => {
                if registers[A] != 0 {
                    head = operand;
                    continue;
                }
            }
            8.. => panic!("Unexpected opcode {}", opcode),
        }
        head += 2;
    }

    return out.iter().join(",");
}

fn part2(_text: &str) -> Output {
    String::new()
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day17.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), "4,6,3,5,6,3,5,2,1,0");
    }

    const TEXT2: &str = indoc! {"
        Register A: 117440
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};
    #[test]
    fn part2a_quine() {
        assert_eq!(crate::part1(TEXT2), "0,3,5,4,3,0");
    }
}
