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

        for pair in connections.iter().combinations(2) {
            let [a, b] = pair[..] else { unreachable!() };
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

fn part2(_text: &str) -> Output {
    0
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
}
