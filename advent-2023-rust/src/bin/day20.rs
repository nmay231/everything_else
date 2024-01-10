use indoc::indoc;
use itertools::Itertools;
use num_integer::Integer;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Write;

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
enum Module<Name: Eq + std::hash::Hash> {
    // Broadcaster, // Technically not needed since we never visit the broadcaster
    FlipFlop(Switch),
    Conjunction(HashMap<Name, Pulse>), // Conjunction Junction, what's your function?
}

fn parse_input(text: &str) -> (HashMap<&str, (Module<&str>, Vec<&str>)>, Vec<&str>) {
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
                        input_map.entry(*name).or_default(); // All inputs start as low pulses
                    }
                },
            }
        }
    }
    (modules, initial_broadcast)
}

fn part1(text: &str, steps: usize) -> Output {
    let (mut modules, initial_broadcast) = parse_input(text);

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

// You have to try it, right?
// I let it run to 10 billion, which is no where near enough since the answer is
// ~200 trillion
fn _part2_brute_force(text: &str) -> Output {
    let (modules, initial_broadcast) = parse_input(text);
    let modules = modules.into_iter().collect_vec();
    // let broadcaster_index = modules
    //     .iter()
    //     .find_position(|(name, _)| name == &"broadcaster")
    //     .unwrap()
    //     .0;
    // modules.swap(0, broadcaster_index);

    let mut module_indexes = modules
        .iter()
        .enumerate()
        .map(|(index, (name, _))| (*name, index))
        .collect::<HashMap<_, _>>();
    module_indexes.insert("rx", module_indexes.len());
    for (_name, index) in module_indexes.iter() {
        assert!(index < &(modules.len() + 1));
    }
    let initial_broadcast = initial_broadcast
        .into_iter()
        .map(|name| *module_indexes.get(name).unwrap())
        .collect_vec();

    let mut modules = modules
        .into_iter()
        .map(|(_, (module, outputs))| {
            let outputs = outputs
                .into_iter()
                .map(|x| *module_indexes.get(x).unwrap())
                .collect_vec();
            let module = match module {
                Module::FlipFlop(x) => Module::FlipFlop(x),
                Module::Conjunction(x) => Module::Conjunction(
                    x.into_iter()
                        .map(|(name, pulse)| (*module_indexes.get(name).unwrap(), pulse))
                        .collect(),
                ),
            };
            (module, outputs)
        })
        .collect_vec();

    for _step in 0.. {
        if _step % 10_000 == 0 {
            println!("Iteration: {}", _step);
        }

        let mut inputs = VecDeque::new();
        inputs.extend(
            initial_broadcast
                .iter()
                .map(|module| (*module, Pulse::Low, 0_usize)),
        );

        while let Some((name, pulse, from_module)) = inputs.pop_front() {
            // println!("'{from_module}' -{pulse:?} -> '{name}'");

            if name == modules.len() {
                // reached rx
                if pulse == Pulse::Low {
                    return _step + 1;
                } else {
                    continue;
                }
            }

            // let module = ;
            let (module, outputs) = modules.swap_remove(name);
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
            modules.push((module, outputs));
            let last_index = modules.len() - 1;
            modules.swap(name, last_index);
        }

        // println!("{:?}", (low_count, high_count));
    }

    unreachable!("It has to happen eventually...");
}

// Highly recommended to open with `xdot assets/day20-visualized.dot` or some
// other graphviz interface.
fn _write_visualization_file(text: &str) {
    let mut map = HashMap::new();
    let mut type_map = HashMap::new();
    for mut line in text.lines() {
        let mut typ = "";
        if line.starts_with(['%', '&']) {
            typ = if line.starts_with('%') {
                "Flip flop"
            } else {
                "Conjunction"
            };
            line = &line[1..];
        } else if line.starts_with("broadcaster ") {
            typ = "Broadcaster";
        }
        let (source, dests) = line.split_once(" -> ").unwrap();
        map.entry(source)
            .or_insert(HashSet::new())
            .extend(dests.split(", "));
        type_map.insert(source, typ);
    }

    let mut result = "digraph day20 {\n".to_owned();
    // I rank the flip flop modules by their distance from the broadcaster. Very
    // helpful in understanding the final solution.
    result.push_str(indoc! {"
    {rank=same; bx nx nr tn}
    {rank=same; jg dj jp md}
    {rank=same; tv qm ps hh}
    {rank=same; qx mj fh tc}
    {rank=same; rt zv dp td}
    {rank=same; kg tk bt bm}
    {rank=same; lg mc rz mr}
    {rank=same; qj kh gq rs}
    {rank=same; qt ck hc dh}
    {rank=same; gb sr hv lt}
    {rank=same; qs jh bz cq}
    {rank=same; bl pt rb kx}
    "});

    for node in map.keys() {
        let node_type = type_map.get(node).unwrap_or(&"");
        let label = if node_type.len() > 0 {
            format!(" ({node_type})")
        } else {
            "".to_owned()
        };
        let color = match node_type {
            &"Flip flop" => "yellow",
            &"Conjunction" => "green",
            _ => "red",
        };
        result.push_str(&format!(
            r#"    {node} [label="{node}{label}", color={color}, style=filled];{}"#,
            "\n"
        ));
    }
    for (source, dests) in map.iter() {
        for dest in dests {
            result.push_str(&format!("    {source} -> {dest};\n"));
        }
    }
    result.push_str("}\n");

    let mut file = File::create("./assets/day20-visualized.txt").unwrap();
    file.write_all(result.as_bytes()).unwrap();
}

// Based on the visualization above, I noticed there were a lot of chains of
// flip-flops (FF) that also send their inputs to conjunctions (CON). A chain of
// FF on their own act as a binary counter with each FF as a bit. When all of
// their inputs point to a CON, that CON acts as an equality check to zero (are
// all the bits zero?). But it only checks after incrementing, so it first
// pulses after 2*n pulses to the starting flip-flop.
//
// However, they don't always work like that. I could try a smarter method of
// brute force. I know the input has a few cliques and each clique has `2^(n+1)`
// possible transition states, where `n` is the number of FFs. In my input,
// there are 4 cliques each with 12 FFs, meaning I could create 4 arrays with
// 2^13 entries mapping each input state and input pulse to the next state. The
// main advantage is that I would no longer have to hash things or compute any
// internal state transitions of the cliques, but it might be the same as just
// using numbers switching the current setup from hash maps to arrays and the
// names to numbers.
//
// Okay, much later from the comment above, I realized it's literally four
// counters that reset at certain intervals, and you just lcm. I thought they
// weren't perfect chains of FF, but after modifying my visualization I saw that
// they were. I don't particularly like that we solve the puzzle because the
// input falls into a specific case, but I guess that's how it works sometimes...

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day20.txt")?;

    _write_visualization_file(&text);

    println!("part 1 result = {:?}", part1(&text, 1000));
    println!(
        "part 2 result = {:?}",
        0b1111_1111_1101_usize.lcm(&0b1111_0100_0011.lcm(&0b1110_1101_0101.lcm(&0b1111_1011_0101)))
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use num_integer::Integer;

    use crate::{_part2_brute_force, part1};

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

    #[test]
    fn part2_my_example1() {
        let expected = _part2_brute_force(
            indoc! {"
            broadcaster -> a1, a2
            %a1 -> con1, b1
            %b1 -> con1, c1
            %c1 -> con1, d1
            %d1 -> con1
            &con1 -> a1, con1x
            &con1x -> main_con

            %a2 -> con2, b2
            %b2 -> c2
            %c2 -> d2
            %d2 -> con2
            &con2 -> a2, b2, c2, con2x
            &con2x -> main_con

            &main_con -> rx
            "}
            .replace("\n\n", "\n")
            .as_str(),
        );
        assert_eq!(0b1001.lcm(&0b1111), expected);
    }
}
