#![feature(assert_matches)]
use std::fmt::Debug;
use std::ops::Deref;

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
pub enum EveOrNode<T> {
    Eve(T),
    Node(usize),
}

#[derive(Debug, Clone)]
pub struct DisjointSet<EveT: Eve> {
    /// Maps a node to its parent, or to itself if it is an eve, aka the set's representative
    nodes: Vec<EveOrNode<EveT>>,
}

pub trait Eve {
    fn init(index: usize) -> Self;
    fn merge(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone)]
pub struct Count(usize);

impl Deref for Count {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

impl Eve for Count {
    fn init(_index: usize) -> Self {
        Self(1)
    }

    fn merge(&self, other: &Self) -> Self {
        Self(self.0 + other.0)
    }
}

#[derive(Debug, Clone)]
pub struct EveAsIndex(usize);

impl Deref for EveAsIndex {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

impl Eve for EveAsIndex {
    fn init(index: usize) -> Self {
        Self(index)
    }

    fn merge(&self, _other: &Self) -> Self {
        Self(self.0)
    }
}

impl<EveT: Eve> DisjointSet<EveT> {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: (0..size)
                .map(|index| EveOrNode::Eve(EveT::init(index)))
                .collect(),
        }
    }

    /// Number of nodes
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    // TODO: I don't like that these peripheral methods take `&mut self` just
    // because they call bookkeeping_eve_index. I guess the only way around that
    // would be to just get over (the solution I'm probably going to go with) or
    // to apply interior mutability with RefCell.
    /// Are the eves of the nodes the same?
    pub fn is_linked(&mut self, a: usize, b: usize) -> bool {
        self.bookkeeping_eve_index(a) == self.bookkeeping_eve_index(b)
    }

    /// Merge a and b into the same set, if not already in the same
    pub fn link(&mut self, a: usize, b: usize) {
        assert!(a < self.nodes.len() && b < self.nodes.len());

        let a = self.bookkeeping_eve_index(a);
        let b = self.bookkeeping_eve_index(b);

        if a != b {
            self.nodes[a] = EveOrNode::Eve(self.eve(a).merge(self.eve(b)));
            self.nodes[b] = EveOrNode::Node(a);
        }
    }

    pub fn eve(&self, mut node: usize) -> &EveT {
        loop {
            match &self.nodes[node] {
                EveOrNode::Eve(result) => return result,
                EveOrNode::Node(parent) => node = *parent,
            }
        }
    }

    pub fn eve_mut(&mut self, mut node: usize) -> &mut EveT {
        loop {
            if let EveOrNode::Node(index) = self.nodes[node] {
                node = index;
            } else {
                break;
            }
        }

        match self.nodes[node] {
            EveOrNode::Eve(ref mut eve) => eve,
            EveOrNode::Node(_) => panic!("Should have found an eve"),
        }
    }

    /// Find the eve/root of the node while updating the parent of any node in between
    pub fn bookkeeping_eve_index(&mut self, mut node: usize) -> usize {
        let mut prev_node = None;
        loop {
            let parent = match &self.nodes[node] {
                EveOrNode::Eve(_) => return node,
                EveOrNode::Node(parent) => *parent,
            };

            // amortize the cost of looking up the eve of a node
            if let Some(prev_node) = prev_node {
                self.nodes[prev_node] = EveOrNode::Node(parent);
            }
            prev_node = Some(node);

            node = parent;
        }
    }

    /// Does a node map back to itself, i.e. it has no parent?
    pub fn is_an_eve(&self, node: usize) -> bool {
        match &self.nodes[node] {
            EveOrNode::Eve(_) => true,
            EveOrNode::Node(_) => false,
        }
    }
}

impl DisjointSetWithCount {
    // A more helpful version than the default Debug implementation
    pub fn debug_string(&mut self) -> String {
        use std::collections::hash_map::HashMap;
        let mut map = HashMap::<usize, Vec<usize>>::new();
        let mut eves = vec![];
        for node in 0..self.size() {
            if self.is_an_eve(node) {
                eves.push((node, **self.eve(node)));
            }
            map.entry(self.bookkeeping_eve_index(node))
                .or_default()
                .push(node);
        }

        fn debug_string(
            node: usize,
            count: usize,
            prefix: &str,
            map: &HashMap<usize, Vec<usize>>,
        ) -> String {
            match map.get(&node) {
                Some(children) => {
                    let mut result = String::new();
                    let prefix = if count > 0 {
                        format!("{}{}(count={}) <- ", prefix, node, count)
                    } else {
                        format!("{}{} <- ", prefix, node)
                    };

                    for child in children.iter() {
                        if child == &node {
                            continue;
                        }
                        result.push_str(&debug_string(*child, 0, &prefix, map));
                    }
                    result
                }
                None => format!("{}{}\n", prefix, node),
            }
        }

        let mut result = String::new();
        for (eve, count) in eves {
            result.push_str(&debug_string(eve, count, "", &map));
        }
        result.push('\n');
        return result;
    }
}

pub type DisjointSetWithCount = DisjointSet<Count>;

impl DisjointSetWithCount {
    /// Number of children of the node's eve (plus one for the eve itself)
    pub fn size_of_eve(&mut self, node: usize) -> usize {
        self.eve(node).0
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
