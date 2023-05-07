use std::fs::read_to_string;

fn keep_max_descending(vector: &mut Vec<u32>, val: u32) {
    let mut val = val;
    // Keep vector sorted descending
    for i in 0..vector.len() {
        if val > vector[i] {
            (vector[i], val) = (val, vector[i]);
        }
    }
}

fn main() {
    let text = read_to_string("./assets/day1.txt").unwrap();
    let mut maxes: Vec<u32> = vec![0, 0, 0];
    let mut elf = 0;

    for line in text.lines() {
        // let num = ;
        match line.parse::<u32>() {
            Ok(num) => elf += num,
            Err(_) => {
                if elf > 0 {
                    keep_max_descending(&mut maxes, elf);
                    elf = 0;
                }
            }
        }
    }
    println!("{maxes:?}.sum() = {}", maxes.iter().sum::<u32>())

    // println!("{}", text.lines().next().unwrap())
}
