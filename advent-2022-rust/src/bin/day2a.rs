use std::fs::read_to_string;

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn score_against(&self, opponent: RPS) -> i32 {
        let diff = self.value() - opponent.value();
        match (diff + 3) % 3 {
            2 => 0, // Loss
            0 => 3, // Tie
            1 => 6, // Win
            unexpected => panic!("Unexpected value modulus 3: {unexpected}"),
        }
    }
}

fn main() {
    let mut turns = Vec::<(RPS, RPS)>::new();

    let text = read_to_string("./assets/day2.txt").expect("File not found");
    for line in text.lines() {
        if line.len() != 3 {
            panic!("Line not 3 chars long");
        }
        if let [them, _, us] = line.chars().collect::<Vec<char>>()[..] {
            let them = match them.to_ascii_uppercase() {
                'A' => RPS::Rock,
                'B' => RPS::Paper,
                'C' => RPS::Scissors,
                chr => panic!("Unexpected character for them: {chr}"),
            };
            let us = match us.to_ascii_uppercase() {
                'X' => RPS::Rock,
                'Y' => RPS::Paper,
                'Z' => RPS::Scissors,
                chr => panic!("Unexpected character for us: {chr}"),
            };
            turns.push((them, us));
        } else {
            panic!("Did not get a pair of chars! `{line}`");
        }
    }

    let score = turns.into_iter().fold(0, |sum, (them, us)| {
        sum + us.value() + us.score_against(them)
    });

    println!("part 1 answer: {score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scores() {
        assert_eq!(RPS::Rock.score_against(RPS::Paper), 0);
        assert_eq!(RPS::Paper.score_against(RPS::Scissors), 0);
        assert_eq!(RPS::Scissors.score_against(RPS::Rock), 0);

        assert_eq!(RPS::Rock.score_against(RPS::Rock), 3);
        assert_eq!(RPS::Paper.score_against(RPS::Paper), 3);
        assert_eq!(RPS::Scissors.score_against(RPS::Scissors), 3);

        assert_eq!(RPS::Rock.score_against(RPS::Scissors), 6);
        assert_eq!(RPS::Paper.score_against(RPS::Rock), 6);
        assert_eq!(RPS::Scissors.score_against(RPS::Paper), 6);
    }
}
