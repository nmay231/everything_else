use std::ops::IndexMut;
use std::thread;
use std::time::SystemTime;

// TODO: This seems to work even with large examples (the first one takes 6 seconds, which I don't know if that's good or bad), but I can't submit it because thread::scope doesn't exist in their rust compiler. Why do they have an example with trillions of numbers? *&$#! them!
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut slots_main = vec![7; n];

        for seat in &reserved_seats {
            let row = slots_main.index_mut(seat[0] as usize - 1);
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

        return thread::scope(|main| {
            let mut handlers = vec![];
            for chunk in slots_main.chunks(1_000_000) {
                handlers.push(main.spawn(|| {
                    return chunk
                        .iter()
                        .map(|slot| {
                            let slot1 = slot & 0b100 > 0;
                            let slot2 = slot & 0b010 > 0;
                            let slot3 = slot & 0b001 > 0;
                            if slot1 && slot3 {
                                2
                            } else if slot2 || slot1 || slot3 {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<i32>();
                }));
            }
            return handlers.into_iter().map(|h| h.join().unwrap()).sum::<i32>();
        });
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
