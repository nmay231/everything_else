use std::collections::HashMap;
use std::iter::repeat_n;
use std::usize;

use advent_2024_rust::{Direc, Point};
use itertools::{EitherOrBoth, Itertools};

type Output = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NumberKey {
    Digit(u32),
    Activate,
    Panic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DirecKey {
    Move(Direc),
    Activate,
    Panic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Key {
    Number(NumberKey),
    Direc(DirecKey),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Keypad<'a> {
    NumberKeypad(&'a [NumberKey], &'a Point<usize>),
    DirecKeypad(&'a [DirecKey], &'a Point<usize>),
}

impl<'a> Keypad<'a> {
    fn invert_self(&self) -> HashMap<char, Point<usize>> {
        match self {
            Keypad::NumberKeypad(pad, size) => {
                let mut inversion = HashMap::<char, Point<usize>>::new();
                for (index, key) in pad.iter().enumerate() {
                    let point = Point::from_index(*size, index);
                    let char = match key {
                        NumberKey::Digit(digit) => {
                            char::from_digit(*digit, 10).expect("digit should be less than 10")
                        }
                        NumberKey::Activate => 'A',
                        NumberKey::Panic => '.',
                    };
                    inversion.insert(char, point);
                }
                inversion
            }
            Keypad::DirecKeypad(pad, size) => {
                let mut inversion = HashMap::<char, Point<usize>>::new();
                for (index, key) in pad.iter().enumerate() {
                    let point = Point::from_index(*size, index);
                    let char = match key {
                        DirecKey::Move(direc) => match direc {
                            Direc::North => '^',
                            Direc::East => '>',
                            Direc::South => 'v',
                            Direc::West => '<',
                        },
                        DirecKey::Activate => 'A',
                        DirecKey::Panic => '.',
                    };
                    inversion.insert(char, point);
                }
                inversion
            }
        }
    }

    fn size(&self) -> &Point<usize> {
        match self {
            Keypad::NumberKeypad(_, size) => size,
            Keypad::DirecKeypad(_, size) => size,
        }
    }

    fn is_panic_slot(&self, slot: &Point<usize>) -> bool {
        match self {
            Keypad::NumberKeypad(pad, grid_size) => {
                &NumberKey::Panic == &pad[slot.as_index(grid_size)]
            }
            Keypad::DirecKeypad(pad, grid_size) => {
                &DirecKey::Panic == &pad[slot.as_index(grid_size)]
            }
        }
    }

    fn get_panic_slots(&self) -> Vec<Point<usize>> {
        match self {
            Keypad::NumberKeypad(pad, grid_size) => {
                return pad
                    .iter()
                    .enumerate()
                    .filter_map(|(index, key)| {
                        if key == &NumberKey::Panic {
                            Some(Point::from_index(grid_size, index))
                        } else {
                            None
                        }
                    })
                    .collect();
            }
            Keypad::DirecKeypad(pad, grid_size) => {
                return pad
                    .iter()
                    .enumerate()
                    .filter_map(|(index, key)| {
                        if key == &DirecKey::Panic {
                            Some(Point::from_index(grid_size, index))
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }
    }

    /// Assumes the desired key is present in the keypad. Panics otherwise or if
    /// the key is not unique.
    fn find_slot(&self, key: Key) -> Point<usize> {
        match (self, key) {
            (Keypad::NumberKeypad(pad, grid_size), Key::Number(search_key)) => {
                let slots = pad
                    .iter()
                    .enumerate()
                    .filter_map(|(index, key)| {
                        if key == &search_key {
                            Some(Point::from_index(grid_size, index))
                        } else {
                            None
                        }
                    })
                    .collect_vec();
                assert_eq!(slots.len(), 1);
                slots[0]
            }
            (Keypad::DirecKeypad(pad, grid_size), Key::Direc(search_key)) => {
                let slots = pad
                    .iter()
                    .enumerate()
                    .filter_map(|(index, key)| {
                        if key == &search_key {
                            Some(Point::from_index(grid_size, index))
                        } else {
                            None
                        }
                    })
                    .collect_vec();
                assert_eq!(slots.len(), 1);
                slots[0]
            }
            // TODO: I wonder if it would have been better to share the variants
            // in the same enum to avoid this whole mess
            (Keypad::NumberKeypad(_, _), Key::Direc(_))
            | (Keypad::DirecKeypad(_, _), Key::Number(_)) => unreachable!("Wrong key type"),
        }
    }
}

#[rustfmt::skip]
const NUMBER_KEYPAD: [NumberKey; 12] = [
    NumberKey::Digit(7), NumberKey::Digit(8), NumberKey::Digit(9),
    NumberKey::Digit(4), NumberKey::Digit(5), NumberKey::Digit(6),
    NumberKey::Digit(1), NumberKey::Digit(2), NumberKey::Digit(3),
    NumberKey::Panic,    NumberKey::Digit(0), NumberKey::Activate,
];

#[rustfmt::skip]
const DIREC_KEYPAD: [DirecKey; 6] = [
    DirecKey::Panic,             DirecKey::Move(Direc::North), DirecKey::Activate,
    DirecKey::Move(Direc::West), DirecKey::Move(Direc::South), DirecKey::Move(Direc::East),
];

fn _debug_forward_passes<'a>(text: &str, pad: Keypad<'a>, robot_arm: usize) -> String {
    let forward = pad
        .invert_self()
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();
    let grid_size = pad.size();

    let mut robot_arm = Point::from_index(grid_size, robot_arm);

    let mut result = String::new();
    for char in text.chars() {
        let direc = match char {
            '^' => Direc::North,
            '>' => Direc::East,
            'v' => Direc::South,
            '<' => Direc::West,
            'A' => {
                result.push(*forward.get(&robot_arm).unwrap());
                continue;
            }
            _ => unreachable!("Unexpected char '{}'", char),
        };

        robot_arm = robot_arm.next_point(&direc, grid_size).expect("Your face");
    }

    return result;
}

fn i_hate_my_life_rn(
    start: Point<usize>,
    target: Point<usize>,
    // key_places: HashMap<Direc, Point<usize>>,
    pad: Keypad,
    n_iters: usize,
) -> String {
    assert!(n_iters > 0);
    assert_eq!(
        pad.size().area(),
        DIREC_KEYPAD.len(),
        "I don't like this pseudo-dependency-injection. I need to change this"
    );

    let path_and_cost = (vec![(start, target, 1)], start.manhattan_distance(&target));
    let mut possible_paths = vec![path_and_cost];

    for _ in 0..n_iters {
        let mut new_min_cost = usize::MAX;
        let prev_paths = possible_paths.clone();
        possible_paths = vec![];
        for (path, cost) in prev_paths.into_iter() {}

        possible_paths = possible_paths
            .into_iter()
            .filter(|(_, cost)| {
                assert!(cost >= &new_min_cost, "forgot to update min_cost");
                cost == &new_min_cost
            })
            .collect();
    }

    ()
}

/// This function recursively finds the best sequence of directional-key presses
/// to move the robot arm from the start to the target. The biggest optimization
/// is to repeat the same move as often as possible, but there are even
/// optimizations that only come about after several iterations, so we need to
/// try both of them to optimize things.
fn better_direc_key_sequence(
    start: &Point<usize>,
    target: &Point<usize>,
    // key_places: HashMap<Direc, Point<usize>>,
    pad: Keypad,
    n_iters: usize,
    activate_n_times: usize,
) -> String {
    assert!(n_iters > 0);
    if start == target {
        // No moves necessary to go nowhere
        return repeat_n('A', activate_n_times).collect();
    }

    let diff = target.map(|u| *u as isize) - start.map(|u| *u as isize);

    let (x, y, dx, dy) = match diff {
        Point {
            x: x @ 0..,
            y: y @ 0..,
        } => (x as usize, y as usize, Direc::East, Direc::South),
        Point {
            x: x @ 0..,
            y: y @ ..0,
        } => (x as usize, -y as usize, Direc::East, Direc::North),
        Point {
            x: x @ ..0,
            y: y @ ..0,
        } => (-x as usize, -y as usize, Direc::West, Direc::North),
        Point {
            x: x @ ..0,
            y: y @ 0..,
        } => (-x as usize, y as usize, Direc::West, Direc::South),
    };

    if n_iters == 1 {
        // return repeat_n('A', activate_n_times).collect();
        let forwards = repeat_n(dx, x).chain(repeat_n(dy, y)).collect_vec();

        return forwards
            .iter()
            .map(Direc::to_ascii)
            // .chain(forwards.iter().rev().map(Direc::to_ascii)) -- This
            // doesn't work because I might need to go backwards a different way
            // or not at all
            // .map(|direc| direc.to_ascii())
            // .chain(['A'])
            .collect();
    }

    let x_or_y_first = if x == 0 || y == 0 {
        // There is no choice of going horizontally or vertically first
        // vec![repeat_n(dx, x).chain(repeat_n(dy, y)).collect_vec()]
        EitherOrBoth::Left(())
    } else {
        let horizontal_corner = Point::new_xy(start.x, target.y);
        let vertical_corner = Point::new_xy(target.x, start.y);
        // TODO: Hard-coded value (though I don't know if I care about that)
        let panic_corner = Point::new_xy(0, 0);

        // If we have to avoid the panic corner, there is only one path
        if horizontal_corner == panic_corner {
            // vec![repeat_n(dy, y).chain(repeat_n(dx, x)).collect_vec()]
            EitherOrBoth::Right(())
        } else if horizontal_corner == panic_corner {
            // vec![repeat_n(dx, x).chain(repeat_n(dy, y)).collect_vec()]
            EitherOrBoth::Left(())
        } else {
            EitherOrBoth::Both((), ())
            // vec![
            //     repeat_n(dx, x).chain(repeat_n(dy, y)).collect_vec(),
            //     repeat_n(dy, y).chain(repeat_n(dx, x)).collect_vec(),
            // ]
        }
    };

    // so we need to recursive try things which means getting the starting and
    // target possitions
    // TODO: Static data structure?
    let activate = pad.find_slot(Key::Direc(DirecKey::Activate));
    let horizontal = pad.find_slot(Key::Direc(DirecKey::Move(dx)));
    let vertical = pad.find_slot(Key::Direc(DirecKey::Move(dy)));

    let mut options = vec![];

    if x_or_y_first.has_left() {
        let move_horizontal =
            better_direc_key_sequence(&activate, &horizontal, pad, n_iters - 1, x);
        let to_target = better_direc_key_sequence(&horizontal, &target, pad, n_iters - 1, y);
        let undo_vertical = better_direc_key_sequence(&target, &horizontal, pad, n_iters - 1, y);
        let undo_horizontal =
            better_direc_key_sequence(&horizontal, &activate, pad, n_iters - 1, x);
        options.push(format!(""));
    }
    if x_or_y_first.has_right() {}

    let asdf = match x_or_y_first {
        EitherOrBoth::Left(_) => {}
        EitherOrBoth::Right(_) => {}
        EitherOrBoth::Both(_, _) => {}
    };
    // if x_or_y_first.has_left() {}

    // If the start and target slot are opposite corners of a
    // rectangle, then we need to check if the other pair of
    // opposing corners contains the panic slot. If it does then it
    // matters which side of the perimeter we walk on. Otherwise, we
    // simply prefer East over West and *then* North over South
    // since they are closer to the A key on the direction pad
    let corner1 = Point::new_xy(start.x, target.y);
    let corner2 = Point::new_xy(target.x, start.y);

    if panic_slots.contains(&corner1) || panic_slots.contains(&corner2) {}
}

fn part1(text: &str) -> Output {
    let number_pad_size = Point::new_xy(3, 4);
    let direc_pad_size = Point::new_xy(3, 2);
    let state = [
        (11, Keypad::NumberKeypad(&NUMBER_KEYPAD, &number_pad_size)),
        (2, Keypad::DirecKeypad(&DIREC_KEYPAD, &direc_pad_size)),
        (2, Keypad::DirecKeypad(&DIREC_KEYPAD, &direc_pad_size)),
    ];

    for (index, pad) in &state {
        match pad {
            Keypad::NumberKeypad(pad, _size) => {
                assert_eq!(pad[*index], NumberKey::Activate);
            }
            Keypad::DirecKeypad(pad, _size) => {
                assert_eq!(pad[*index], DirecKey::Activate);
            }
        }
    }

    let mut state = state.map(|(index, pad)| {
        (
            Point::from_index(pad.size(), index),
            pad.invert_self(),
            pad.get_panic_slots(),
        )
    });

    let mut total = 0;
    for line in text.lines() {
        let mut chars = line.chars().collect_vec();
        let mut next_chars = vec![];
        for (state_index, (robot_arm, ref map, ref panic_slots)) in state.iter_mut().enumerate() {
            let chars_clone = chars.clone();
            for c in chars {
                let Some(target) = map.get(&c) else {
                    panic!(
                        "Unexpected char '{}' during state transition {}",
                        c, state_index
                    )
                };
                let diff = target.map(|u| *u as isize) - robot_arm.map(|u| *u as isize);

                let (x, y, dx, dy) = match diff {
                    Point {
                        x: x @ 0..,
                        y: y @ 0..,
                    } => (x as usize, y as usize, Direc::East, Direc::South),
                    Point {
                        x: x @ 0..,
                        y: y @ ..0,
                    } => (x as usize, -y as usize, Direc::East, Direc::North),
                    Point {
                        x: x @ ..0,
                        y: y @ ..0,
                    } => (-x as usize, -y as usize, Direc::West, Direc::North),
                    Point {
                        x: x @ ..0,
                        y: y @ 0..,
                    } => (-x as usize, y as usize, Direc::West, Direc::South),
                };
                // If the robot_arm and target slot are opposite corners of a
                // rectangle, then we need to check if the other pair of
                // opposing corners contains the panic slot. If it does then it
                // matters which side of the perimeter we walk on. Otherwise, we
                // simply prefer East over West and *then* North over South
                // since they are closer to the A key on the direction pad
                let corner1 = Point::new_xy(robot_arm.x, target.y);
                let corner2 = Point::new_xy(target.x, robot_arm.y);

                if panic_slots.contains(&corner1) || panic_slots.contains(&corner2) {}

                // To avoid hitting the panic spots, we prefer East, then North/South, then West
                if dx == Direc::East {
                    next_chars.extend(repeat_n('>', x));
                    match dy {
                        Direc::North => {
                            next_chars.extend(repeat_n('^', y));
                        }
                        Direc::South => {
                            next_chars.extend(repeat_n('v', y));
                        }
                        _ => unreachable!(),
                    }
                } else {
                    assert_eq!(dx, Direc::West);
                    match dy {
                        Direc::North => {
                            next_chars.extend(repeat_n('^', y));
                        }
                        Direc::South => {
                            next_chars.extend(repeat_n('v', y));
                        }
                        _ => unreachable!(),
                    }
                    next_chars.extend(repeat_n('<', x));
                }
                next_chars.push('A');
                *robot_arm = target.clone();
                // next_chars.extend();
            }
            if line == "379A" {
                println!(
                    "transition: {:?} -> {:?}",
                    &chars_clone.iter().collect::<String>(),
                    &next_chars.iter().collect::<String>()
                );
            }
            chars = next_chars;
            next_chars = vec![];
        }
        println!(
            "asdf{:?}",
            (chars.len(), &line[0..3], chars.iter().collect::<String>())
        );
        total += chars.len() * &line[0..3].parse::<usize>().expect("Failed to parse number");
    }

    total
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day21.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use advent_2024_rust::Point;
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn part1_given_example() {
        use crate::{Keypad, _debug_forward_passes, DIREC_KEYPAD, NUMBER_KEYPAD};
        let number_pad_size = Point::new_xy(3, 4);
        let direc_pad_size = Point::new_xy(3, 2);
        let state = [
            (2, Keypad::DirecKeypad(&DIREC_KEYPAD, &direc_pad_size)),
            (2, Keypad::DirecKeypad(&DIREC_KEYPAD, &direc_pad_size)),
            (11, Keypad::NumberKeypad(&NUMBER_KEYPAD, &number_pad_size)),
        ];
        let mut asdf =
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string();
        // let mut next = String::new();
        for (a, b) in &state {
            let next = _debug_forward_passes(&asdf, b.clone(), a.clone());
            println!("change {:?} -> {:?}", (asdf), next);
            asdf = next;
        }
        println!();

        asdf = "v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A".to_string();
        for (a, b) in state {
            let next = _debug_forward_passes(&asdf, b, a);
            println!("change {:?} -> {:?}", (asdf), next);
            asdf = next;
        }
        println!();

        assert_eq!(crate::part1(TEXT1), 126384);
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
