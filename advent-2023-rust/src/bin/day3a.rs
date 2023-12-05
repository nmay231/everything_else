use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn number_is_relevant(top: &str, middle: &str, bottom: &str, slice: Range<usize>) -> bool {
    let start = slice.start;
    let end = slice.end;
    if start > 0 && middle.chars().nth(start - 1).unwrap() != '.' {
        return true;
    } else if let Some(symbol) = middle.chars().nth(end) {
        if symbol != '.' {
            return true;
        }
    }
    let min = if start == 0 { 0 } else { start - 1 };
    let min_top = std::cmp::min(min, top.len() - 1);
    let min_bottom = std::cmp::min(min, bottom.len() - 1);
    let max_top = std::cmp::min(end + 1, top.len());
    let max_bottom = std::cmp::min(end + 1, bottom.len());

    let is_symbol = |c: char| c != '.' && !c.is_ascii_digit();
    return top[min_top..max_top].chars().any(is_symbol)
        || bottom[min_bottom..max_bottom].chars().any(is_symbol);
}

fn main() -> io::Result<()> {
    let f = File::open("./assets/day3.txt")?;
    let mut lines = io::BufReader::new(f).lines().chain([Ok(".".into())]);
    let mut top = ".".to_string();
    let mut middle = lines.next().unwrap().unwrap();

    let mut sum = 0;

    for bottom in lines {
        let bottom = bottom.unwrap();
        let mut digits = None;
        let mut start_index = 0;

        for (i, char) in middle.chars().chain(['.']).enumerate() {
            match (char.is_ascii_digit(), &mut digits) {
                (true, None) => {
                    start_index = i;
                    digits = Some(char.to_string())
                }
                (true, Some(ref mut str)) => str.push(char),

                (false, None) => (),
                (false, Some(ref str)) => {
                    let number: u32 = str.parse().unwrap();
                    digits = None;

                    if number_is_relevant(&top, &middle, &bottom, start_index..i) {
                        sum += number;
                        // color debug: print!("\x1b[1;32m{number}\x1b[0m{char}");
                    }
                    // color debug: else { print!("\x1b[1;31m{number}\x1b[0m{char}"); } continue;
                }
            }

            // color debug: if !char.is_ascii_digit() {print!("{}", char);}
        }
        // color debug: println!();

        (top, middle) = (middle, bottom);
    }
    println!("sum = {}", sum);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::number_is_relevant;

    #[test]
    fn should_not_have_false_positives() {
        let top = "!...!";
        let mid = "!.5.!";
        let bot = "!...!";
        assert!(!number_is_relevant(top, mid, bot, 2..3))
    }
}
