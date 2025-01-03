use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Output = usize;

fn part1(text: &str) -> Output {
    let mut conns = HashMap::<&str, HashSet<&str>>::new();
    for line in text.lines() {
        let (a, b) = line.split_once('-').expect("Missing hyphen in file");
        conns.entry(a).or_default().insert(b);
        conns.entry(b).or_default().insert(a);
    }

    let mut total = 0;
    for (computer, connections) in conns.iter() {
        if !computer.starts_with('t') {
            continue;
        }

        for (a, b) in connections.iter().tuple_combinations() {
            // Only count unique connections
            if (a.starts_with('t') && a < computer) || (b.starts_with('t') && b < computer) {
                continue;
            }
            if conns.get(a).unwrap().contains(b) {
                total += 1;
            }
        }
    }

    return total;
}

/// If there are multiple maximum cliques, returns one arbitrarily
fn largest_clique<T: Copy + Eq + std::hash::Hash + Ord + std::fmt::Debug>(
    graph: &HashMap<T, HashSet<T>>,
) -> HashSet<T> {
    let mut max = HashSet::new();
    // We know there are triangles; we need better than triangles
    let mut known_min = 3;
    for start in graph.keys() {
        max = match largest_clique_helper(graph, HashSet::from([start.clone()]), known_min, start) {
            Some(clique) => clique,
            None => continue,
        };
        assert!(max.len() > known_min);
        known_min = max.len();
    }
    return max;
}

fn largest_clique_helper<T: Copy + Eq + std::hash::Hash + Ord + std::fmt::Debug>(
    graph: &HashMap<T, HashSet<T>>,
    clique: HashSet<T>,
    known_min: usize,
    start: &T,
) -> Option<HashSet<T>> {
    let mut max = None;
    if clique.len() > known_min {
        max = Some(clique.clone())
    }

    for neighbor in graph.get(start).unwrap() {
        if neighbor < start {
            continue; // Avoid duplicate work
        } else if !graph.get(neighbor).unwrap().is_superset(&clique) {
            continue; // clique + {neighbor} is not completely connected
        }

        let x = largest_clique_helper(
            graph,
            clique
                .union(&[*neighbor].into_iter().collect())
                .map(|x| *x)
                .collect(),
            known_min,
            neighbor,
        );

        // TODO: I wish Option<T> implemented .max_by()
        max = [max, x]
            .into_iter()
            .filter_map(|x| x)
            .max_by(|a, b| a.len().cmp(&b.len()));
    }

    return max;
}

fn part2(text: &str) -> String {
    let mut conns = HashMap::<&str, HashSet<&str>>::new();
    for line in text.lines() {
        let (a, b) = line.split_once('-').expect("Missing hyphen in file");
        conns.entry(a).or_default().insert(b);
        conns.entry(b).or_default().insert(a);
    }

    let clique = largest_clique(&conns);
    let mut clique = clique.into_iter().collect_vec();
    clique.sort();
    return clique.join(",");
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day23.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 7);
    }

    #[test]
    fn part2_given_example() {
        assert_eq!(crate::part2(TEXT1), "co,de,ka,ta".to_string());
    }
}
