use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Write};
use std::usize;

use advent_2024_rust::{Direc, Point};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Key {
    Activate,
    Panic,
    Digit(u32),
    Move(Direc),
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Activate => f.write_char('A'),
            Key::Panic => f.write_char('!'),
            Key::Move(direc) => f.write_char(direc.to_ascii()),
            Key::Digit(d) => write!(f, "{}", d),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KeyPad<'a> {
    keys: &'a [Key],
    grid_size: Point<usize>,
    directional: bool,
}

impl KeyPad<'static> {
    const fn new(keys: &'static [Key], grid_size: Point<usize>, directional: bool) -> Self {
        Self {
            keys,
            grid_size,
            directional,
        }
    }
}

impl<'a> KeyPad<'a> {
    // TODO: I wanted to put this into ::new() but iteration is not allowed in
    // const functions, apparently. I'm still fairly new to the rules of const
    // stuff though so I might be doing it wrong.
    fn assert_consistent(&self) {
        assert_eq!(self.grid_size.area(), self.keys.len());

        let mut unique = HashSet::new();
        for key in self.keys {
            unique.insert(key);
            match key {
                Key::Digit(_) if self.directional => {
                    unreachable!("Put the wrong thing in the thing")
                }
                Key::Move(_) if !self.directional => {
                    unreachable!("Put the wrong thing in the thing")
                }
                _ => (),
            }
        }
        assert_eq!(
            unique.len(),
            self.keys.len(),
            "Assumed each key listed once"
        );
    }

    fn get_slot_of(&self, key: &Key) -> Point<usize> {
        for (i, k) in self.keys.iter().enumerate() {
            if k == key {
                return Point::from_index(&self.grid_size, i);
            }
        }
        unreachable!("Failed to find key: {:?}", key);
    }

    fn at_slot(&self, point: &Point<usize>) -> &Key {
        &self.keys[point.as_index(&self.grid_size)]
    }

    fn _point_to_str_map(&self) -> HashMap<Point<usize>, String> {
        let mut result = HashMap::new();
        for (index, key) in self.keys.iter().enumerate() {
            result.insert(
                Point::from_index(&self.grid_size, index),
                format!("{}", key),
            );
        }
        return result;
    }

    fn _debug_forward_passes(&self, text: &str) -> String {
        let forward = self._point_to_str_map();
        let mut robot_arm = self.get_slot_of(&Key::Activate);
        let panic_button = self.get_slot_of(&Key::Panic);
        let grid_size = &self.grid_size;

        let mut result = String::new();
        for char in text.chars() {
            let direc = match char {
                '^' => Direc::North,
                '>' => Direc::East,
                'v' => Direc::South,
                '<' => Direc::West,
                'A' => {
                    result.push_str(forward.get(&robot_arm).unwrap());
                    continue;
                }
                _ => unreachable!("Unexpected char '{}'", char),
            };

            robot_arm = robot_arm.next_point(&direc, grid_size).expect("Your face");
            assert_ne!(robot_arm, panic_button, "Looked at the panic button!");
        }

        return result;
    }
}

#[rustfmt::skip]
const _NUMBER: [Key; 12] = [
    Key::Digit(7), Key::Digit(8), Key::Digit(9),
    Key::Digit(4), Key::Digit(5), Key::Digit(6),
    Key::Digit(1), Key::Digit(2), Key::Digit(3),
    Key::Panic,    Key::Digit(0), Key::Activate,
];
const NUMBER_KEYPAD: KeyPad<'_> = KeyPad::new(&_NUMBER, Point::new_xy(3, 4), false);

#[rustfmt::skip]
const _DIREC: [Key; 6] = [
    Key::Panic,             Key::Move(Direc::North), Key::Activate,
    Key::Move(Direc::West), Key::Move(Direc::South), Key::Move(Direc::East),
];
const DIREC_KEYPAD: KeyPad<'_> = KeyPad::new(&_DIREC, Point::new_xy(3, 2), true);

