use std::fs::read_to_string;

fn main() {
    let digits1 = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .map(|s| s.to_owned());
    let digits2 = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(|int| int.to_string());

    let text = read_to_string("./assets/day1.txt").unwrap();
    let mut sum = 0;
    for line in text.lines() {
        let mut first = (line.len(), 0);
        let mut second = (0, 0);

        for (i, digit) in digits2.iter().enumerate().chain(digits1.iter().enumerate()) {
            let digit_value = i + 1;
            if let Some(x) = line.find(digit) {
                if x <= first.0 {
                    first = (x, digit_value);
                }
            }
            if let Some(x) = line.rfind(digit) {
                if x >= second.0 {
                    second = (x, digit_value);
                }
            }
        }

        sum += 10 * first.1 + second.1;
    }
    println!("The sum is: {}", sum);
}
