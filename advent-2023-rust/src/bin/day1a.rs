use std::fs::read_to_string;

fn main() {
    let text = read_to_string("./assets/day1.txt").unwrap();
    let mut sum = 0;
    for line in text.lines() {
        let mut first = None;
        let mut last = 0;
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                if first == None {
                    first = Some(digit);
                }
                last = digit;
            }
        }

        sum += (first.unwrap()) * 10 + last;
    }
    println!("The sum is: {}", sum);
}
