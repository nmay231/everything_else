use regex::Regex;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let handful = Regex::from_str(r"((\d+) (green|red|blue),? ?)+;?").unwrap();
    let color_count = Regex::from_str(r"(\d+) (green|red|blue),? ?").unwrap();

    let text = read_to_string("./assets/day2.txt").unwrap();
    let mut sum = 0;

    for line in text.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for mat in handful.find_iter(line) {
            for color in color_count.captures_iter(mat.as_str()) {
                let (_, pair) = color.extract::<2>();
                let count: u32 = (pair[0]).parse().unwrap();
                let color = pair[1];

                match color {
                    "red" => red = std::cmp::max(red, count),
                    "green" => green = std::cmp::max(green, count),
                    "blue" => blue = std::cmp::max(blue, count),
                    _ => panic!("color mismatch {}", color),
                };
            }
        }
        sum += red * green * blue;
    }
    println!("sum = {}", sum);
}
