use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use num_integer::Integer;

type Output = usize;

fn part1(text: &str) -> Output {
    let mut stones = text
        .trim()
        .split(' ')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|stone| match format!("{}", stone).as_str() {
                "0" => vec![1],
                text if text.len() % 2 == 0 => {
                    let (a, b) = text.split_at(text.len() / 2);
                    vec![a.parse().unwrap(), b.parse().unwrap()]
                }
                _ => vec![stone * 2024],
            })
            .collect();
    }

    stones.len()
}

// TODO: I can either move on to the next problem for now, or push the changes
// to the cache into a temporary variable before merging. I do have to do this
// for 75 iterations after all. Note: I can't just use a mutex because I think I
// do have to lock the mutex to read a value and returning a subvalue of the
// hashmap is impossible unless I keep it locked. Maybe this is where I need to
// use a (Ref)Cell?


fn part2(text: &str) -> Output {
    RefCell
    Cell
    let mut stones = text
        .trim()
        .split(' ')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut map = HashMap::new();
    map.insert(0, Cow::from(vec![1_usize]));
    // let empty = vec![];
    // Rc::new

    for _ in 0..75 {
        stones = stones
            .into_iter()
            .flat_map(|stone| {
                match map.get(&stone) {
                    Some(next) => {
                        let x = next.to_mut();
                        x
                    }
                    None => {
                        // Kudos to: https://stackoverflow.com/questions/69297477/getting-the-length-of-an-int
                        let n_digits = stone.checked_ilog10().expect("stone should never be 0");
                        if n_digits % 2 == 0 {
                            let (div, rem) = stone.div_mod_floor(&10_usize.pow(n_digits));
                            map.insert(stone, Cow::from(vec![div, rem]));
                        } else {
                            map.insert(stone, Cow::from(vec![stone * 2024]));
                        }
                        map.get(&stone).unwrap().to_mut()
                        // Rc::clone(map.get(&stone).unwrap()).into_iter()
                        // &empty
                    }
                }
                // match format!("{}", stone).as_str() {
                //     "0" => vec![1],
                //     text if text.len() % 2 == 0 => {
                //         let (a, b) = text.split_at(text.len() / 2);
                //         vec![a.parse().unwrap(), b.parse().unwrap()]
                //     }
                //     _ => vec![stone * 2024],
                // }
            })
            .map(|num| *num)
            .collect();
    }

    stones.len()
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day11.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;

    const TEXT: &str = indoc! {"
        125 17
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT), 55312);
    }
}
