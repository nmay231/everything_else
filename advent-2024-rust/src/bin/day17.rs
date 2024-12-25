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

        match opcode {
            // The adv instruction (opcode 0) performs division. The numerator
            // is the value in the A register. The denominator is found by
            // raising 2 to the power of the instruction's combo operand. (So,
            // an operand of 2 would divide A by 4 (2^2); an operand of 5 would
            // divide A by 2^B.) The result of the division operation is
            // truncated to an integer and then written to the A register.
            0 => {
                registers[A] /= 2_usize.pow(interpret_combo_operand(operand, registers) as u32);
                head += 2;
            }
            // The bxl instruction (opcode 1) calculates the bitwise XOR of
            // register B and the instruction's literal operand, then stores the
            // result in register B.
            1 => {
                registers[B] ^= operand;
                head += 2;
            }
            // The bst instruction (opcode 2) calculates the value of its combo
            // operand modulo 8 (thereby keeping only its lowest 3 bits), then
            // writes that value to the B register.
            2 => {
                registers[B] = 0b111 & interpret_combo_operand(operand, registers);
                head += 2;
            }
            // The jnz instruction (opcode 3) does nothing if the A register is
            // 0, However, if the A register is not zero, it jumps by setting
            // the instruction pointer to the value of its literal operand; if
            // this instruction jumps, the instruction pointer is not increased
            // by 2 after this instruction.
            3 => {
                if registers[A] == 0 {
                    head += 2;
                } else {
                    head = operand;
                }
            }
            // The bxc instruction (opcode 4) calculates the bitwise XOR of
            // register B and register C, then stores the result in register B.
            // (For legacy reasons, this instruction reads an operand but
            // ignores it.)
            4 => {
                registers[B] ^= registers[C];
                head += 2;
            }
            // The out instruction (opcode 5) calculates the value of its combo
            // operand modulo 8, then outputs that value. (If a program outputs
            // multiple values, they are separated by commas.)
            5 => {
                out.push(0b111 & interpret_combo_operand(operand, registers));
                head += 2;
            }
            // The bdv instruction (opcode 6) works exactly like the adv
            // instruction except that the result is stored in the B register.
            // (The numerator is still read from the A register.)
            6 => {
                registers[B] =
                    registers[A] / 2_usize.pow(interpret_combo_operand(operand, registers) as u32);
                head += 2;
            }
            // The cdv instruction (opcode 7) works exactly like the adv
            // instruction except that the result is stored in the C register.
            // (The numerator is still read from the A register.)
            7 => {
                registers[C] =
                    registers[A] / 2_usize.pow(interpret_combo_operand(operand, registers) as u32);
                head += 2;
            }

            opcode => panic!("Unexpected opcode {}", opcode),
        }
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
}
