use std::fs::read_to_string;

fn main() {
    let text = read_to_string("./assets/day1.txt").unwrap();
    let mut max = 0;
    let mut elf = 0;

    for line in text.lines() {
        // let num = ;
        match line.parse::<u32>() {
            Ok(num) => elf += num,
            Err(_) => {
                if elf > 0 {
                    max = if max > elf { max } else { elf };
                    elf = 0;
                }
            }
        }
    }
    println!("{max}")

    // println!("{}", text.lines().next().unwrap())
}
