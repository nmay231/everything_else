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
}

// TODO: Implement as a generic and type aliases for usize and isize, and
// implement the Add, etc. traits
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct IsizePoint(isize, isize);

impl IsizePoint {
    #[inline(always)]
    pub fn within_grid(&self, grid_size: &UsizePoint) -> bool {
        self.0 >= 0
            && self.1 >= 0
            && (self.0 as usize) < grid_size.0
            && (self.1 as usize) < grid_size.1
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

#[derive(Debug, Clone)]
pub struct DisjointSetWithCount {
    /// Maps a node to its parent, or to itself if it is an eve, aka the set's representative
    indexes: Vec<usize>,
    /// Number of children (plus one for the node itself)
    counts: Vec<usize>,
}

impl DisjointSetWithCount {
    pub fn new(size: usize) -> Self {
        Self {
            indexes: (0..size).collect(),
            counts: vec![1; size],
        }
    }

    /// Number of nodes
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.indexes.len()
    }

    /// Number of children of the node's eve (plus one for the eve itself)
    pub fn size_of_eve(&mut self, node: usize) -> usize {
        if node == self.indexes[node] {
            return self.counts[node];
        } else {
            let eve = self.bookkeeping_eve(node);
            return self.size_of_eve(eve);
        }
    }

    /// Merge a and b into the same set, if not already in the same
    pub fn link(&mut self, a: usize, b: usize) {
        assert!(a < self.indexes.len() && b < self.indexes.len());

        let a = self.bookkeeping_eve(a);
        let b = self.bookkeeping_eve(b);

        if a != b {
            self.indexes[a] = b;
            self.counts[b] += self.counts[a];
        }
    }

    // TODO: Don't think I ever need this...
    // pub fn eve(&self, node: usize) -> usize {}

    /// Find the eve/root of the node while updating the parent of any node in between
    pub fn bookkeeping_eve(&mut self, mut node: usize) -> usize {
        loop {
            let parent = self.indexes[node];
            if node == parent {
                return node;
            }
            // amortize the cost of looking up the eve of a node.
            self.indexes[node] = self.indexes[parent];

            node = parent;
        }
    }

    /// Does a node map back to itself, i.e. it has no parent?
    pub fn is_an_eve(&self, node: usize) -> bool {
        return self.indexes[node] == node;
    }

    pub fn debug_print(&self) {
        use std::collections::hash_map::HashMap;
        let mut map = HashMap::<usize, Vec<usize>>::new();
        let mut eves = vec![];
        for node in 0..self.size() {
            if self.is_an_eve(node) {
                eves.push(node);
            }
            map.entry(self.indexes[node]).or_default().push(node);
        }

        fn debug_string(
            node: usize,
            prefix: &str,
            map: &HashMap<usize, Vec<usize>>,
            counts: &Vec<usize>,
        ) -> String {
            match map.get(&node) {
                Some(children) => {
                    let mut result = String::new();
                    let prefix = format!("{}{}(count={}) <- ", prefix, node, counts[node]);
                    for child in children.iter() {
                        if child == &node {
                            continue;
                        }
                        result.push_str(&debug_string(*child, &prefix, map, counts));
                    }
                    result
                }
                None => format!("{}{}\n", prefix, node),
            }
        }

        for eve in eves {
            print!("{}", debug_string(eve, "", &map, &self.counts));
        }
        println!();
    }
}

#[cfg(test)]
mod test_disjoint_set_with_count {
    use itertools::Itertools;

    use crate::DisjointSetWithCount;

    #[derive(PartialEq, Eq, Debug)]
    struct Summary {
        pub total_count: usize,
        pub eve_count: usize,
    }
    fn get_summary(dis_set: &mut DisjointSetWithCount) -> Summary {
        let mut total_count = 0;
        let mut eve_count = 0;
        for node in 0..dis_set.size() {
            if dis_set.is_an_eve(node) {
                total_count += dis_set.size_of_eve(node);
                eve_count += 1;
            }
        }
        Summary {
            total_count,
            eve_count,
        }
    }

    macro_rules! assert_consistency {
        ($grid:expr, $eve_count:expr) => {
            assert_eq!(
                get_summary($grid),
                Summary {
                    total_count: $grid.size(),
                    eve_count: $eve_count
                }
            )
        };
    }

    #[test]
    fn first_rows_then_columns() {
        let mut grid = DisjointSetWithCount::new(25);
        assert_consistency!(&mut grid, 25);

        for window in (0..25).chunks(5).into_iter() {
            for (a, b) in window.tuple_windows() {
                grid.link(a, b);
            }
        }

        for start in 0..5 {
            let mut grid = grid.to_owned();
            assert_consistency!(&mut grid, 5);

            for (a, b) in (0..25).skip(start).step_by(5).tuple_windows() {
                grid.link(a, b);
            }
            assert_consistency!(&mut grid, 1);
        }
    }
}
