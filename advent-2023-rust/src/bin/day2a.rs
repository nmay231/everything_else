use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    lazy_static! {
        static ref HANDFUL: Regex = Regex::from_str(r"((\d+) (green|red|blue),? ?)+;?").unwrap();
        static ref COLOR_COUNT: Regex = Regex::from_str(r"(\d+) (green|red|blue),? ?").unwrap();
    }
    let text = read_to_string("./assets/day2.txt").unwrap();
    let mut sum = 0;

    for (i, line) in text.lines().enumerate() {
        let mut possible_game = true;
        for mat in HANDFUL.find_iter(line) {
            for color in COLOR_COUNT.captures_iter(mat.as_str()) {
                let (_, pair) = color.extract::<2>();
                let count: u32 = (pair[0]).parse().unwrap();
                let color = pair[1];

                possible_game = match color {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => panic!("color mismatch {}", color),
                };

                // println!("{possible_game}, {count}, {color}");

                if !possible_game {
                    break;
                }
            }

            if !possible_game {
                break;
            }
            // println!("{}", mat.as_str());
        }
        if possible_game {
            sum += i + 1;
        }
    }
    println!("sum = {}", sum);
}
