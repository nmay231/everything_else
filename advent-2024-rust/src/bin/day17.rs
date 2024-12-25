use std::collections::VecDeque;

use indoc::indoc;
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

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct PartialSolution {
//     a: u64,
//     // B is fully determined by A
//     // c: u64,
//     /// We need to know which bits of a have been determined, so this is the
//     /// power of 2 with a bit larger than all of the bits in A
//     max_bit: u64,
//     // n_steps: u64,
// }
//
// impl PartialSolution {
//     fn get_a(&self) -> u64 {
//         self.a - (1 << self.a.ilog2())
//     }
// }

type Usize = usize;

/// I could try to write a general purpose algorithm to turn every possible
/// program into a quine, but I think I'll just optimize for my input for now at
/// least.
///
/// Old thoughts are below:
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
    // Here's a "decompiled" version of the program
    // 2,4 # B = 0b111 & A
    // 1,1 # B ^= 1
    // 7,5 # C = A / 2**B
    // 0,3 # A = A / 2**3
    // 4,3 # B ^= C
    // 1,6 # B ^= 6
    // 5,5 # print(B)
    // 3,0 # repeat if A != 0

    let output = [2, 4, 1, 1, 7, 5, 0, 3, 4, 3, 1, 6, 5, 5, 3, 0];
    // Numbers are vectors of bits with least significant at index 0
    let mut possible_a = vec![VecDeque::new()];

    // As we iterate through the outputted numbers, we keep updating the value
    // of a to reflect what's valid. So in other words, A represents a valid
    // *starting* value of A while B and C are the current possible values.
    for (n_ins, next_ins) in output.into_iter().enumerate() {
        possible_a = possible_a
            .into_iter()
            .flat_map(|a| {
                let mut new_a = vec![];
                for b in 0..8 {
                    let mut a = a.clone();

                    // Inverse of exec(B = 0b111 & A)
                    a.push_front(b & 4 == 4);
                    a.push_front(b & 2 == 2);
                    a.push_front(b & 1 == 1);
                    // println!("b, a: {:?}", (b, &a));

                    // exec(b ^= 1)
                    let b: Usize = b ^ 1;
                    // exec(C = A / 2**B), also the inverse of exec(A = A / 2**3)
                    let mut c_bits: Vec<bool> =
                        a.iter().skip(3 * n_ins + b).take(3).map(|b| *b).collect();

                    let c_bits_count = c_bits.len();
                    assert!(c_bits_count < 4);

                    c_bits.extend(std::iter::repeat_n(false, 3 - c_bits_count));
                    assert_eq!(c_bits.len(), 3);

                    // Possible C's based on the bits present in A
                    let mut possible_c = vec![];
                    for mut new_bits in 0..((2 as Usize).pow(3 - c_bits_count as u32)) {
                        new_bits <<= c_bits_count;
                        possible_c.push(
                            new_bits
                                | (c_bits[0] as Usize)
                                | (c_bits[1] as Usize * 2)
                                | (c_bits[2] as Usize * 4),
                        );
                    }

                    // Filter to C's that produce the current instruction
                    let possible_c = possible_c
                        .into_iter()
                        .filter_map(|c| {
                            assert!(c < 8);
                            // exec(B ^= C), and exec(B ^= 6)
                            let b = (b ^ c ^ 6) & 0b111;
                            // exec(print(B))
                            if b == next_ins {
                                Some(vec![c & 1 == 1, c & 2 == 2, c & 4 == 4])
                            } else {
                                None
                            }
                        })
                        .collect_vec();

                    // Update A with new bits of C if needed
                    for bits in possible_c {
                        let mut asdf = a.clone();
                        let bits_copy = bits.clone();
                        for (i, bit) in bits.into_iter().enumerate() {
                            let i = i + 3 * n_ins + b;
                            if i >= asdf.len() {
                                asdf.push_back(bit);
                            } else {
                                asdf[i] = bit;
                            }
                        }

                        println!("init_a, c, b, new_a: {:?}", (&a, &bits_copy, b, &asdf));

                        new_a.push(asdf);
                    }
                }
                new_a
            })
            .collect();

        for a in &possible_a {
            let a_int = a
                .iter()
                .rev()
                .fold(0_usize, |res, bit| res * 2 + if *bit { 1 } else { 0 });
            let program = indoc! {"
                Register A: REPLACE
                Register B: 0
                Register C: 0

                Program: 2,4,1,1,7,5,0,3,4,3,1,6,5,5,3,0

            "}
            .replace("REPLACE", &a_int.to_string());
            println!("a = {} ({:?}): {:?}", a_int, a, (part1(&program)));
        }
        break;
    }

    // for (n_ins, next_ins) in output.into_iter().enumerate() {
    //     possibilities = possibilities
    //         .into_iter()
    //         .flat_map(|solution| {
    //             let mut tmp = vec![];
    //             for mut b in 0_u64..8 {
    //                 let mut solution = solution.clone();
    //                 let result = {
    //                     b ^= 1;
    //                     let a = ((solution.get_a() << 3) & b);
    //                     let c = a >> b;
    //                     let x = b.ilog2();
    //                     let c = solution.a >> b;
    //                     // ;solution.c
    //                 };
    //             }
    //             tmp
    //         })
    //         .collect();
    // }

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
