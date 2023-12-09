use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day7.txt")?;
    let card_values = "J23456789TQKA"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<_, _>>();

    let mut hands = text
        .lines()
        .map(|line| {
            if let [hand, bid] = line.split(" ").collect::<Vec<_>>()[..] {
                let mut map = HashMap::new();
                for card in hand.chars() {
                    map.entry(card).and_modify(|x| *x += 1).or_insert(1);
                }

                let jokers = map.remove(&'J').or(Some(0)).unwrap();
                let mut counts = map.values().collect::<Vec<_>>();
                counts.sort_unstable();

                // value = zip(1..=7, [high card, one pair, two pair, 3 of a kind, full house, 4 of a kind, 5 of a kind])
                let max = counts.last();
                let next_max = counts.iter().nth_back(1);
                let value = match (max, next_max, jokers) {
                    (_, _, 5) => 7,

                    (Some(1), _, 0 | 1) => 1 + jokers,
                    (Some(1), _, 2 | 3) => 2 * jokers,
                    (Some(1), _, 4) => 7,

                    (Some(2), Some(2), 0 | 1) => 3 + 2 * jokers,
                    (Some(2), _, 0 | 1 | 2) => 2 + 2 * jokers,
                    (Some(2), _, 3) => 7,

                    (Some(3), Some(2), _) => 5,
                    (Some(3), _, 0) => 4,
                    (Some(3), _, 1 | 2) => 6 + jokers - 1,

                    (Some(max @ (4 | 5)), _, _) => *max + 2 + jokers,

                    _ => panic!("Unforeseen pattern with: counts={counts:?}; jokers={jokers}"),
                };

                let bid = bid.parse::<u32>().unwrap();
                let hand_vec = hand
                    .chars()
                    .map(|char| card_values.get(&char).unwrap())
                    .collect::<Vec<_>>();

                return (value, (hand_vec, hand), bid);
            } else {
                panic!("line doesn't contain hand and bid: {line}");
            }
        })
        .collect::<Vec<_>>();

    hands.sort();

    println!(
        "total winnings = {}",
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (_, _, bid))| { acc + (i + 1) * *bid as usize })
    );

    Ok(())
}
