use std::cmp::Ordering;
use std::fmt::Debug;

use itertools::Itertools;
use num_integer::{ExtendedGcd, Integer};
use num_traits::Euclid;

/// (row, column)
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct UsizePoint(pub usize, pub usize);

impl UsizePoint {
    #[inline(always)]
    pub fn next_point(&self, direc: &Direc, grid_size: &UsizePoint) -> Option<UsizePoint> {
        self.next_point_steps(1, direc, grid_size)
    }

    pub fn next_point_steps(
        &self,
        steps: usize,
        direc: &Direc,
        grid_size: &UsizePoint,
    ) -> Option<UsizePoint> {
        match direc {
            Direc::North => {
                if self.0 >= steps {
                    Some(UsizePoint(self.0 - steps, self.1))
                } else {
                    None
                }
            }
            Direc::East => {
                if self.1 + steps < grid_size.1 {
                    Some(UsizePoint(self.0, self.1 + steps))
                } else {
                    None
                }
            }
            Direc::South => {
                if self.0 + steps < grid_size.0 {
                    Some(UsizePoint(self.0 + steps, self.1))
                } else {
                    None
                }
            }
            Direc::West => {
                if self.1 >= steps {
                    Some(UsizePoint(self.0, self.1 - steps))
                } else {
                    None
                }
            }
        }
    }

    // TODO: Don't think I really need `next_point_steps_wrap()`
    pub fn next_point_wrap(&self, direc: &Direc, grid_size: &UsizePoint) -> UsizePoint {
        match direc {
            Direc::North => {
                if self.0 > 0 {
                    UsizePoint(self.0 - 1, self.1)
                } else {
                    UsizePoint(grid_size.0 - 1, self.1)
                }
            }
            Direc::East => {
                if self.1 + 1 < grid_size.1 {
                    UsizePoint(self.0, self.1 + 1)
                } else {
                    UsizePoint(self.0, 0)
                }
            }
            Direc::South => {
                if self.0 + 1 < grid_size.0 {
                    UsizePoint(self.0 + 1, self.1)
                } else {
                    UsizePoint(0, self.1)
                }
            }
            Direc::West => {
                if self.1 > 0 {
                    UsizePoint(self.0, self.1 - 1)
                } else {
                    UsizePoint(self.0, grid_size.1 - 1)
                }
            }
        }
    }

    #[inline(always)]
    pub fn is_on_edge(&self, grid_size: &UsizePoint) -> bool {
        self.0 == 0 || self.1 == 0 || self.0 + 1 == grid_size.0 || self.1 + 1 == grid_size.1
    }

    #[inline(always)]
    pub fn as_index(&self, grid_size: &UsizePoint) -> usize {
        grid_size.1 * self.0 + self.1
    }

    #[inline(always)]
    pub fn from_index(grid_size: &UsizePoint, index: usize) -> Self {
        Self(index / grid_size.1, index % grid_size.1)
    }

    #[inline(always)]
    pub fn debug_grid(&self, grid: &[char]) {
        assert_eq!(self.0 * self.1, grid.len());
        for row_i in 0..self.0 {
            println!(
                "{}",
                String::from_iter(&grid[row_i * self.1..(row_i + 1) * self.1])
            );
        }
    }

    #[inline(always)]
    pub fn manhattan_distance(&self, other: &UsizePoint) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direc {
    North,
    East,
    South,
    West,
}

impl Direc {
    pub const POWERS_OF_I: [Direc; 4] = [Direc::East, Direc::North, Direc::West, Direc::South];

    pub fn rotate(&self, rotation_counter_clockwise: i32) -> Self {
        let current_index = Direc::POWERS_OF_I
            .iter()
            .enumerate()
            .find_map(|(i, x)| if x == self { Some(i as i32) } else { None })
            .unwrap();
        return Direc::POWERS_OF_I
            [(rotation_counter_clockwise + current_index).rem_euclid(4) as usize];
    }

    #[inline]
    pub fn cmp_points(&self, a: &UsizePoint, b: &UsizePoint) -> Ordering {
        match self {
            Direc::South => a.0.cmp(&b.0),
            Direc::North => b.0.cmp(&a.0),
            Direc::East => a.1.cmp(&b.1),
            Direc::West => b.1.cmp(&a.1),
        }
    }
}

pub fn chinese_remainder<T: Integer + Clone + Debug + Euclid>(
    remainder_modulus: Vec<(T, T)>,
    cast: impl Fn(isize) -> T,
) -> T {
    // println!("{:?}", (remainder_modulus));
    let zero = cast(0);
    let one = cast(1);
    for (a, b) in remainder_modulus.iter().tuple_combinations() {
        // moduli must be coprime
        let gcd = &a.1.gcd(&b.1);
        assert_eq!(
            gcd,
            &one,
            "gcd({:?}, {:?})={gcd:?} ({gcd:?} * {:?}, {gcd:?} * {:?})",
            &a.1,
            &b.1,
            a.1.to_owned() / gcd.to_owned(),
            b.1.to_owned() / gcd.to_owned(),
        );

        // I think the remainder has to be in its smallest form
        assert!(&zero < &a.0 && &a.0 < &a.1);
        assert!(&zero < &b.0 && &b.0 < &b.1);
    }

    let mut x = zero.clone();
    let cap_n = remainder_modulus
        .iter()
        .fold(one.to_owned(), |acc, (_, n_i)| acc * n_i.to_owned());
    for (a_i, n_i) in remainder_modulus {
        let y_i = cap_n.to_owned() / n_i.to_owned();
        let ExtendedGcd { x: z_i, .. } = y_i.extended_gcd(&n_i);
        // let z_i = z_i.rem_euclid(n_i);
        // + Mul<&T, Output = T>
        // let test = z_i * y_i;
        // test.e
        assert_eq!(&(z_i.to_owned() * y_i.to_owned()).rem_euclid(&n_i), &one);

        x = x + a_i * y_i * z_i;
        // x = x + a_i.to_owned() * y_i.to_owned() * z_i.to_owned();
        // println!("{:?}", (a_i, y_i, z_i, a_i * y_i * z_i));
    }
    // 0.rem_
    let x = x.rem_euclid(&cap_n);
    assert!(x >= zero);
    x
}

#[cfg(test)]
mod test_chinese_remainder {
    use crate::chinese_remainder;

    #[test]
    fn example1() {
        let actual = chinese_remainder(vec![(1, 3), (4, 5), (6, 7)], |x| x);
        assert_eq!(actual, 34);
    }

    #[test]
    fn example2() {
        let actual = chinese_remainder(vec![(2, 3), (3, 8)], |x| x);
        assert_eq!(actual, 11);
    }

    #[test]
    fn single_entry() {
        let actual = chinese_remainder(vec![(2, 3)], |x| x);
        assert_eq!(actual, 2);
    }
}

pub trait Zipper: Sized {
    type Target;
    type Index;

    /// Get the child at index as a zipper, or return the current zipper if index
    /// does not exist
    fn child(self, index: Self::Index) -> Result<Self, Self>;

    /// Get the parent as a zipper, or return the current zipper if `self` is
    /// the root
    fn parent(self) -> Result<Self, Self>;

    /// Convert the target type into a zipper view
    fn new(target: Self::Target) -> Self;

    /// Extract the current target type consuming `self`. Use `.unzip()` if you
    /// need to get the target type of the root.
    fn unwrap_target(self) -> Self::Target;

    /// Convert a zipper view into the target type
    fn unzip(mut self) -> Self::Target {
        loop {
            match self.parent() {
                Ok(parent) => self = parent,
                Err(root) => return root.unwrap_target(),
            }
        }
    }
}
