pub struct CoinChange {
    coins: PerCoin,
    goal: usize,
}

struct PerCoin {
    coin_value: usize,
    coin_count: usize,
    next: Option<Box<PerCoin>>,
}

impl PerCoin {
    /// Prioritizing right-most denominations, generate the next possible set of
    /// coins that sums to the goal
    fn next_trailing_counts(&mut self, goal: usize) -> Option<Vec<usize>> {
        loop {
            match goal.checked_sub(self.coin_value * self.coin_count) {
                // We've exceeded the sum
                None => {
                    self.reset_trailing_counts();
                    return None;
                }
                // We got even change on the last coin
                Some(0) if self.next.is_none() => {
                    self.coin_count += 1;
                    return Some(vec![self.coin_count - 1]);
                }

                Some(remaining) => {
                    if self.next.is_none() {
                        self.coin_count += 1;
                    } else if let Some(mut next_coin) = self.next.take() {
                        let next_counts = next_coin.next_trailing_counts(remaining);
                        self.next = Some(next_coin);

                        match next_counts {
                            None => {
                                self.coin_count += 1;
                            }
                            Some(mut trailing_counts) => {
                                trailing_counts.push(self.coin_count);
                                return Some(trailing_counts);
                            }
                        }
                    }
                }
            }
        }
    }

    fn reset_trailing_counts(&mut self) {
        self.coin_count = 0;
        if let Some(ref mut next) = self.next {
            next.reset_trailing_counts();
        }
    }
}

impl CoinChange {
    pub fn new(face_values: &[usize], sum: usize) -> Self {
        let mut per_coin = None;
        for coin in face_values {
            per_coin = Some(PerCoin {
                coin_count: 0,
                coin_value: *coin,
                next: per_coin.and_then(|per_coin| Some(Box::new(per_coin))),
            });
        }
        Self {
            coins: per_coin.expect("To have at least one coin"),
            goal: sum,
        }
    }
}

impl Iterator for CoinChange {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.coins.next_trailing_counts(self.goal)
    }
}

pub struct CoinChangePair {
    goal: usize,
    coin_a: usize,
    coin_b: usize,
    num_a: usize,
}

impl CoinChangePair {
    pub fn try_new(coin_a: usize, coin_b: usize, goal: usize) -> Self {
        let num_a = coin_a * (goal / coin_a);
        Self {
            goal,
            coin_a,
            coin_b,
            num_a,
        }
    }
}

impl Iterator for CoinChangePair {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test_coin_change {
    use std::assert_matches::assert_matches;
    use std::time::Duration;

    use itertools::Itertools;
    use rstest::rstest;

    use crate::CoinChange;

    #[rstest]
    #[case::zero(0, vec![1], 1)]
    #[case::one(1, vec![1], 1)]
    #[case::ten(10, vec![1], 1)]
    // The game of nim studied in game theory (aka the subtraction game version)
    #[case::nim_1(0, vec![1, 2], 1)]
    #[case::nim_1(1, vec![1, 2], 1)]
    #[case::nim_2(2, vec![1, 2], 2)]
    #[case::nim_3(3, vec![1, 2], 2)]
    #[case::nim_4(4, vec![1, 2], 3)]
    #[case::nim_5(5, vec![1, 2], 3)]
    #[case::nim_6(6, vec![1, 2], 4)]
    #[case::nim_7(7, vec![1, 2], 4)]
    #[case::nim_8(8, vec![1, 2], 5)]
    #[case::nim_9(9, vec![1, 2], 5)]
    #[case::impossible(11, vec![4, 5], 0)]
    #[case::impossible(11, vec![5, 4], 0)]
    #[case::quarter_dime_penny(26, vec![25, 10, 1], 4)]
    #[timeout(Duration::from_secs(5))]
    fn asdf(#[case] goal: usize, #[case] coins: Vec<usize>, #[case] n_sequences: usize) {
        // TODO: Use rstest_reuse to make separate test functions for each of
        // these sub-tests

        // Sum matches the goal
        for counts in CoinChange::new(&coins, goal) {
            let sum = counts
                .iter()
                .zip_eq(&coins)
                .map(|(count, value)| count * value)
                .sum::<usize>();
            assert_eq!(sum, goal);
        }

        // The sequences are produced in a certain order with earlier coins being preferred
        let mut prev_sequence: Option<Vec<usize>> = None;
        for counts in CoinChange::new(&coins, goal) {
            if let Some(prev) = prev_sequence {
                let first_deviation =
                    prev.iter()
                        .zip(&counts)
                        .rfold(None, |first_difference, (prev, current)| {
                            first_difference.or_else(|| {
                                if prev == current {
                                    None
                                } else {
                                    Some((prev, current))
                                }
                            })
                        });

                // The sequences are not the same (there is at least one change)
                // and the change is fewer of the right-most denomination coins
                // than in the previous sequence
                assert_matches!(first_deviation, Some((a, b)) if a < b);
            }
            prev_sequence = Some(counts);
        }

        // We know how many valid sequences
        let actual = CoinChange::new(&coins, goal)
            .map(|counts| {
                println!("counts{:?}", (counts));
            })
            .count();
        assert_eq!(actual, n_sequences);
    }
}
