use std::ops::{Deref, Neg};

use num_bigint::{BigInt, BigUint, ToBigInt};

type Output = usize;

fn part1(text: &str) -> Output {
    let mut tmp = vec![];
    for line in text.lines() {
        let (x, line) = line.split_once(", ").unwrap();
        let (y, line) = line.split_once(", ").unwrap();
        let (z, line) = line.split_once(" @ ").unwrap();
        let (dx, line) = line.split_once(", ").unwrap();
        let (dy, dz) = line.split_once(", ").unwrap();
        let [x, y, _z] = [x, y, z].map(|s| s.parse::<usize>().unwrap());
        let [dx, dy, _dz] = [dx.trim(), dy.trim(), dz.trim()].map(|s| s.parse::<i32>().unwrap());

        // Thank you for making all deltas non-zero
        assert!(dx != 0);
        assert!(dy != 0);
        assert!(_dz != 0);
        tmp.push((x, y, dx, dy, dy as f64 / dx as f64));
    }

    let mut inside = 0;
    let mut number = 0;
    for (ai, a) in tmp.iter().enumerate() {
        for b in tmp.iter().skip(ai + 1) {
            number += 1;

            let (x1, y1, dx1, _dy1, m1) = *a;
            let (x2, y2, dx2, _dy2, m2) = *b;
            let c1 = y1 as f64 - m1 * x1 as f64;
            let c2 = y2 as f64 - m2 * x2 as f64;

            if m1 == m2 {
                if c1 == c2 {
                    panic!(
                        "Paths lie on each other: {:?}, {:?}",
                        (x1, y1, m1, c1),
                        (x2, y2, m2, c2)
                    );
                } else {
                    // println!("{:?}", (a, b, "parallel"));
                    continue;
                }
            }
            // y1 = m1*x1 + c1
            // y2 = m2*x2 + c2
            // c1 = y1 - m1*x1, etc.
            // When (x1, y1) = (x2, y2)
            // m1*x + c1 = m2*x + c2
            // x = (c2 - c1) / (m1 - m2)
            // y = ...

            const BOUNDS: (f64, f64) = (2e14, 4e14);

            let x_intersect = (c2 - c1) / (m1 - m2);
            let y_intersect = m1 * x_intersect + c1;

            let (_message, diff) = match (
                (x1 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx1),
                (x2 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx2),
            ) {
                (false, false) => ("both in past", 0),
                (false, true) => ("first in past", 0),
                (true, false) => ("second in past", 0),
                (true, true)
                    if x_intersect < BOUNDS.0
                        || x_intersect > BOUNDS.1
                        || y_intersect < BOUNDS.0
                        || y_intersect > BOUNDS.1 =>
                {
                    ("outside the test area", 0)
                }
                (true, true) => ("inside the test area", 1),
            };
            inside += diff
            // println!("{:?}", (x1, y1, x2, y2, x_intersect, y_intersect, _message));
        }
    }

    let len = tmp.len();
    println!("{number}; {len} => {}", len * (len - 1) / 2);
    inside
}

// MIT License: https://github.com/frewsxcv/rust-gcd/tree/8fb3a59184bc0672f6171363fd44860a3e3c9349
fn euclid(a: BigUint, b: BigUint) -> BigUint {
    // variable names based off euclidean division equation: a = b · q + r
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b != BigUint::from(0u32) {
        std::mem::swap(&mut a, &mut b);
        b %= &a;
    }

    a
}

#[derive(Debug, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
    z: T,
}
type BigPoint = Point<BigInt>;

impl BigPoint {
    fn new_from<T: Into<BigInt>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    fn dot_prod(&self, other: &Self) -> BigInt {
        &self.x * &other.x + &self.y * &other.y + &self.z * &other.z
    }

    fn cross_prod(&self, other: &Self) -> Self {
        Self {
            x: &self.y * &other.z - &self.z * &other.y,
            y: &self.z * &other.x - &self.x * &other.z,
            z: &self.x * &other.y - &self.y * &other.x,
        }
    }

    // Not impl Display, since that might be better as a "(x, y, z)"
    fn to_string(&self) -> String {
        format!("{}, {}, {}", self.x, self.y, self.z)
    }
}

impl<T: Into<BigInt>> From<(T, T, T)> for BigPoint {
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
        }
    }
}

impl Neg for BigPoint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -&self.x,
            y: -&self.y,
            z: -&self.z,
        }
    }
}

