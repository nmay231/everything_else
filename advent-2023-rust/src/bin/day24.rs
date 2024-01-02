use itertools::Itertools;
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use std::ops::Neg;

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
    fn _to_string(&self) -> String {
        format!("{}, {}, {}", self.x, self.y, self.z)
    }

    fn rotate_axis(&self) -> Self {
        Self::new_from(self.y.to_owned(), self.z.to_owned(), self.x.to_owned())
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

fn rock_delta(hailstones: &[(BigPoint, BigPoint)]) -> BigPoint {
    let (point1, delta1) = hailstones[0].to_owned();
    let (point2, d2) = hailstones[1].to_owned();
    let (point3, d3) = hailstones[2].to_owned();

    // Reframe velocity with respect to the first hailstone.
    let delta2 = &d2 - &delta1;
    let delta3 = &d3 - &delta1;

    let basis1 = &point2 - &point1;
    let basis2 = delta2.to_owned();

    let normal = basis1.cross_prod(&basis2);
    let (num, denom) = (
        normal.dot_prod(&(&point1 - &point3)),
        normal.dot_prod(&delta3),
    );
    assert_ne!(denom, BigInt::zero());
    assert_eq!(&num % &denom, BigInt::zero());

    let delta_main = (&(&delta3 * num) / denom);
    println!("1: {:?}", (&delta_main, &delta1));

    let gcd = delta_main.x.gcd(&delta_main.y).gcd(&delta_main.z);
    let delta_main = &delta_main / gcd + &delta1;
    println!("2: {:?}", (delta_main));
    let gcd = delta_main.x.gcd(&delta_main.y).gcd(&delta_main.z);
    println!("3: {:?}", (&delta_main / gcd.to_owned()));
    return &delta_main / gcd;

    // let mut no_collision = None;

    // 'outer: for (i1, (point1, delta1)) in hailstones.iter().enumerate() {
    //     for (i2, (point2, delta2)) in hailstones.iter().enumerate().skip(i1 + 1) {
    //         // t2 = ((y2 - y1) * dx1 - (x2 - x1) * dy1) / (dx2*dy1 - dy2*dx1)
    //         let (num, denom) = (
    //             ((&p2.y - &p1.y) * &d1.x - (&p2.x - &p1.x) * &d1.y),
    //             (&d2.x * &d1.y - &d2.y * &d1.x),
    //         );
    //         assert_eq!(&num % &denom, BigInt::zero());
    //         let t2 = &num / &denom;
    //         {
    //             // // let (x1, y1, z1, dx1, dy1, dz1) = *a;
    //             // // let (x2, y2, z2, dx2, dy2, dz2) = *b;
    //
    //             // // let m1 = &delta1.y / &delta1.x;
    //             // // let m2 = &delta2.y / &delta2.x;
    //             // let gcd = delta1.x.gcd(&delta1.y).gcd(&delta1.z);
    //             // let delta1 = delta1 / gcd;
    //
    //             // if m1 == m2 {
    //             //     no_collision = Some((
    //             //         (point1.to_owned(), delta1.to_owned()),
    //             //         (point2.to_owned(), delta2.to_owned()),
    //             //         [i1, i2],
    //             //     ));
    //             //     break 'outer;
    //             //     // panic!(
    //             //     //     "Don't know yet what to do with equal slopes {:?}",
    //             //     //     ((&point1, &delta1), (&point2, &delta2),)
    //             //     // );
    //             // }
    //
    //             // let c1 = &point1.y - &m1 * &point1.x;
    //             // let c2 = &point2.y - &m2 * &point2.x;
    //
    //             // let x_intersect = (&c2 - &c1) / (&m1 - &m2);
    //
    //             // let first_collides_in_future =
    //             //     (point1.x).partial_cmp(&x_intersect) == BigInt::zero().partial_cmp(&delta1.x);
    //             // let second_collides_in_future =
    //             //     (point2.x).partial_cmp(&x_intersect) == BigInt::zero().partial_cmp(&delta2.x);
    //             // if !first_collides_in_future || !second_collides_in_future {
    //             //     no_collision = Some((
    //             //         (point1.to_owned(), delta1.to_owned()),
    //             //         (point2.to_owned(), delta2.to_owned()),
    //             //         [i1, i2],
    //             //     ));
    //             //     break 'outer;
    //             // } else {
    //             //     // Might not collide in z still
    //             //     let time_delta = (&x_intersect - &point1.x) / &delta1.x;
    //             //     if ((&point1.z - &point2.z) + (&delta1.z * &time_delta - &delta2.z * &time_delta))
    //             //         .magnitude()
    //             //         > &0u64.into()
    //             //     {
    //             //         no_collision = Some((
    //             //             (point1.to_owned(), delta1.to_owned()),
    //             //             (point2.to_owned(), delta2.to_owned()),
    //             //             [i1, i2],
    //             //         ));
    //             //         break 'outer;
    //             //     }
    //             // }
    //         }
    //     }
    // }
    // let ((point1, delta1), (point2, d2), indexes) = no_collision
    //     .expect("there should be at least one pair of hailstones that don't collide in the future");
    // let point1 = BigPoint::new_from(a.0, a.1, a.2);
    // let delta1 = BigPoint::new_from(a.3, a.4, a.5);
    // let point2 = BigPoint::new_from(b.0, b.1, b.2);
    // let d2 = BigPoint::new_from(b.3, b.4, b.5);

    // Get a random (different) hailstone
    // let third_index = (0..3).filter(|x| !indexes.contains(x)).next().unwrap();
    let (a, b, c) = hailstones.iter().take(3).collect_tuple().unwrap();
    let (point1, delta1) = a.to_owned();
    let (point2, d2) = b.to_owned();
    let (point3, d3) = c.to_owned();

    // Reframe velocity with respect to the first hailstone.
    let delta2 = &d2 - &delta1;
    let delta3 = &d3 - &delta1;

    // println!("{:?}", (&point1, &point2, &point3));

    // The points (point1, point2, point2 + delta2) form a plane
    let basis1 = &point2 - &point1;
    let basis2 = &point2 - &point1 + &delta2;

    // Find the normal vector of that plane
    let normal = basis1.cross_prod(&basis2);

    // Assert the third hailstone trajectory and the plane are not parallel
    let dot_prod = delta3.dot_prod(&normal);
    assert_ne!(dot_prod, BigInt::zero());

    // Find intersection
    let coef = (&point1 - &point3).dot_prod(&normal) / dot_prod;
    let inter = &point3 + &(&delta3 * coef);
    let delta_main = &inter - &point1;

    let gcd = delta_main.x.gcd(&delta_main.y).gcd(&delta_main.z);

    // println!("original delta: {:?}", (&delta_main));

    let delta_main = &delta_main / gcd + &delta1;

    delta_main
}

fn parse_hailstones(text: &str) -> Vec<(BigPoint, BigPoint)> {
    let mut hailstones = vec![];
    for line in text.lines() {
        let (x, line) = line.split_once(", ").unwrap();
        let (y, line) = line.split_once(", ").unwrap();
        let (z, line) = line.split_once(" @ ").unwrap();
        let (dx, line) = line.split_once(", ").unwrap();
        let (dy, dz) = line.split_once(", ").unwrap();
        let [x, y, z] = [x, y, z].map(|s| s.parse::<isize>().unwrap());
        let [dx, dy, dz] = [dx.trim(), dy.trim(), dz.trim()].map(|s| s.parse::<isize>().unwrap());

        // // Thank you for making all deltas non-zero
        // assert!(dx != 0);
        // assert!(dy != 0);
        // assert!(dz != 0);
        hailstones.push((BigPoint::new_from(x, y, z), BigPoint::new_from(dx, dy, dz)));
    }
    hailstones
}

fn part2(text: &str) -> (BigPoint, BigPoint) {
    let hailstones = parse_hailstones(text);

    // v1 + t1*dv1 = v2 + t2*dv2
    // x1 - x2 + t1*dx1 - t2*dx2 = 0, ditto. for y, z
    // t1 = (t2*dx2 + x2 - x1) / dx1
    // t1 = (t2*dy2 + y2 - y1) / dy1
    // (t2*dx2 + x2 - x1) / dx1 = (t2*dy2 + y2 - y1) / dy1
    // (t2*dx2 + x2 - x1) * dy1 = (t2*dy2 + y2 - y1) * dx1
    // t2*dx2*dy1 - t2*dy2*dx1 = (y2 - y1) * dx1 - (x2 - x1) * dy1
    // t2 = ((y2 - y1) * dx1 - (x2 - x1) * dy1) / (dx2*dy1 - dy2*dx1)

    let delta_main = rock_delta(&hailstones);
    println!("delta_main: {:?}", (&delta_main));
    let (ref_point, ref_delta) = &hailstones[0];

    let p1 = ref_point;
    let d1 = &delta_main - ref_delta;

    let mut neg_min_time = BigUint::one();

    for (p2, d2) in hailstones.iter().skip(1) {
        if &d2 == &ref_delta {
            todo!("Cannot have parallel hailstones");
        }
        let d2 = d2 - ref_delta;

        let (p1, p2, d1, d2) = match (d1.x.is_zero(), d1.y.is_zero(), d1.z.is_zero()) {
            (false, _, _) if &d2.x * &d1.y != &d2.y * &d1.x => {
                (p1.to_owned(), p2.to_owned(), d1.to_owned(), d2)
            }
            (_, false, _) if &d2.y * &d1.z != &d2.z * &d1.y => (
                p1.rotate_axis(),
                p2.rotate_axis(),
                d1.rotate_axis(),
                d2.rotate_axis(),
            ),
            (_, _, false) if &d2.z * &d1.x != &d2.x * &d1.z => (
                p1.rotate_axis().rotate_axis(),
                p2.rotate_axis().rotate_axis(),
                d1.rotate_axis().rotate_axis(),
                d2.rotate_axis().rotate_axis(),
            ),
            _ => panic!(
                "The main delta cannot be all zeros or the two deltas be parallel, {:?}",
                (&p1, &p2, &d1, &d2)
            ),
        };
        // t2 = ((y2 - y1) * dx1 - (x2 - x1) * dy1) / (dx2*dy1 - dy2*dx1)
        let (num, denom) = (
            ((&p2.y - &p1.y) * &d1.x - (&p2.x - &p1.x) * &d1.y),
            (&d2.x * &d1.y - &d2.y * &d1.x),
        );
        assert_eq!(&num % &denom, BigInt::zero());
        let t2 = &num / &denom;
        // t1 = (t2*dx2 + x2 - x1) / dx1
        let t1 = (&t2 * &d2.x + &p2.x - &p1.x) / &d1.x;
        println!(
            "t1: {:?}",
            (
                &t1,
                &t2,
                (p1.to_owned(), p2.to_owned(), d1.to_owned(), d2.to_owned())
            )
        );
        if t1.sign() == Sign::Minus && t1.magnitude() > &neg_min_time {
            neg_min_time = 2u64 * t1.magnitude();
        }
        {
            // x1 + t1 * dx1 = x2 + t2 * dx2
            // x =

            // let delta = delta - ref_delta;
            // if &delta == &ref_delta {
            //     panic!("Cannot have parallel hailstones ")
            // }

            // // x = ((y2 - y1)*dx1*dx2 - (dy2*x2*dx1 - dy1*x1*dx2)) / (dy1*dx2 - dy2*dx1)
            // // denom = (dy1*dx2 - dy2*dx1)
            // let denom = &delta_main.y * &delta.x - &delta.y * &delta_main.x;
            // assert_ne!(
            //     denom,
            //     BigInt::from(064),
            //     "{:?}",
            //     ((ref_point, &delta_main), (&point, &delta), &ref_delta)
            // );

            // // x = ((y2 - y1)*dx1*dx2 - (dy2*x2*dx1 - dy1*x1*dx2)) / (dy1*dx2 - dy2*dx1)
            // let x_intersect = ((&point.y - &ref_point.y) * &delta_main.x * &delta.x
            //     - (&delta.y * &point.x * &delta_main.x - &delta_main.y * &ref_point.x * &delta.x))
            //     / &denom;

            // let time_delta = (&point.x - &x_intersect)
            //     .checked_div(&delta.x)
            //     .or_else(|| (&point.x - &x_intersect).checked_div(&delta.x))
            //     .or_else(|| (&point.x - &x_intersect).checked_div(&delta.x));
            // neg_min_time = std::cmp::max(neg_min_time, time_delta.magnitude().to_owned());
            // // assert_eq!(&point.x, )
        }
    }
    {
        // for (i1, (point1, delta1)) in hailstones.iter().enumerate() {
        //     for (i2, (point2, delta2)) in hailstones.iter().enumerate().skip(i1 + 1) {
        //         // let (x1, y1, z1, dx1, dy1, dz1) = *a;
        //         // let (x2, y2, z2, dx2, dy2, dz2) = *b;

        //         if m1 == m2 {
        //             panic!("Don't know yet what to do with equal slopes");
        //         }

        //         let c1 = &point1.y - &m1 * &point1.x;
        //         let c2 = &point2.y - &m2 * &point2.x;

        //         let x_intersect = (&c2 - &c1) / (&m1 - &m2);

        //         let first_collides_in_future =
        //             (point1.x).partial_cmp(&x_intersect) == BigInt::zero().partial_cmp(&delta1.x);
        //         let second_collides_in_future =
        //             (point2.x).partial_cmp(&x_intersect) == BigInt::zero().partial_cmp(&delta2.x);
        //         if !first_collides_in_future || !second_collides_in_future {
        //             no_collision = Some((
        //                 (point1.to_owned(), delta1.to_owned()),
        //                 (point2.to_owned(), delta2.to_owned()),
        //                 [i1, i2],
        //             ));
        //             break 'outer;
        //         } else {
        //             // Might not collide in z still
        //             let time_delta = (&x_intersect - &point1.x) / &delta1.x;
        //             if ((&point1.z - &point2.z)
        //                 + (&delta1.z * &time_delta - &delta2.z * &time_delta))
        //                 .magnitude()
        //                 > &0u64.into()
        //             {
        //                 no_collision = Some((
        //                     (point1.to_owned(), delta1.to_owned()),
        //                     (point2.to_owned(), delta2.to_owned()),
        //                     [i1, i2],
        //                 ));
        //                 break 'outer;
        //             }
        //         }
        //     }
        // }
    }

    let start = ref_point - &(&d1 * (neg_min_time.to_bigint().unwrap()));
    println!("start: {:?}", (&start));

    (start, delta_main)
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
    let (start, delta) = part2(&text);
    println!(
        "part 2 result = {:?}",
        (&start, delta, &start.x + &start.y + &start.z)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use itertools::Itertools;

    use crate::{_debug_simulate_step, parse_hailstones, part2, rock_delta, BigPoint};

    #[test]
    fn test_rock_delta() {
        let text = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3"};
        let hailstones = parse_hailstones(text);

        for gap in 0..hailstones.len() {
            let hailstones = hailstones
                .iter()
                .skip(gap)
                .chain(hailstones.iter().take(gap))
                .map(|pair| pair.to_owned())
                .collect_vec();
            println!("starting hailstones order: {:?}", (&hailstones));
            println!("final main_delta: {:?}", (&rock_delta(&hailstones)));
        }
        assert!(false);
    }

    #[test]
    fn test_given_example() {
        let text = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3"};
        let tmp = part2(text);
        assert_eq!(
            tmp,
            (BigPoint::new_from(24, 13, 10), BigPoint::new_from(-3, 1, 2))
        );
        assert!(false);
    }

    #[test]
    fn test_123() {
        let mut result = vec![
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
            .map(|(point, delta)| format!("{} @ {}", point._to_string(), delta._to_string()))
            .join("\n");

        let original_rock = result[3].to_owned();

        for step in 0..3 {
            result = _debug_simulate_step(&result);
            assert_eq!(result[step].0, result[3].0);
        }

        assert_eq!(original_rock, part2(&text));
        assert!(false);
    }

    #[test]
    fn test_135() {
        let mut result = vec![
            (
                BigPoint::new_from(112, 212, 312),
                BigPoint::new_from(-1, -1, -1),
            ),
            (
                BigPoint::new_from(116, 206, 314),
                BigPoint::new_from(1, 1, -1),
            ),
            (
                BigPoint::new_from(136, 204, 322),
                BigPoint::new_from(-1, 1, -2),
            ),
            // The expected rock
            (
                BigPoint::new_from(100, 210, 310),
                BigPoint::new_from(5, 0, 0),
            ),
        ];
        let text = result
            .iter()
            .take(3)
            .map(|(point, delta)| format!("{} @ {}", point._to_string(), delta._to_string()))
            .join("\n");

        let original_rock = result[3].to_owned();

        for step in 0..3 {
            result = _debug_simulate_step(&result);
            result = _debug_simulate_step(&result);
            assert_eq!(result[step].0, result[3].0);
        }

        assert_eq!(original_rock, part2(&text));
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
