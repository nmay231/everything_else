use std::collections::HashSet;
use std::fs::read_to_string;

fn priority_of_char(ch: &char) -> usize {
    if ch <= &'Z' {
        *ch as usize - 64 + 26
    } else {
        *ch as usize - 96
    }
}

fn main() {
    let text = read_to_string("./assets/day3.txt").expect("file not found or readable");
    let mut priority = 0;

    for line in text.lines() {
        let mid = line.len() / 2;
        let (first, second) = line.split_at(mid);

        let first: HashSet<_> = first.chars().into_iter().collect();
        let second: HashSet<_> = second.chars().into_iter().collect();
        let mut inter = first.intersection(&second);

        let shared = inter
            .next()
            .expect("Should be at least one shared character");
        assert!(
            inter.next().is_none(),
            "Should have no more than one shared character"
        );

        priority += priority_of_char(shared);
    }

    println!("part 1 answer: priority = {}", priority);
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
