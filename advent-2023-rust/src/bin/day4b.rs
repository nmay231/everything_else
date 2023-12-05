use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let text = read_to_string("./assets/day4.txt")?;

    let len = text.lines().count();
    let mut accumulation = vec![1 as usize; len];

    for (card_number, line) in text.lines().enumerate() {
        let all = line[10..]
            .split(" ")
            .filter(|x| x != &"" && x != &"|")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let winning = &all[..10];
        let our_numbers = &all[10..];

        let overlap = our_numbers.iter().filter(|n| winning.contains(n)).count();

        let current_copy_count = accumulation[card_number];
        for copied_scratchcards in card_number + 1..=card_number + overlap {
            accumulation[copied_scratchcards] += current_copy_count;
        }
    }

    println!(
        "number of scratchcards = {}",
        accumulation.iter().sum::<usize>()
    );

    Ok(())
}
