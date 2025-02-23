use crate::Direc;

/// (row, column)
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
#[deprecated(
    note = "I kept thinking this was (x, y) instead of (rows, cols). Use Point<usize> instead."
)]
pub struct UsizePoint(pub usize, pub usize);

#[allow(deprecated)]
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

    #[inline(always)]
    pub fn within_grid(&self, grid_size: &UsizePoint) -> bool {
        self.0 < grid_size.0 && self.1 < grid_size.1
    }

    #[inline(always)]
    pub fn isize(&self) -> IsizePoint {
        IsizePoint(self.0 as isize, self.1 as isize)
    }

    #[inline(always)]
    pub fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    #[inline(always)]
    pub fn sub(&self, other: &Self) -> IsizePoint {
        self.isize().sub(&other.isize())
    }

    #[inline(always)]
    pub fn neg(&self) -> IsizePoint {
        self.isize().neg()
    }

    #[inline(always)]
    pub fn mul(&self, scalar: usize) -> Self {
        Self(self.0 * scalar, self.1 * scalar)
    }

    #[inline(always)]
    pub fn piecewise_mul(&self, other: &UsizePoint) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }

    #[inline(always)]
    pub fn area(&self) -> usize {
        self.0 * self.1
    }
}

// TODO: Implement as a generic and type aliases for usize and isize, and
// implement the Add, etc. traits
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
#[deprecated(
    note = "I kept thinking this was (x, y) instead of (rows, cols). Use Point<isize> instead."
)]
pub struct IsizePoint(pub isize, pub isize);

#[allow(deprecated)]
impl IsizePoint {
    #[inline(always)]
    pub fn within_grid(&self, grid_size: &UsizePoint) -> bool {
        self.0 >= 0
            && self.1 >= 0
            && (self.0 as usize) < grid_size.0
            && (self.1 as usize) < grid_size.1
    }

    #[inline(always)]
    pub fn usize(&self) -> UsizePoint {
        UsizePoint(self.0 as usize, self.1 as usize)
    }

    pub fn try_usize(&self) -> Option<UsizePoint> {
        Some(UsizePoint(self.0.try_into().ok()?, self.1.try_into().ok()?))
    }

    #[inline(always)]
    pub fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    #[inline(always)]
    pub fn sub(&self, other: &Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }

    #[inline(always)]
    pub fn neg(&self) -> Self {
        Self(-self.0, -self.1)
    }
}
