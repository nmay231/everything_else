use std::str::FromStr;

use regex::Regex;

type Output = usize;

fn part1(text: &str) -> Output {
    // Sometimes a regex IS the right answer!
    let mul = Regex::from_str(r"mul\(\d+,\d+\)").unwrap();
    let mut result = 0;
    for mat in mul.find_iter(text) {
        let sub = mat.as_str();
        let sub = &sub[4..].trim_end_matches(')');
        let (a, b) = sub
            .split_once(',')
            .expect("to have two numbers split by comma");
        let a: usize = a.parse().unwrap();
        let b: usize = b.parse().unwrap();
        result += a * b;
    }
    result
}

fn part2(text: &str) -> Output {
    let mul = Regex::from_str(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let mut enabled = true;

    for mat in mul.find_iter(text) {
        match mat.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if !enabled => continue,
            sub => {
                let sub = &sub[4..].trim_end_matches(')');
                let (a, b) = sub
                    .split_once(',')
                    .expect("to have two numbers split by comma");
                let a: usize = a.parse().unwrap();
                let b: usize = b.parse().unwrap();
                result += a * b;
            }
        }
    }
    result
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day3.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}
