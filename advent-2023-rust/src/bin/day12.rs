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

fn repeat<'a, T: Clone>(val: &'a [T], times: usize, sep: &[T]) -> Vec<T> {
    let mut tmp = vec![];
    for (i, rep) in [val].iter().cycle().take(times).enumerate() {
        if i > 0 {
            tmp.extend(sep.to_owned());
        }
        tmp.extend_from_slice(rep);
    }
    tmp
}

fn count_better(springs: &str, known_broken: &[usize]) -> usize {
    count_better_(&springs.chars().collect::<Vec<_>>(), known_broken)
}

/// Perform the equivalent of binary search for this calculation.
fn count_better_(springs: &[char], known_broken: &[usize]) -> usize {
    debug_println!(
        "spring='{}', known_broken={:?}",
        String::from_iter(springs),
        known_broken
    );

    if known_broken.len() == 0 {
        if springs.contains(&'#') {
            return 0;
        } else {
            // There's one way to satisfy no requirements
            return 1;
        }
    } else if known_broken.len() == 1 {
        let broken = known_broken[0];

        let mut iter = springs.iter().enumerate().rev();
        let last_broken = iter.find_map(|(i, char)| if *char == '#' { Some(i) } else { None });
        let min_start = last_broken.unwrap_or(0).saturating_sub(broken - 1);

        let mut iter = iter.rev();
        let max_start = iter
            .find_map(|(i, char)| if *char == '#' { Some(i) } else { None })
            .or(last_broken)
            .unwrap_or(springs.len())
            .clamp(0, springs.len() - broken);

        debug_println!("min_start={}, max_start={}", min_start, max_start);

        let mut sum = 0;
        for start in min_start..=max_start {
            let slice = &springs[start..start + broken];
            debug_println!("start = {}; slice = {}", start, String::from_iter(slice));
            if !slice.contains(&'.') {
                sum += 1
            }
        }
        debug_println!("sum = {sum}");
        return sum;
    } else {
        let split = known_broken.len() / 2;
        let broken = known_broken[split];

        let before = &known_broken[..split];
        let after = &known_broken[split + 1..];

        let min_start = before.len() + before.iter().sum::<usize>();
        let max_start = springs.len() - (broken - 1) - after.len() - after.iter().sum::<usize>();

        let mut sum = 0;
        for start in min_start..max_start {
            let end = start + broken;
            debug_println!(
                "{start}..{end} => '{}'",
                String::from_iter(&springs[start..end])
            );
            // The slice must not have a '#' adjacent on either end
            if start > 0 && springs[start - 1] == '#' {
                continue;
            } else if springs.iter().nth(end) == Some(&'#') {
                continue;
            }

            let slice = &springs[start..end];
            // The slice cannot contain known working springs
            if slice.contains(&'.') {
                continue;
            }

            debug_println!("Not skipped");
            let on_left = count_better_(&springs[..start.saturating_sub(1)], before);
            // Both sides must be greater than 0 since they're gonna be multiplied together
            if on_left == 0 {
                continue;
            }
            let on_right = count_better_(&springs[std::cmp::min(springs.len(), end + 1)..], after);
            sum += on_left * on_right;
        }
        debug_println!("sum = {sum}");
        return sum;
    }
}

fn part2(text: &str) -> Output {
    return text
        .lines()
        .map(|line| {
            let space = line.find(' ').unwrap();
            let springs = repeat(&line[..space].chars().collect::<Vec<_>>(), 5, &['?'])
                .into_iter()
                .collect::<String>();
            let contiguous_broken_count = repeat(
                &line[space + 1..]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                5,
                &[],
            );
            // println!("{};; {}, {:?}", line, springs, contiguous_broken_count);
            let tmp = count_better(&springs, &contiguous_broken_count);
            debug_println!("{};; {}", line, tmp);
            tmp
        })
        .sum();
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day12.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod test {
    use rstest;
    use rstest::*;

    use crate::{count_better, count_spring_row_configs};

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
        #[values(true, false)] use_fast_version: bool,
    ) {
        if use_fast_version {
            let actual = count_better(springs, &broken);
            assert_eq!(actual, expected);
        } else {
            let actual = count_spring_row_configs(springs, broken);
            assert_eq!(actual, expected);
        }
    }
}
