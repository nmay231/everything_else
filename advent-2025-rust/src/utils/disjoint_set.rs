use std::fmt::Debug;
use std::ops::Deref;

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
