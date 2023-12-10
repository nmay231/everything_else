use std::collections::HashMap;
use std::time::SystemTime;

// Well, this solution now works with n == 1_000_000_000, but I had to look at all the hints
// to realize reserved_seats never goes above 10_000
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut slots_main = HashMap::new();

        for seat in &reserved_seats {
            let row = slots_main.entry(seat[0] as usize - 1).or_insert(7);
            if *row == 0 {
                continue;
            } else if seat[1] >> 1 == 1 {
                *row &= 0b011
            } else if seat[1] >> 1 == 2 {
                *row &= 0b001
            } else if seat[1] >> 1 == 3 {
                *row &= 0b100
            } else if seat[1] >> 1 == 4 {
                *row &= 0b110
            }
        }

        let mut sum = (n as i32) * 2;
        for slot in slots_main.values() {
            let slot1 = slot & 0b100 > 0;
            let slot2 = slot & 0b010 > 0;
            let slot3 = slot & 0b001 > 0;
            sum -= if slot1 && slot3 {
                0
            } else if slot2 || slot1 || slot3 {
                1
            } else {
                2
            };
        }
        return sum;
    }
}

struct Solution;

fn main() {
    let seats = vec![[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]]
        .iter()
        .map(|pair| pair.to_vec())
        .collect();

    let start = SystemTime::now();
    println!("{}", Solution::max_number_of_families(3, seats));
    println!(
        "total: {}ms",
        SystemTime::now().duration_since(start).unwrap().as_millis()
    );
}
