use std::fs;

struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        if nums.len() == 0 {
            return 0;
        }
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<i64>>();
        let k = k.abs() as i64;
        let mut remainder = vec![0i64; k as usize];
        for number in nums.iter() {
            remainder[number.rem_euclid(k) as usize] += 1;
        }

        let mut total_pairs: i64 = if remainder[0] > 0 {
            remainder[0] * (nums.len() as i64 - remainder[0]) + choose_2(remainder[0])
        } else {
            0
        };

        // println!("{k};; {total_pairs}; {:?}", remainder);

        let factors = prime_factors_of_k(k);
        assert!(factors.len() > 0);
        if factors.len() == 1 {
            // println!("{k};;;;; {factors:?}");
            return total_pairs;
        } else if factors.len() <= k.ilog2() as usize + 1 {
            let old = remainder;
            remainder = vec![0; k as usize];

            for (a, b) in factors.iter().zip(factors.iter().skip(1).chain([0].iter())) {
                if a == b {
                    continue; // No need to check the same prime factor more than once
                }

                let a = *a as usize;
                for i in (a..k as usize).step_by(a) {
                    remainder[i] = old[i];
                }
            }
        }
        // assert_eq!(factors.iter().product::<i64>(), k);
        // println!("{k};; {total_pairs}; {:?}", remainder);

        for (i, a) in remainder.iter().enumerate().skip(2) {
            if a == &0 {
                continue;
            }
            for (j, b) in remainder.iter().enumerate().skip(i) {
                if (i * j) as i64 % k == 0 {
                    if i == j {
                        total_pairs += choose_2(*a);
                    } else {
                        total_pairs += (a) * (b);
                    }
                    // println!("{total_pairs}, {i}:{j}");
                }
            }
        }
        return total_pairs;
    }
}

fn choose_2(k: i64) -> i64 {
    if k < 2 {
        return 0;
    }
    return k * (k - 1) / 2;
}
fn _choose(n: i64, k: i64) -> i64 {
    if k > n {
        return 0;
    }
    assert!(n > 0 && k > -1);
    let min = k.min(n - k);
    let max = k.max(n - k);

    return (max + 1..=n).product::<i64>() / (1..=min).product::<i64>();
}

fn prime_factors_of_k(k: i64) -> Vec<i64> {
    let max_factor = (k as f64).sqrt().ceil() as i64;
    for factor in 2..=max_factor {
        if k % factor == 0 {
            let mut inner = prime_factors_of_k(k / factor);
            inner.push(factor);
            return inner;
        }
    }
    return vec![k];
}

fn main() {
    // println!("{:?}", prime_factors_of_k(44002));
    // // return;
    // assert_eq!(
    //     Solution::count_pairs(vec![10, 10, 6, 9, 3, 7, 4, 3, 8, 8], 4),
    //     27
    // );

    // assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4, 5], 2), 7);
    // assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 5), 0);
    // assert_eq!(
    //     Solution::count_pairs(vec![8, 10, 2, 5, 9, 6, 3, 8, 2], 6),
    //     18
    // );
    // assert_eq!(Solution::count_pairs(vec![5, 10, 4, 5, 8, 3], 8), 6);

    // let x: Vec<i32> = serde_json::from_str(&fs::read_to_string("./tmp.txt").unwrap()).unwrap();
    // println!("{}", Solution::count_pairs(x, 55503));

    let x: Vec<i32> = serde_json::from_str(&fs::read_to_string("./tmp2.txt").unwrap()).unwrap();
    assert_eq!(Solution::count_pairs(x, 44002), 20711);

    println!("success?");
    // println!(
    //     "{}",
    //     ,
    //     // Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7),
    // );
}
