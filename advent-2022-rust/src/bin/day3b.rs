use std::collections::HashSet;
use std::fs::read_to_string;

fn priority_of_char(ch: &char) -> usize {
    if ch <= &'Z' {
        *ch as usize - 64 + 26
    } else {
        *ch as usize - 96
    }
}

fn main() -> Result<(), &'static str> {
    let text = read_to_string("./assets/day3.txt").or(Err("file not found or readable"))?;
    let lines: Vec<_> = text.lines().collect();
    let mut priority = 0;

    // I wish Iterators had .chunk and .chuck_exact
    for chunk in lines.chunks_exact(3) {
        if let [a, b, c] = chunk[..] {
            let a: HashSet<char> = a.chars().collect();
            let b: HashSet<char> = b.chars().collect();
            let c: HashSet<char> = c.chars().collect();

            let inter: HashSet<char> = a.intersection(&b).map(|v| *v).collect();
            let inter: Vec<&char> = c.intersection(&inter).collect();
            let shared = inter
                .get(0)
                .ok_or("Should be at least one shared character")?;
            if inter.len() > 1 {
                return Err("Should have no more than one shared character");
            }

            priority += priority_of_char(shared);
        } else {
            return Err("chunks_error didn't do what it should");
        }
    }

    println!("part 2 answer: priority = {}", priority);
    Ok(())
}

#[cfg(test)]
mod test_priority_of_char {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(priority_of_char(&'a'), 1);
    }

    #[test]
    fn test_z() {
        assert_eq!(priority_of_char(&'z'), 26);
    }

    #[test]
    fn test_cap_a() {
        assert_eq!(priority_of_char(&'A'), 26 + 1);
    }

    #[test]
    fn test_cap_z() {
        assert_eq!(priority_of_char(&'Z'), 26 + 26);
    }
}
