use std::fs::read_to_string;

fn main() {
    let text = read_to_string("./assets/day1.txt").unwrap();
    println!("{}", text.lines().next().unwrap());
}