macro_rules! impl_big_point {
    ($Trait:ty, $op:ident) => {
        impl $Trait for &BigPoint {
            type Output = BigPoint;

            fn $op(self, rhs: Self) -> Self::Output {
                Self::Output {
                    x: (&self.x).$op(&rhs.x),
                    y: (&self.y).$op(&rhs.y),
                    z: (&self.z).$op(&rhs.z),
                }
            }
        }
    };
    (ref $Trait:ty, $op:ident) => {
        impl $Trait for BigPoint {
            type Output = BigPoint;

            fn $op(self, rhs: &Self) -> Self::Output {
                Self::Output {
                    x: self.x.$op(&rhs.x),
                    y: self.y.$op(&rhs.y),
                    z: self.z.$op(&rhs.z),
                }
            }
        }
    };
    ($Trait:ty, $op:ident, $other:ty) => {
        impl $Trait for &BigPoint {
            type Output = BigPoint;

            fn $op(self, rhs: $other) -> Self::Output {
                Self::Output {
                    x: (&self.x).$op(&rhs),
                    y: (&self.y).$op(&rhs),
                    z: (&self.z).$op(&rhs),
                }
            }
        }
    };
}

impl_big_point!(std::ops::Add, add);
impl_big_point!(std::ops::Sub, sub);
impl_big_point!(ref std::ops::Add<&Self>, add);
impl_big_point!(ref std::ops::Sub<&Self>, sub);
// impl_big_point!(std::ops::Mul, mul);
// impl_big_point!(std::ops::Div, div);

impl_big_point!(std::ops::Add<BigInt>, add, BigInt);
impl_big_point!(std::ops::Sub<BigInt>, sub, BigInt);
impl_big_point!(std::ops::Mul<BigInt>, mul, BigInt);
impl_big_point!(std::ops::Div<BigInt>, div, BigInt);

type Isize4 = (isize, isize, isize, isize);
fn no_future_collision_2d(a: Isize4, b: Isize4) -> bool {
    let (x1, y1, dx1, dy1) = a;
    let (x2, y2, dx2, dy2) = b;

    let m1 = dy1 as f64 / dx1 as f64;
    let m2 = dy2 as f64 / dx2 as f64;

    let c1 = y1 as f64 - m1 * x1 as f64;
    let c2 = y2 as f64 - m2 * x2 as f64;

    let x_intersect = (c2 - c1) / (m1 - m2);

    let first_collides_in_future = (x1 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx1);
    let second_collides_in_future = (x2 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx2);
    !first_collides_in_future || !second_collides_in_future
}

