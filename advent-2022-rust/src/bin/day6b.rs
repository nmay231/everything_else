use std::fs::read_to_string;

fn main() -> Result<(), &'static str> {
    let text = read_to_string("./assets/day6.txt").or(Err("File missing or unreadable"))?;
    let chars: Vec<char> = text.chars().collect();
    let window_width = 14;

    'main: for (outer, window) in chars.windows(window_width).enumerate() {
        for (inner, a) in window.iter().enumerate() {
            for b in window.iter().skip(inner + 1) {
                if a == b {
                    continue 'main;
                }
            }
        }

        println!(
            "first instance: {} '{}'",
            outer + window_width,
            String::from_iter(window),
        );
        break;
    }

    Ok(())
}
