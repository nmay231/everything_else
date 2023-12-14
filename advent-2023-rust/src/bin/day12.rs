type Output = usize;

const DEBUGGING: bool = false;

macro_rules! debug_println {
    () => {
        if DEBUGGING {
            print!("\n")
        }
    };
    ($($arg:tt)*) => {{
        if DEBUGGING {
            println!("{}", format!($($arg)*))
        }
    }};
}

pub fn count_spring_row_configs(springs: &str, contiguous_broken_count: Vec<usize>) -> usize {
    let springs_start = springs.find(|char| char != '.').unwrap();
    let springs_end = springs.rfind(|char| char != '.').unwrap();
    let springs = &springs[springs_start..=springs_end];

    let maybe_broken = springs
        .split('.')
        .filter_map(|str| if str.len() > 0 { Some(str.len()) } else { None })
        .collect::<Vec<_>>();

    count_spring_row_configs_(springs, &maybe_broken, &contiguous_broken_count)
}

fn count_spring_row_configs_(
    springs: &str,
    maybe_broken: &[usize],
    known_broken: &[usize],
) -> usize {
    debug_println!("{}; {:?}, {:?}", springs, maybe_broken, known_broken);

    if known_broken.len() == 0 {
        if springs.find('#') == None {
            debug_println!("There is one way to satisfy no requirements");
            return 1;
        } else {
            debug_println!("Required broken still left over. Can't satisfy requirements");
            return 0;
        }
    } else if maybe_broken.len() == 0
        || maybe_broken.iter().sum::<usize>() < known_broken.iter().sum()
    {
        debug_println!("No fit: There must be a configuration where the known broken can fit into possible slots");
        return 0;
    }

    let next_broken_slot = springs
        .chars()
        .enumerate()
        .skip_while(|(_, char)| *char != '.')
        .find_map(|(i, char)| if char != '.' { Some(i) } else { None })
        .or(Some(springs.len()))
        .unwrap();

    let broken_required_start = springs.find('#').or(Some(springs.len())).unwrap();

    if known_broken[0] > maybe_broken[0] {
        if broken_required_start < next_broken_slot {
            debug_println!("This slot is too small for current block, but the block cannot be satisfied. Return 0");
            return 0;
        }
        debug_println!("REENTRY: The slot is too small for this block of broken springs. Skipping");
        return count_spring_row_configs_(
            &springs[next_broken_slot..],
            &maybe_broken[1..],
            known_broken,
        );
    } else {
        let if_skipping_this_block = if broken_required_start < next_broken_slot {
            debug_println!("Cannot skip first block");
            0
        } else {
            debug_println!("REENTRY: Check if we can skip the first block");
            count_spring_row_configs_(
                &springs[next_broken_slot..],
                &maybe_broken[1..],
                known_broken,
            )
        };
        debug_println!("count if can skip first block: {}", if_skipping_this_block);

        return if_skipping_this_block
            + (0..=std::cmp::min(maybe_broken[0] - known_broken[0], broken_required_start))
                .map(|start| {
                    let end = start + known_broken[0];
                    if (start == 0 || springs.chars().nth(start - 1) != Some('#'))
                        && springs.chars().nth(end) != Some('#')
                    {
                        let springs = if maybe_broken[0] - end > 1 {
                            &springs[end + 1..]
                        } else if end == springs.len() {
                            ""
                        } else {
                            &springs[next_broken_slot..]
                        };

                        let tmp;
                        let maybe_broken = if maybe_broken[0] - end > 1 {
                            tmp = [&[maybe_broken[0] - end - 1], &maybe_broken[1..]].concat();
                            &tmp[..]
                        } else {
                            &maybe_broken[1..]
                        };
                        debug_println!("REENTRY: Checking offset into first: {}", start);
                        return count_spring_row_configs_(
                            springs,
                            &maybe_broken,
                            &known_broken[1..],
                        );
                    } else {
                        return 0;
                    }
                })
                .sum::<usize>();
    }
}

fn part1(text: &str) -> Output {
    return text
        .lines()
        .map(|line| {
            let space = line.find(' ').unwrap();
            let springs = &line[..space];
            let contiguous_broken_count = line[space + 1..]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            count_spring_row_configs(&springs, contiguous_broken_count)
        })
        .sum();
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day12.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::count_spring_row_configs;

    #[rstest]
    #[case(".###.##.#...", vec![3, 2, 1], 1)]
    #[case("###..##...#", vec![3, 2, 1], 1)]
    #[case("???.###", vec![1, 1, 3], 1)]
    #[case(".??..??...?##.", vec![1, 1, 3], 4)]
    #[case("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6], 1)]
    #[case("????.#...#...", vec![4, 1, 1], 1)]
    #[case("?###????????", vec![3, 2, 1], 10)]
    #[case("????.######..#####.", vec![1, 6, 5], 4)]
    #[case("???????", vec![3], 5)]
    #[case("??????????", vec![1, 1], 8*9/2)]
    #[case("#.#", vec![1], 0)]
    #[case("#.##", vec![2], 0)]
    #[case("?.#.#.?.#.#.?", vec![1, 1, 1], 0)]
    #[case("?.#.#.?.#.#.?", vec![1, 1, 1, 1], 1)]
    fn test_count_spring_row_configs(
        #[case] springs: &str,
        #[case] broken: Vec<usize>,
        #[case] expected: usize,
    ) {
        let actual = count_spring_row_configs(springs, broken);
        assert_eq!(actual, expected);
    }
}