fn rock_delta(hailstones: &[(isize, isize, isize, isize, isize, isize)]) -> BigPoint {
    // First off, I tried solving the set of equations
    // (x1 + t1*dx1, y1 + t1*dy1, z1 + t1*dz1) = (x_n + t1*dx_n, y_n + t1*dy_n, z_n + t1*dz_n)
    // (x2 + t2*dx2, y2 + t2*dy2, z2 + t2*dz2) = (x_n + t2*dx_n, y_n + t2*dy_n, z_n + t2*dz_n)
    // (x3 + t3*dx3, y3 + t3*dy3, z3 + t3*dz3) = (x_n + t3*dx_n, y_n + t3*dy_n, z_n + t3*dz_n)
    // However, that gives a family of possible solutions to check (for any one
    // equation) since there is always a free variable.
    //
    // So then I had a long thought, couldn't think of anything reasonable
    // besides brute force, and looked at the AoC subreddit. I found an answer
    // that gave the idea of reframing the reference point. I'm going to use
    // that idea in a way that's different to their solution.
    //
    // I'm going to take two hailstones, A and B, that are parallel  and reframe
    // all trajectories based on one of them. Then by stepping the position of B
    // by 1 nanosecond at a time, I get a line between them that the rock I
    // throw would have to pass through (according to the adjusted frame of
    // reference). And since the rock has to hit every hailstone, every other
    // hailstone has to travel through that line. If they don't all do it, then
    // it's the wrong line and I step B forward another nanosecond. Once I find
    // a line that should work, I also verify it actually does.

    let mut no_collision = None;
    'outer: for (ai, a) in hailstones.iter().enumerate() {
        for (bi, b) in hailstones.iter().enumerate().skip(ai + 1) {
            let (x1, y1, z1, dx1, dy1, dz1) = *a;
            let (x2, y2, z2, dx2, dy2, dz2) = *b;

            let m1 = dy1 as f64 / dx1 as f64;
            let m2 = dy2 as f64 / dx2 as f64;

            let c1 = y1 as f64 - m1 * x1 as f64;
            let c2 = y2 as f64 - m2 * x2 as f64;

            let x_intersect = (c2 - c1) / (m1 - m2);

            let first_collides_in_future =
                (x1 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx1);
            let second_collides_in_future =
                (x2 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx2);
            if !first_collides_in_future || !second_collides_in_future {
                no_collision = Some((a, b, [ai, bi]));
                break 'outer;
            } else {
                // Might not collide in z still
                let time_delta = (x_intersect - x1 as f64) / dx1 as f64;
                if ((z1 - z2) as f64 + (dz1 as f64 * time_delta - dz2 as f64 * time_delta)).abs()
                    > 1e-15
                {
                    no_collision = Some((a, b, [ai, bi]));
                    break 'outer;
                }
            }
        }
    }

    let (a, b, indexes) = no_collision
        .expect("there should be at least one pair of hailstones that don't collide in the future");
    let point1 = BigPoint::new_from(a.0, a.1, a.2);
    let delta1 = BigPoint::new_from(a.3, a.4, a.5);
    let point2 = BigPoint::new_from(b.0, b.1, b.2);
    let d2 = BigPoint::new_from(b.3, b.4, b.5);

    // Get a random (different) hailstone
    let third_index = (0..3).filter(|x| !indexes.contains(x)).next().unwrap();
    let c = hailstones[third_index];
    let point3 = BigPoint::new_from(c.0, c.1, c.2);
    let d3 = BigPoint::new_from(c.3, c.4, c.5);

    // Reframe velocity with respect to the first hailstone. Now I have to be
    // careful of division by zero.
    let delta2 = &d2 - &delta1;
    let delta3 = &d3 - &delta1;

    println!("{:?}", (&point1, &point2, &point3));

    // The points (point1, point2, point2 + delta2) form a plane
    let basis1 = &point2 - &point1;
    let basis2 = &point2 - &point1 + &delta2;

    // Find the normal vector of that plane
    let normal = basis1.cross_prod(&basis2);
    // let offset = slope.dot_prod(&(x1, y1, z1).into());
    // Now we have a place of the form `dot_prod(slope, (x, y, z)) = offset`

    // Assert the third hailstone trajectory and the plane are not parallel
    let dot_prod = delta3.dot_prod(&normal);
    assert_ne!(dot_prod, BigInt::from(0));

    // Find intersection
    let coef = (&point1 - &point3).dot_prod(&normal) / dot_prod;
    let inter = &point3 + &(&delta3 * coef);
    let delta_main = &inter - &point1;

    let factor = euclid(
        euclid(
            delta_main.x.magnitude().to_owned(),
            delta_main.y.magnitude().to_owned(),
        ),
        delta_main.z.magnitude().to_owned(),
    )
    .to_bigint()
    .unwrap();

    println!("real_delta: {:?}", (&delta_main));

    let delta_main = &delta_main / factor + &delta1;

    delta_main
}

