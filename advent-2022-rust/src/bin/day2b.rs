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
        return ((diff + 4) % 3) * 3;
    }
}

fn main() {
    let mut turns = Vec::<(RPS, RPS)>::new();

    let text = read_to_string("./assets/day2.txt").expect("File not found");
    for line in text.lines() {
        if line.len() != 3 {
            panic!("Line not 3 chars long");
        }
        if let [them, _, predicted_score] = line.chars().collect::<Vec<char>>()[..] {
            let them = match them.to_ascii_uppercase() {
                'A' => RPS::Rock,
                'B' => RPS::Paper,
                'C' => RPS::Scissors,
                chr => panic!("Unexpected character for them: {chr}"),
            };
            let predicted_score = match predicted_score.to_ascii_uppercase() {
                'X' => 0, // Loss
                'Y' => 3, // Draw
                'Z' => 6, // Win
                chr => panic!("Unexpected character for predicted_score: {chr}"),
            };

            // If `score / 3 == (us - them + 1) (mod 3)` then `us == score / 3 + them - 1 (mod 3)`
            let us = (predicted_score / 3 + them.value() + 2) % 3;
            let us = match us {
                1 => RPS::Rock,
                2 => RPS::Paper,
                0 => RPS::Scissors, // 0 == 3 (mod 3)
                unexpected => panic!("Should have gotten a value [0..3): {unexpected}"),
            };
            turns.push((them, us));
        } else {
            panic!("Did not get a pair of chars! `{line}`");
        }
    }

    let score = turns.into_iter().fold(0, |sum, (them, us)| {
        sum + us.value() + us.score_against(them)
    });

    println!("part 2 answer: {score}");
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