fn shortest_sequence(
    start_path: Vec<(Point<usize>, usize)>,
    keypad: &KeyPad,
    n_iters: usize,
) -> usize {
    assert_eq!(start_path[0].1, 0, "We don't press the starting point");
    assert!(n_iters > 0);

    type Transition = (Point<usize>, Point<usize>, usize, Point<usize>);
    type MoveSet = Vec<(Point<usize>, usize)>;
    type TransitionMap = HashMap<Transition, Result<MoveSet, [MoveSet; 2]>>;

    // Return the minimum transition if unique, while updating the transition map
    fn prune_transitions<'a>(
        by_transitions: &'a mut TransitionMap,
        key: &Transition,
    ) -> Option<Result<&'a MoveSet, Vec<&'a MoveSet>>> {
        let mut move_sets = match by_transitions.get(key) {
            None => return None,
            Some(Ok(move_set)) => return Some(Ok(move_set)),
            Some(Err(move_sets)) => move_sets.to_vec(),
        };

        let paths_were_equal = move_sets
            .iter()
            .map(|move_set| move_set.iter().map(|(_, cost)| cost).sum::<usize>())
            .all_equal();
        assert!(
            paths_were_equal,
            "If one path was better than the other, then it should have been pruned already"
        );

        let tmp = move_sets;
        move_sets = vec![];
        for move_set in tmp.into_iter() {
            for (a, b) in move_set.iter().tuple_windows() {
                let ((start, _), (dest, _)) = (a, b);

                match by_transitions.get(todo!()) {}

                let diff = dest.try_map_isize().unwrap() - start.try_map_isize().unwrap();

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

                let move_x = (DIREC_KEYPAD.get_slot_of(&Key::Move(dx)), x);
                let move_y = (DIREC_KEYPAD.get_slot_of(&Key::Move(dy)), y);
                let activate = (DIREC_KEYPAD.get_slot_of(&Key::Activate), *activations);

                let extend = if x == 0 {
                    vec![move_y, activate]
                } else if y == 0 {
                    vec![move_x, activate]
                } else {
                    let horizontal = Point::new_xy(dest.x, start.y);
                    let vertical = Point::new_xy(start.x, dest.y);

                    if horizontal == panic_button {
                        // Must go vertical first
                        vec![move_y, move_x, activate]
                    } else if vertical == panic_button {
                        // Must go horizontal first
                        vec![move_x, move_y, activate]
                    } else {
                        // We check one of the sub-paths later
                        let [a, b] = [
                            vec![move_x, move_y, activate],
                            vec![move_y, move_x, activate],
                        ];
                        let mut new_path = new_path.clone();
                        new_path.extend(a.clone());
                        let old_path = old_path[index + 1..].to_vec();
                        possible_paths.push_front((old_path, new_path, pad, n_iters));

                        by_transition
                            .entry((*start, *dest, *activations, panic_button))
                            .and_modify(|_| unreachable!("We assume key is not present"))
                            .or_insert(Err([a, b.clone()]));

                        b
                    }
                };
                new_path.extend(extend.clone().into_iter());

                by_transition
                    .entry((*start, *dest, *activations, panic_button))
                    .or_insert(Ok(extend));
            }
        }
        todo!();
        // if move_sets.len() == 1 {
        //     return Some(Ok(move_sets.iter().next().unwrap()));
        // }

        // So, we end up with a tree of possible transitions up to a certain
        // depth. We stop at the earliest commonly available depth. If some
        // leaves are longer than others, we need to prune those. If we haven't
        // reduced the next transition to a single move-set, then we return the
        // error condition of the two possible move_sets. While we prune to an
        // indefinite depth, we only ever return the next one or two possible
        // move set(s).
        //
        // Thoughts:
        // - Is it reasonable to call this every time I check for cached
        //   results? Or will I get large un-prunable subtrees?
        // - Is Up and Right always irreducible? I don't think so because each
        //   recursive step goes back to Activate eventually meaning you always
        //   get either Left or Down.
        // - I might also need to detect cycles that happen to be shorter, not
        //   just total move length. Actually, that doesn't matter because
        //   infinite boxes of chocolates is the same as infinite chocolate.
    }

    let mut by_transition = TransitionMap::new();
    let mut min_by_iter = HashMap::new();
    let mut possible_paths = VecDeque::from([(start_path, vec![], keypad, n_iters)]);
    let mut max_iters = 10000_usize;

    while let Some((old_path, mut new_path, pad, n_iters)) = possible_paths.pop_front() {
        max_iters -= 1;
        if max_iters == 0 {
            println!(
                "len: {:?}",
                (
                    possible_paths.len(),
                    &possible_paths.iter().take(5).collect_vec()
                )
            );
            break;
        }
        if new_path.len() == 0 {
            if min_by_iter.get(&n_iters).is_none() {
                println!("count: {}", possible_paths.len());
            }
            let min_length = min_by_iter.entry(n_iters).or_insert(usize::MAX);
            let path_len = old_path.iter().map(|(_, steps)| steps).sum::<usize>();
            if path_len > *min_length {
                continue;
            }

            // if path_len < *min_length {
            //     print!("iter: {}, path: ", n_iters - 1);
            //     for x in &old_path {
            //         print!("{}", format!("{}", pad.at_slot(&x.0)).repeat(x.1));
            //     }
            //     println!();
            // }

            *min_length = path_len;
            if n_iters == 1 {
                continue;
            }

            new_path.push((DIREC_KEYPAD.get_slot_of(&Key::Activate), 0));
        }
        assert!(n_iters > 1);

        let panic_button = pad.get_slot_of(&Key::Panic);

        for (index, (a, b)) in old_path.iter().tuple_windows().enumerate() {
            let (source, _) = a;
            let (target, activations) = b;

            match prune_transitions(
                &mut by_transition,
                &(*source, *target, *activations, panic_button),
            ) {
                Some(Ok(move_set)) => {
                    new_path.extend_from_slice(move_set);
                    continue;
                }
                Some(Err(move_sets)) => {
                    {
                        let mut new_path = new_path.clone();
                        new_path.extend(move_sets[0].clone());
                        let old_path = old_path[index + 1..].to_vec();
                        possible_paths.push_front((old_path, new_path, pad, n_iters));
                    }
                    new_path.extend_from_slice(move_sets[1]);
                    continue;
                }
                None => (), // TODO: Just use empty arrays instead of Option<Result<>>
            }

            todo!();
            //
        }

        possible_paths.push_back((new_path, vec![], &DIREC_KEYPAD, n_iters - 1));
    }

    return *min_by_iter.get(&1).expect("to find answer");
}

