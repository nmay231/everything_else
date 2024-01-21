use indoc::indoc;
use num_bigint::BigInt;
use std::ops::Neg;

fn _part1(text: &str) -> usize {
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

    // Not impl Display, since that might be better as a "(x, y, z)"
    fn _to_string(&self) -> String {
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

fn part2(text: &str) -> () {
    let hailstones = parse_hailstones(text);

    println!(indoc! {"
    (declare-const rx Int)
    (declare-const ry Int)
    (declare-const rz Int)
    (declare-const rdx Int)
    (declare-const rdy Int)
    (declare-const rdz Int)
    "});
    for (i, (point, delta)) in hailstones.into_iter().enumerate() {
        // let ti = format!("t{}", i);
        let Point { x, y, z } = point;
        let Point {
            x: dx,
            y: dy,
            z: dz,
        } = delta;
        println!("(declare-const t{i} Int)");
        println!("(assert(= (+ {x} (* t{i} {dx})) (+ rx (* t{i} rdx))))");
        println!("(assert(= (+ {y} (* t{i} {dy})) (+ ry (* t{i} rdy))))");
        println!("(assert(= (+ {z} (* t{i} {dz})) (+ rz (* t{i} rdz))))");
    }
    println!("(check-sat)");
    println!("(get-model)");
}

fn _debug_simulate_step(hailstones: &[(BigPoint, BigPoint)]) -> Vec<(BigPoint, BigPoint)> {
    hailstones
        .iter()
        .map(|(hail, delta)| (hail + delta, (*delta).clone()))
        .collect()
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day24.txt")?;

    // cargo run --bin day24 | ~/.build_dirs/z3-z3-4.12.4/build/z3 -in | grep -A1 " r"
    // Then calculate rx + ry + rz
    // This was deeply unsatisfying, tbh...
    part2(&text);

    Ok(())
}
