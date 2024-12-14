use std::collections::VecDeque;

type Output = usize;

fn _debug_file_system(text: &str) {
    assert!(text.len() & 1 == 1);
    for (i, c) in text.char_indices() {
        if i & 1 == 0 {
            print!(
                "{}",
                format!("{}", i / 2).repeat(c.to_digit(10).unwrap() as usize)
            );
        } else {
            print!("{}", "0".repeat(c.to_digit(10).unwrap() as usize));
        }
    }
    println!();
}

fn part1(text: &str) -> Output {
    let text = text.trim();
    // Always has file blocks at the start and end
    let mut blocks = text
        .chars()
        .map(|c| c.to_digit(10).expect("Only digits allowed") as usize)
        .collect::<VecDeque<_>>();

    assert_eq!(blocks.len() & 1, 1, "Unexpected trailing empty blocks");
    assert!(
        blocks.len() >= 3,
        "Don't want to handle edge cases right now..."
    );

    // File ids
    let mut id_front = 0;
    let mut id_back = blocks.len() / 2;

    // Shorter names are better here b/c it's hard to read long ones
    // front file blocks
    let mut front = blocks.pop_front().unwrap();
    // the empty blocks immediately following
    let mut empty = blocks.pop_front().unwrap();
    // Last file block (never refers to the same block as the front one)
    let mut back = blocks.pop_back().unwrap();

    // remove trailing empty blocks
    if blocks.len() > 0 {
        assert!(matches!(blocks.pop_back(), Some(_)));
    } else {
        empty = back;
    }

    let mut total = 0;
    // 'blocks: for index in 0_usize.. {
    let mut index = 0;
    'blocks: loop {
        if blocks.len() > 0 {
            assert!(blocks.len() & 1 == 1);

            while back == 0 {
                id_back -= 1;
                back = blocks.pop_back().unwrap(); // blocks has odd length
                match blocks.pop_back() {
                    Some(_) => (), // Discard trailing empty blocks
                    None => continue 'blocks,
                }
            }

            while front == 0 && empty == 0 {
                id_front += 1;
                front = blocks.pop_front().unwrap(); // blocks has odd length
                match blocks.pop_front() {
                    Some(next_empty) => empty = next_empty,
                    None => continue 'blocks,
                }
            }
        }

        match (front, back) {
            (0, 0) if blocks.len() == 0 => break 'blocks,
            (0, 0) => unreachable!("We would've kept popping blocks if both were zero"),

            (1.., _) => {
                front -= 1;
                total += id_front * index;
                index += 1;
            }
            (0, 1..) => {
                if empty > 0 {
                    empty -= 1;
                }
                back -= 1;
                total += id_back * index;
                index += 1;
            }
        }
    }

    return total;
}

fn part2(text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day9.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{_debug_file_system, part1};
    use rstest::rstest;

    #[rstest]
    #[case::x105_1("105", 1 * 15)]
    #[case::x105_2("00105", 2 * 15)]
    #[case::x105_3("0000105", 3 * 15)]
    #[case::x105_4("000000105", 4 * 15)]
    #[case::useless_empty("005", 0 + 1 + 2 + 3 + 4)]
    #[case::useless_empty("095", 0 + 1 + 2 + 3 + 4)]
    #[case::useful_empty("00501", 1*(0 + 1 + 2 + 3 + 4) + 2*5)]
    #[case::useful_empty("09501", 2*0 + 1*(1 + 2 + 3 + 4 + 5))]
    #[case::given_example1("12345", 60)]
    #[case::given_example2("2333133121414131402", 1928)]
    fn part1_cases(#[case] text: &str, #[case] result: usize) {
        _debug_file_system(text);

        assert_eq!(part1(text), result);
    }
}
