use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day7.txt")?;
    let card_values = "23456789TJQKA"
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

                let mut counts = map.values().collect::<Vec<_>>();
                counts.sort_unstable();
                let bid = bid.parse::<u32>().unwrap();
                let hand = hand
                    .chars()
                    .map(|char| card_values.get(&char).unwrap())
                    .collect::<Vec<_>>();

                return match counts.last().unwrap() {
                    1 => (1, hand, bid),
                    2 => {
                        if counts.len() == 4 {
                            (2, hand, bid)
                        } else {
                            (3, hand, bid)
                        }
                    }
                    3 => {
                        if counts.len() == 3 {
                            (4, hand, bid)
                        } else {
                            (5, hand, bid)
                        }
                    }
                    max @ (4 | 5) => (*max + 2, hand, bid),
                    _ => panic!("didn't have 5 cards"),
                };
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
