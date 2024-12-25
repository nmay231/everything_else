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

/// I think my strategy for part2 will be to imitate the training of a Neural
/// Network. So, we will go through the instructions of the program one number
/// at a time and back-propagate the possible values as iterators based on the
/// instructions in the program. Once we get to the first instruction, we
/// eliminate any that contradict the initial values of B and C. This then
/// generates possible values of A. We can then repeat with the second number in
/// the instructions and find out which values of A intersect with that. Doing
/// this for the first two or three instructions should already limit the
/// possible values by quite a bit. In which case we can just start brute
/// forcing the possible A's based on that iterator.
///
/// However, I don't know how much better than simple brute force that will be.
/// Perhaps I could be more clever if I'm smart enough to do it. Here's what I
/// mean. If we have to generate 2 as the first number, then B must be of the
/// form `8k + 2`. Then based on the third to last instruction ("decompiled" to
/// `B = B ^ 6`) then it must be `8k + 4`. Then based on the fourth to last
/// instruction `B ^ C = 8k + 4`.
///
/// Okay, I've looked at the instructions more closely and realized that the
/// final output is basically based on the value of each group of 3 bits, so I
/// think I can manually solve it without too much trouble. Maybe not, but in
/// any case, I need to eat dinner with family, so I'll be back. Merry Christmas.
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