fn part2(text: &str) -> Output {
    let mut hailstones = vec![];
    for line in text.lines() {
        let (x, line) = line.split_once(", ").unwrap();
        let (y, line) = line.split_once(", ").unwrap();
        let (z, line) = line.split_once(" @ ").unwrap();
        let (dx, line) = line.split_once(", ").unwrap();
        let (dy, dz) = line.split_once(", ").unwrap();
        let [x, y, z] = [x, y, z].map(|s| s.parse::<isize>().unwrap());
        let [dx, dy, dz] = [dx.trim(), dy.trim(), dz.trim()].map(|s| s.parse::<isize>().unwrap());

        // Thank you for making all deltas non-zero
        assert!(dx != 0);
        assert!(dy != 0);
        assert!(dz != 0);
        hailstones.push((x, y, z, dx, dy, dz));
    }

    let delta_main = rock_delta(&hailstones);
    println!("real_delta: {:?}", (&delta_main));

    0
    // Find the intersection of this random hailstone with the plane:
    //
    // let basis1 = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32, z2 as i32 - z1 as i32);
    // let basis1 = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32, z2 as i32 - z1 as i32);
    //
    // // And check if all hailstone paths travel through the line between A and B
    // for step in 0..1_000_001 {
    //     ;
    //     if step == 1_000_000 {
    //         panic!("It should not take this long, I think...")
    //     }
    // }
    //
    // for (ai, a) in hailstones.iter().enumerate() {
    //     for b in hailstones.iter().skip(ai + 1) {
    //         let (x1, y1, z1, dx1, dy1, dz1) = *a;
    //         let (x2, y2, z2, dx2, dy2, dz2) = *b;
    //
    //         if (dy1 as f64 / dx1 as f64) - (dy2 as f64 / dx2 as f64) < 1e-10 {
    //             parallel = Some((a, b));
    //         }
    //     }
    // }
    //
    // for (ai, a) in hailstones.iter().enumerate() {
    //     for (bi, b) in hailstones.iter().enumerate().skip(ai + 1) {
    //         number += 1;
    //         if ai == bi {
    //             continue;
    //         }
    //
    //         let (x1, y1, dx1, _dy1, m1) = *a;
    //         let (x2, y2, dx2, _dy2, m2) = *b;
    //         let c1 = y1 as f64 - m1 * x1 as f64;
    //         let c2 = y2 as f64 - m2 * x2 as f64;
    //
    //         if m1 == m2 {
    //             if c1 == c2 {
    //                 panic!(
    //                     "Paths lie on each other: {:?}, {:?}",
    //                     (x1, y1, m1, c1),
    //                     (x2, y2, m2, c2)
    //                 );
    //             } else {
    //                 // println!("{:?}", (a, b, "parallel"));
    //                 continue;
    //             }
    //         }
    //         // y1 = m1*x1 + c1
    //         // y2 = m2*x2 + c2
    //         // c1 = y1 - m1*x1, etc.
    //         // When (x1, y1) = (x2, y2)
    //         // m1*x + c1 = m2*x + c2
    //         // x = (c2 - c1) / (m1 - m2)
    //         // y = ...
    //
    //         const BOUNDS: (f64, f64) = (2e14, 4e14);
    //
    //         let x_intersect = (c2 - c1) / (m1 - m2);
    //         let y_intersect = m1 * x_intersect + c1;
    //
    //         let (_message, diff) = match (
    //             (x1 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx1),
    //             (x2 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx2),
    //         ) {
    //             (false, false) => ("both in past", 0),
    //             (false, true) => ("first in past", 0),
    //             (true, false) => ("second in past", 0),
    //             (true, true)
    //                 if x_intersect < BOUNDS.0
    //                     || x_intersect > BOUNDS.1
    //                     || y_intersect < BOUNDS.0
    //                     || y_intersect > BOUNDS.1 =>
    //             {
    //                 ("outside the test area", 0)
    //             }
    //             (true, true) => ("inside the test area", 1),
    //         };
    //         inside += diff
    //         // println!("{:?}", (x1, y1, x2, y2, x_intersect, y_intersect, _message));
    //     }
    // }
}

fn _debug_simulate_step(hailstones: &[(BigPoint, BigPoint)]) -> Vec<(BigPoint, BigPoint)> {
    hailstones
        .iter()
        .map(|(hail, delta)| (hail + delta, (*delta).clone()))
        .collect()
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day24.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use itertools::Itertools;

    use crate::{_debug_simulate_step, part2, BigPoint};

    #[test]
    fn test_given_example() {
        let text = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3"};
        part2(text);
        assert!(false);
    }

    #[test]
    fn test_one_overtakes_two_others() {
        let result = vec![
            (
                BigPoint::new_from(111, 211, 311),
                BigPoint::new_from(-1, -1, -1),
            ),
            (
                BigPoint::new_from(118, 208, 312),
                BigPoint::new_from(1, 1, -1),
            ),
            (
                BigPoint::new_from(133, 207, 304),
                BigPoint::new_from(-1, 1, 2),
            ),
            // The expected rock
            (
                BigPoint::new_from(100, 210, 310),
                BigPoint::new_from(10, 0, 0),
            ),
        ];
        let text = result
            .iter()
            .take(3)
            .map(|(point, delta)| format!("{} @ {}", point.to_string(), delta.to_string()))
            .join("\n");
        part2(&text);

        let result = _debug_simulate_step(&result);
        assert_eq!(result[0].0, result[3].0);
        let result = _debug_simulate_step(&result);
        assert_eq!(result[1].0, result[3].0);
        let result = _debug_simulate_step(&result);
        assert_eq!(result[2].0, result[3].0);

        assert!(false);
    }

    #[test]
    fn test_simulate_step() {
        let result =
            _debug_simulate_step(&[(BigPoint::new_from(0, 0, 0), BigPoint::new_from(1, 2, -3))]);
        assert_eq!(
            &result,
            &[(BigPoint::new_from(1, 2, -3), BigPoint::new_from(1, 2, -3)),]
        );

        let result = _debug_simulate_step(&result);
        assert_eq!(
            &result,
            &[(BigPoint::new_from(2, 4, -6), BigPoint::new_from(1, 2, -3)),]
        );
    }
}
