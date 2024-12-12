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
    /// Following the rotation of polar coordinates
    pub const EIGHT_WAYS: [&[Direc]; 8] = [
        &[Direc::East],
        &[Direc::East, Direc::North],
        &[Direc::North],
        &[Direc::West, Direc::North],
        &[Direc::West],
        &[Direc::West, Direc::South],
        &[Direc::South],
        &[Direc::East, Direc::South],
    ];

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
    pub fn cmp_points(&self, a: &UsizePoint, b: &UsizePoint) -> std::cmp::Ordering {
        match self {
            Direc::South => a.0.cmp(&b.0),
            Direc::North => b.0.cmp(&a.0),
            Direc::East => a.1.cmp(&b.1),
            Direc::West => b.1.cmp(&a.1),
        }
    }

    #[inline]
    pub fn to_power_of_i(&self) -> usize {
        match self {
            Direc::East => 0,
            Direc::North => 1,
            Direc::West => 2,
            Direc::South => 3,
        }
    }
}

/// A trait to allow any tree-like structure to become mutable. Basically, you
/// will convert the source type into one that implements Zipper, and then use
/// the parent and child methods to traverse up and down the tree. The important
/// distinction is that these are fully owned values, which is only possible
/// (without Rc and the like) because getting a child element pops it from the
/// tree while remembering the steps needed to re-add it in the correct spot.
///
/// A simple example would be a binary tree as the source type while the zipper
/// type keeps track of the current root of the tree alongside a stack of
/// parents "left or right" enums/bools showing where the current node would be
/// reinserted when traversing up the tree.
pub trait Zipper: Sized {
    type Source;
    type Index;

    /// Convert the source type into a zipper view
    fn new(root: Self::Source) -> Self;

    /// Get a reference to the current root of the source tree
    fn source(&self) -> &Self::Source;

    /// Get the child at index as a zipper, or return the current zipper if index
    /// does not exist
    fn child(self, index: Self::Index) -> Result<Self, Self>;

    /// Get the parent as a zipper, or return the current zipper if `self` is
    /// the root
    fn parent(self) -> Result<Self, Self>;

    /// Extract the current source type consuming `self`. Use `.unzip()` if you
    /// need to get the source type of the root.
    fn unwrap_source(self) -> Self::Source;

    /// Convert a zipper view into the source type
    fn unzip(mut self) -> Self::Source {
        loop {
            match self.parent() {
                Ok(parent) => self = parent,
                Err(root) => return root.unwrap_source(),
            }
        }
    }
}
