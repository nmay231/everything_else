use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

type Output = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Switch {
    Off,
    On,
}

impl Switch {
    fn flip(&self) -> Self {
        match self {
            Switch::Off => Switch::On,
            Switch::On => Switch::Off,
        }
    }
}

impl Into<Pulse> for Switch {
    fn into(self) -> Pulse {
        match self {
            Switch::Off => Pulse::Low,
            Switch::On => Pulse::High,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Pulse {
    #[default]
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module<'a> {
    // Broadcaster, // Technically not needed since we never visit the broadcaster
    FlipFlop(Switch),
    Conjunction(HashMap<&'a str, Pulse>), // Conjunction Junction, what's your function?
}

fn part1(text: &str, steps: usize) -> Output {
    let mut modules = HashMap::new();
    let mut initial_broadcast = vec![];
    for line in text.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ").collect_vec();
        assert!(!outputs.contains(&"broadcaster")); // Should never link back to the broadcaster

        match &name[..1] {
            "b" => {
                assert_eq!(name, "broadcaster");
                // Masquerade as a FlipFlop module for simplicity
                initial_broadcast = outputs;
            }
            "%" => {
                modules.insert(&name[1..], (Module::FlipFlop(Switch::Off), outputs));
            }
            "&" => {
                modules.insert(&name[1..], (Module::Conjunction(HashMap::new()), outputs));
            }
            _ => unreachable!("Can only accept broadcaster, FlipFlop, or Conjunction modules"),
        }
    }
    assert!(initial_broadcast.len() > 0);

    for (name, (_, outputs)) in modules.to_owned().iter() {
        for output in outputs {
            match modules.get_mut(output) {
                None => (), // TODO: Do I provide something or leave as is?
                Some((module, _)) => match module {
                    Module::FlipFlop(_) => (),
                    Module::Conjunction(ref mut input_map) => {
                        input_map.entry(&name).or_default(); // All inputs start as low pulses
                    }
                },
            }
        }
    }

    let mut low_count = 0;
    let mut high_count = 0;
    // println!("{modules:?}");
    for _step in 0..steps {
        let mut inputs = VecDeque::new();
        inputs.extend(
            initial_broadcast
                .iter()
                .map(|module| (*module, Pulse::Low, "broadcaster")),
        );
        low_count += 1;

        while let Some((name, pulse, from_module)) = inputs.pop_front() {
            // println!("'{from_module}' -{pulse:?} -> '{name}'");

            match &pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }

            let module = modules.remove(name);
            if module.is_none() {
                continue;
            }
            let (module, outputs) = module.unwrap();
            let module = match (module, pulse) {
                (Module::FlipFlop(tmp), Pulse::High) => Module::FlipFlop(tmp),
                (Module::FlipFlop(switch), Pulse::Low) => {
                    let state = switch.flip();
                    inputs.extend(outputs.iter().map(|output| (*output, state.into(), name)));
                    Module::FlipFlop(state)
                }
                (Module::Conjunction(mut pulse_map), _) => {
                    pulse_map.insert(from_module, pulse);
                    let output_pulse = if pulse_map.values().all(|p| p == &Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    inputs.extend(outputs.iter().map(|output| (*output, output_pulse, name)));
                    Module::Conjunction(pulse_map)
                }
            };
            modules.insert(name, (module, outputs));
        }

        // println!("{:?}", (low_count, high_count));
    }

    low_count * high_count
}

fn part2(_text: &str, _steps: usize) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day20.txt")?;

    println!("part 1 result = {:?}", part1(&text, 1000));
    println!("part 2 result = {:?}", part2(&text, 1000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::part1;

    #[test]
    fn first_example_part1() {
        let input = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a"};

        assert_eq!(part1(input, 10), 80 * 40);
    }

    #[test]
    fn second_example_part1() {
        let input = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"};

        assert_eq!(part1(input, 40), 170 * 110);
    }
}
