use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let text = read_to_string("./assets/day4.txt")?;

    let mut sum = 0;

    for line in text.lines() {
        let all = line[10..]
            .split(" ")
            .filter(|x| x != &"" && x != &"|")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let winning = &all[..10];
        let our_numbers = &all[10..];

        let overlap = our_numbers.iter().filter(|n| winning.contains(n)).count();
        if overlap > 0 {
            sum += (2 as u32).pow(overlap as u32 - 1);
        }
    }

    println!("sum = {}", sum);

    Ok(())
}
