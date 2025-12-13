use advent_2025_rust::DisjointSetWithCount;
use itertools::Itertools;

/// Since we are only sorting by distance and don't actually need it (yet), we
/// can just compute the distance squared since the square-root function is monotonic
fn euclid_squared(a: &(usize, usize, usize), b: &(usize, usize, usize)) -> usize {
    return a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2);
}

fn part1(text: &str, n_closest: usize) -> usize {
    let mut coords = vec![];

    for line in text.lines() {
        let (x, tmp) = line.split_once(',').unwrap();
        let (y, z) = tmp.split_once(',').unwrap();
        let [x, y, z] = [x, y, z].map(|s| s.parse::<usize>().unwrap());
        coords.push((x, y, z))
    }

    let mut distances = vec![];

    for (index_a, a) in coords.iter().enumerate() {
        for (index_b, b) in coords.iter().enumerate().skip(index_a + 1) {
            distances.push((euclid_squared(a, b), (index_a, index_b)));
        }
    }

    distances.sort();
    assert_eq!(distances.len(), coords.len() * (coords.len() - 1) / 2);
    assert!(
        distances[n_closest - 1].0 < distances[n_closest].0,
        "If this is not true, than the answer could be ambiguous since the connected circuits could be different"
    );

    let mut circuits = DisjointSetWithCount::new(coords.len());
    for (_, (index_a, index_b)) in distances[..n_closest].into_iter() {
        circuits.link(*index_a, *index_b);
    }

    let mut circuits = circuits
        .into_eves()
        .into_iter()
        .map(|count| *count)
        .collect_vec();
    circuits.sort();

    return circuits[circuits.len() - 3..]
        .into_iter()
        .fold(1, |prod, n| prod * n);
}

fn part2(text: &str) -> usize {
    let mut coords = vec![];

    for line in text.lines() {
        let (x, tmp) = line.split_once(',').unwrap();
        let (y, z) = tmp.split_once(',').unwrap();
        let [x, y, z] = [x, y, z].map(|s| s.parse::<usize>().unwrap());
        coords.push((x, y, z))
    }

    let mut distances = vec![];

    for (index_a, a) in coords.iter().enumerate() {
        for (index_b, b) in coords.iter().enumerate().skip(index_a + 1) {
            distances.push((euclid_squared(a, b), (index_a, index_b)));
        }
    }

    distances.sort();
    assert_eq!(distances.len(), coords.len() * (coords.len() - 1) / 2);

    let mut circuits = DisjointSetWithCount::new(coords.len());
    let mut index_a = 0;
    let mut index_b = 0;

    for (_, indexes) in distances.into_iter() {
        (index_a, index_b) = indexes;
        circuits.link(index_a, index_b);
        if circuits.size_of_eve(index_a) == coords.len() {
            break;
        }
    }

    return coords[index_a].0 * coords[index_b].0;
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day8.txt")?;

    println!("part 1 result = {:?}", part1(&text, 1000));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1, 10), 40);
    }

    #[test]
    fn part2_given_example() {
        assert_eq!(crate::part2(TEXT1), 25272);
    }
}