fn partial_part1(text: &str, n_iters: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let activate_slot = NUMBER_KEYPAD.get_slot_of(&Key::Activate);

    for line in text.lines() {
        let keys = [(activate_slot, 0)]
            .into_iter()
            .chain(
                line.chars()
                    .map(|c| match c {
                        'A' => Key::Activate,
                        '0'..='9' => Key::Digit(c.to_digit(10).unwrap()),
                        _ => unreachable!("Unexpected character {:?} in room code {}", c, line),
                    })
                    .map(|key| (NUMBER_KEYPAD.get_slot_of(&key), 1)),
            )
            .collect_vec();

        let short = shortest_sequence(keys, &NUMBER_KEYPAD, n_iters);
        let x = line[0..3].parse::<usize>().unwrap();
        result.push((x, short));
    }

    return result;
}

fn part1(text: &str) -> usize {
    return partial_part1(text, 4).into_iter().map(|(a, b)| a * b).sum();
}

fn part2(text: &str) -> usize {
    return partial_part1(text, 25)
        .into_iter()
        .map(|(a, b)| a * b)
        .sum();
}

fn main() -> std::io::Result<()> {
    // Sanity check
    NUMBER_KEYPAD.assert_consistent();
    DIREC_KEYPAD.assert_consistent();

    let text = std::fs::read_to_string("./assets/day21.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
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
        assert_eq!(
            crate::partial_part1(TEXT1, 4),
            vec![(29, 68), (980, 60), (179, 68), (456, 64), (379, 64)]
        );
    }

    #[rstest::rstest]
    #[case::x029a("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_string(), "029A")]
    #[case::x980a("<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".to_string(), "980A")]
    #[case::x179a("<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string(), "179A")]
    #[case::x456a("<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".to_string(), "456A")]
    #[case::x379a("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string(), "379A")]
    fn debug_forward(#[case] mut text: String, #[case] expected: &str) {
        use crate::{DIREC_KEYPAD, NUMBER_KEYPAD};
        for pad in [&DIREC_KEYPAD, &DIREC_KEYPAD, &NUMBER_KEYPAD] {
            let next = pad._debug_forward_passes(&text);
            println!("change {:?} -> {:?}", text, next);
            text = next;
        }
        assert_eq!(&text, expected);
    }
}
