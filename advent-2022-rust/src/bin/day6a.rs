use std::fs::read_to_string;

fn main() -> Result<(), &'static str> {
    let text = read_to_string("./assets/day6.txt").or(Err("File missing or unreadable"))?;
    let chars: Vec<char> = text.chars().collect();
    'main: for (outer, c4) in chars.windows(4).enumerate() {
        for (inner, a) in c4.iter().enumerate() {
            for b in c4.iter().skip(inner + 1) {
                if a == b {
                    continue 'main;
                }
            }
        }

        println!("first instance: {} '{}'", outer + 4, String::from_iter(c4));
        break;
    }

    Ok(())
}
