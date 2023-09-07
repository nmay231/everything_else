use std::fs::File;
use std::io::{self, BufRead};

#[derive(Default, Debug)]
struct CratePiles {
    piles: Vec<Vec<char>>,
}

impl CratePiles {
    fn flip(&mut self) {
        // Used once at the start after parsing
        for pile in self.piles.iter_mut() {
            pile.reverse()
        }
    }

    fn transfer(&mut self, first_pile: usize, second_pile: usize, amount: u32) {
        for _ in 0..amount {
            let tmp = self.piles[first_pile].pop().unwrap();
            self.piles[second_pile].push(tmp);
        }
    }

    fn pile_tops(&self) -> String {
        self.piles
            .iter()
            .map(|pile| pile.iter().last().unwrap().to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

fn main() -> Result<(), &'static str> {
    let f = File::open("./assets/day5.txt").or(Err("File missing or unreadable"))?;
    let mut lines = io::BufReader::new(f).lines().peekable();
    let mut piles = CratePiles::default();

    let first = lines.peek().expect("Empty file").as_ref().unwrap();
    let pile_count = first.len() / 4 + 1;
    for _ in 0..pile_count {
        piles.piles.push(vec![]);
    }

    for line in &mut lines {
        let line = line.as_ref().unwrap().as_str();
        if line.chars().nth(1) == Some('1') {
            break;
        }

        for (index, crate_) in line.chars().enumerate().filter(|(i, _)| i % 4 == 1) {
            if crate_ != ' ' {
                piles.piles[index / 4].push(crate_);
            }
        }
    }

    lines.next(); // Skip blank line in file
    piles.flip(); // Piles were stacked in reverse order, classic inverted y-axis problem

    for line in lines {
        // format: `move 4 from 9 to 6`
        let mut parts = line.as_ref().unwrap().as_str().split(" ");
        let amount = parts
            .nth(1)
            .expect("bad line")
            .parse::<u32>()
            .expect("Expected number for `amount`");
        let from = (parts
            .nth(1)
            .expect("bad line")
            .parse::<i32>()
            .expect("Expected number for `from`")
            - 1) as usize;
        let to = (parts
            .nth(1)
            .expect("bad line")
            .parse::<i32>()
            .expect("Expected number for `to`")
            - 1) as usize;

        piles.transfer(from, to, amount);
    }

    println!("{:?}", piles);
    println!("{}", piles.pile_tops());

    Ok(())
}
