use std::collections::VecDeque;

use itertools::repeat_n;

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
            print!("{}", ".".repeat(c.to_digit(10).unwrap() as usize));
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

/// Unfortunately, I don't see a way to solve part2 without physically
/// allocating memory for each block. I guess I could keep track of the files
/// storing them as ranges, but that is more trouble than it is worth.
fn part2(text: &str) -> Output {
    let text = text.trim();
    // Technically could do 9 * text.len(), but it doesn't matter much anyways
    let mut blocks = Vec::with_capacity(text.len());

    // We skip the first file so that 0 can mean empty blocks
    for (i, c) in text.char_indices().skip(1) {
        let id_or_empty = if i & 1 == 0 { i / 2 } else { 0 };
        let n = c.to_digit(10).unwrap() as usize;
        blocks.extend(repeat_n(id_or_empty, n));
    }

    let mut min_left_index_by_len = [0; 10];

    let mut cur_id = text.len() / 2;
    let mut right_index = blocks.len() - 1;
    while cur_id > 0 {
        // (after the first loop) find the last block of the current file
        while right_index > 0 && blocks[right_index] != cur_id {
            right_index -= 1;
        }

        // Determine the length of the file
        let mut len = 0;
        while right_index > 0 && blocks[right_index] == cur_id {
            right_index -= 1;
            len += 1;
        }
        right_index += 1; // Now indexes the first block of the file

        let mut left_index = min_left_index_by_len[len];
        if left_index >= right_index {
            // The earliest gap with the same length as the file is to the right
            // of the file
            cur_id -= 1;
            continue;
        }

        // Scan from the left finding the earliest gap of the same length
        let mut gap = 0;
        while gap < len && left_index < right_index {
            if blocks[left_index] == 0 {
                gap += 1;
            } else {
                gap = 0;
            }
            left_index += 1;
        }

        if gap == len {
            // Swap the file with the gap
            for (a, b) in (left_index - len..left_index).zip(right_index..right_index + len) {
                blocks.swap(a, b);
            }
        }

        // Whether a gap was found or not, update the earliest possible index
        // for a gap of that size or greater
        for i in len..10 {
            min_left_index_by_len[i] = left_index;
        }

        cur_id -= 1;
    }

    let first_file_len = text.chars().next().unwrap().to_digit(10).unwrap() as usize;
    let mut total = 0;
    for (i, file_id) in blocks.iter().enumerate() {
        total += file_id * (i + first_file_len);
    }

    total
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day9.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{_debug_file_system, part1, part2};
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

    #[rstest]
    #[case::packed1("009", 0+1+2+3+4+5+6+7+8)]
    // It's ambiguous if this is supposed to be allowed, but it didn't seem to
    // matter anyways...
    // #[case::packed2_5("059", 0+1+2+3+4+5+6+7+8)]
    #[case::packed2("099", 0+1+2+3+4+5+6+7+8)]
    #[case::packed3("54321", 2*5 + 1*(6+7+8))]
    #[case::given_example1("12345", 3+4+5 + 2*(10+11+12+13+14))]
    #[case::given_example2("2333133121414131402", 2858)]
    fn part2_cases(#[case] text: &str, #[case] result: usize) {
        _debug_file_system(text);

        assert_eq!(part2(text), result);
    }
}
