use std::ops::{Add, Div, Mul, Sub};

use num_traits::{Num, NumRef};

use crate::Direc;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub trait MyNumber: NumRef + Num + Ord + Clone {}
impl<T: NumRef + Num + Ord + Clone> MyNumber for T {}

// TODO: Support non-Copy types by using operations on &T
impl<T: MyNumber> Point<T> {
    #[inline(always)]
    pub const fn new_xy(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn within_grid(&self, grid_size: &Point<T>) -> bool {
        self.x >= T::zero()
            && self.y >= T::zero()
            && (self.x) < grid_size.x
            && (self.y) < grid_size.y
    }

    #[inline(always)]
    pub fn next_point(&self, direc: &Direc, grid_size: &Self) -> Option<Self> {
        self.next_point_steps(&T::one(), direc, grid_size)
    }

    pub fn next_point_steps(&self, steps: &T, direc: &Direc, grid_size: &Self) -> Option<Self> {
        assert!(steps >= &T::zero());

        match direc {
            Direc::North => {
                if &self.y >= steps {
                    Some(Self::new_xy(self.x.clone(), self.y.clone() - steps))
                } else {
                    None
                }
            }
            Direc::East => {
                if self.x.clone() + steps < grid_size.x {
                    Some(Self::new_xy(self.x.clone() + steps, self.y.clone()))
                } else {
                    None
                }
            }
            Direc::South => {
                if self.y.clone() + steps < grid_size.y {
                    Some(Self::new_xy(self.x.clone(), self.y.clone() + steps))
                } else {
                    None
                }
            }
            Direc::West => {
                if &self.x >= steps {
                    Some(Self::new_xy(self.x.clone() - steps, self.y.clone()))
                } else {
                    None
                }
            }
        }
    }

    // TODO: Don't think I really need `next_point_steps_wrap()`
    pub fn next_point_wrap(&self, direc: &Direc, grid_size: &Self) -> Self {
        match direc {
            Direc::North => {
                if self.y > T::zero() {
                    Self::new_xy(self.x.clone(), self.y.clone() - T::one())
                } else {
                    Self::new_xy(self.x.clone(), grid_size.y.clone() - T::one())
                }
            }
            Direc::East => {
                if self.x.clone() + T::one() < grid_size.x {
                    Self::new_xy(self.x.clone() + T::one(), self.y.clone())
                } else {
                    Self::new_xy(T::zero(), self.y.clone())
                }
            }
            Direc::South => {
                if self.y.clone() + T::one() < grid_size.y {
                    Self::new_xy(self.x.clone(), self.y.clone() + T::one())
                } else {
                    Self::new_xy(self.x.clone(), T::zero())
                }
            }
            Direc::West => {
                if self.x > T::zero() {
                    Self::new_xy(self.x.clone() - T::one(), self.y.clone())
                } else {
                    Self::new_xy(grid_size.x.clone() - T::one(), self.y.clone())
                }
            }
        }
    }

    #[inline(always)]
    pub fn is_on_edge(&self, grid_size: &Self) -> bool {
        self.x == T::zero()
            || self.y == T::zero()
            || self.x.clone() + T::one() == grid_size.x
            || self.y.clone() + T::one() == grid_size.y
    }

    #[inline(always)]
    pub fn as_index(&self, grid_size: &Self) -> T {
        grid_size.x.clone() * &self.y + &self.x
    }

    #[inline(always)]
    pub fn from_index(grid_size: &Self, index: T) -> Self {
        Self::new_xy(index.clone() % &grid_size.x, index / &grid_size.x)
    }

    // pub fn debug_grid<Cell: Display>(&self, grid: &[Cell]) {
    //     assert_eq!(self.area(), grid.len());
    //     for row_i in T::zero()..self.y {
    //         println!(
    //             "{}",
    //             String::from_iter(&grid[row_i * self.y..(row_i + 1) * self.x])
    //         );
    //     }
    // }

    #[inline(always)]
    pub fn manhattan_distance(&self, other: &Self) -> T {
        let mut xs = [&self.x, &other.x];
        let mut ys = [&self.y, &other.y];
        xs.sort();
        ys.sort();
        xs[1].clone() - xs[0] + &(ys[1].clone() - ys[0])
    }

    /// self.x * self.y
    #[inline(always)]
    pub fn area(&self) -> T {
        self.x.clone() * &self.y
    }

    #[inline(always)]
    pub fn dot_product(&self, other: &Self) -> T {
        self.x.clone() * &other.x + &(self.y.clone() * &other.y)
    }

    #[inline(always)]
    pub fn map<U: MyNumber>(&self, mapper: impl Fn(&T) -> U) -> Point<U> {
        Point {
            x: mapper(&self.x),
            y: mapper(&self.y),
        }
    }
}

impl<T: MyNumber + TryInto<usize>> Point<T> {
    pub fn try_map_usize(&self) -> Option<Point<usize>> {
        Some(Point::new_xy(
            self.x.clone().try_into().ok()?,
            self.y.clone().try_into().ok()?,
        ))
    }
}

impl<T: MyNumber + TryInto<isize>> Point<T> {
    pub fn try_map_isize(&self) -> Option<Point<isize>> {
        Some(Point::new_xy(
            self.x.clone().try_into().ok()?,
            self.y.clone().try_into().ok()?,
        ))
    }
}

macro_rules! operator {
    ($trait:ty, $operation:ident) => {
        impl<T: MyNumber> $trait for Point<T> {
            type Output = Self;

            fn $operation(self, rhs: Self) -> Self::Output {
                Self::new_xy(self.x.$operation(&rhs.x), self.y.$operation(&rhs.y))
            }
        }
    };
}

operator!(Add, add);
operator!(Sub, sub);
operator!(Mul, mul);
operator!(Div, div);
