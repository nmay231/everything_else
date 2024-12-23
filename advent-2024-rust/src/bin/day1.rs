use std::collections::HashMap;

type Output = usize;

fn parse_line(line: &str) -> Option<(usize, usize)> {
    let (a, b) = line.split_once("   ")?;
    return Some((a.parse().ok()?, b.parse().ok()?));
}

fn part1(text: &str) -> Output {
    // I want a chill day, so I'll stick to a simple solution. If I was crazy, I
    // could compare the numbers by ascii value since all the numbers are the
    // same length
    let mut list1 = vec![];
    let mut list2 = vec![];
    for (i, line) in text.lines().enumerate() {
        let (a, b) = parse_line(line).expect(&format!("Unknown format on line {i}"));
        list1.push(a);
        list2.push(b);
    }
    list1.sort();
    list2.sort();
    return list1.iter().zip(&list2).map(|(a, b)| a.abs_diff(*b)).sum();
}

fn part2(text: &str) -> Output {
    let mut counts1 = HashMap::<usize, usize>::new();
    let mut counts2 = HashMap::<usize, usize>::new();
    for (i, line) in text.lines().enumerate() {
        let (a, b) = parse_line(line).expect(&format!("Unknown format on line {i}"));
        *counts1.entry(a).or_default() += 1;
        *counts2.entry(b).or_default() += 1;
    }
    return counts1
        .iter()
        .map(|(key, count)| key * counts2.get(key).unwrap_or(&0) * count)
        .sum();
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day1.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn sample() {
//         assert!(true);
//     }
// }
