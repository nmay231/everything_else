#![feature(assert_matches)]
use itertools::{Combinations, Itertools};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Range;
use std::time::Instant;

// This is so cool! Rust type aliases are powerful.
type UndirectedGraph<'a, T = &'a str> = HashMap<T, HashSet<T>>;

type EdgeVisitCount<'a, T = &'a str> = HashMap<[T; 2], usize>;

trait EdgeVisitTrait<T> {
    fn increment_edge(&mut self, edge: [T; 2]);
}

impl<'a> EdgeVisitTrait<&'a str> for EdgeVisitCount<'a, &'a str> {
    fn increment_edge(&mut self, mut edge: [&'a str; 2]) {
        edge.sort();
        self.entry(edge)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
}

trait UndirectedGraphTrait<'a, T> {
    fn add_edge(&'a mut self, a: T, b: T);
    fn remove_edge(&'a mut self, a: T, b: T);
    fn all_edges(&'a self) -> Vec<(&'a T, &'a T)>;
}

impl<'a, T: Eq + Hash + Copy + PartialOrd> UndirectedGraphTrait<'a, T> for UndirectedGraph<'a, T> {
    fn add_edge(&mut self, a: T, b: T) {
        self.entry(a).or_default().insert(b);
        self.entry(b).or_default().insert(a);
    }

    fn remove_edge(&mut self, a: T, b: T) {
        let neighbors = self.entry(a).or_default();
        neighbors.remove(&b);
        if neighbors.len() == 0 {
            self.remove(&a);
        }
        let (a, b) = (b, a);
        let neighbors = self.entry(a).or_default();
        neighbors.remove(&b);
        if neighbors.len() == 0 {
            self.remove(&a);
        }
    }

    // TODO: Too lazy right now to implement as an iterator. I really can't wait
    // TODO: for generators
    fn all_edges(&self) -> Vec<(&T, &T)> {
        let mut edges = vec![];

        for (a, bs) in self {
            for b in bs {
                if a <= b {
                    edges.push((a, b));
                }
            }
        }

        edges
    }
}

fn breadth_first_connect<'a>(
    graph: &'a UndirectedGraph,
    a: &'a str,
    b: &'a str,
) -> Option<Vec<&'a str>> {
    assert_ne!(a, b);

    let mut paths = VecDeque::new();
    paths.extend([vec![a]]);
    let mut visited = HashSet::new();
    visited.insert(a);

    while let Some(mut path) = paths.pop_front() {
        for neigh in graph.get(path.last().unwrap()).unwrap() {
            if *neigh == b {
                path.push(b);
                return Some(path);
            } else if !visited.contains(neigh) {
                visited.insert(*neigh);
                let mut path = path.to_owned(); // Copy for each neighbor
                path.push(*neigh);
                paths.push_back(path);
            }
        }
    }
    None
}

fn connected_to(graph: &UndirectedGraph, target: &str) -> usize {
    let mut next = VecDeque::new();
    next.push_back(target);
    let mut visited = HashSet::new();

    while let Some(vert) = next.pop_front() {
        visited.insert(vert);
        next.extend(
            graph
                .get(vert)
                .unwrap()
                .iter()
                .filter_map(|v| (!visited.contains(v)).then_some(*v)),
        )
    }

    visited.len()
}

fn parse_graph(text: &str) -> UndirectedGraph {
    let mut graph = UndirectedGraph::<&str>::new();
    for line in text.lines() {
        let (source, line) = line.split_once(": ").unwrap();
        for dest in line.split(" ") {
            graph.add_edge(source, dest);
        }
    }
    graph
}

fn part1_statistical(text: &str) -> bool {
    // TODO: My original thought process. I now think that brute force would
    // have worked fine (since I only actually have 1527 vertices), but I only
    // know once I test it (and I will do that).
    //
    // I think the way to solve this is to count the times an edge is visited
    // when traveling from one vertex to another and the three edges that are
    // traveled the most are the connected ones. Or at least, I can try removing
    // three edges sorted by highest number of connections and that should work
    // better than randomly doing it.
    //
    // Then again, if the graph is small enough, I could just brute force...
    // 6852 edges seems like too many for that, unfortunately... Actually, I was
    // parsing wrong; it's 2354 nodes, 2492 edges. Still a bit too large. `2492
    // choose 3` = 3,103,786
    //
    // I think I'll use a more statistical approach. I'll travel breadth-first
    // between two random vertices through the graph 1000 times and pick the
    // edge used the most. Maybe I can do this check every 300 searches since
    // that should help. Anyways, we then see if that edge is one of the
    // king-pins by removing it, finding the path between the points again,
    // removing that path, repeat last two steps if possible, and then see if
    // the graph is disjoint. If it is, then repeatedly reduce the path(s) down
    // to two edges and if it's still disjoint after removing those two edges,
    // then those are the three king-pin edges.

    let graph = parse_graph(text);
    let rng = &mut thread_rng();
    let vertices = graph.keys().map(|s| *s).collect_vec();

    let mut edge_visit = EdgeVisitCount::new();
    for i in 0..300 {
        let v1 = *vertices.choose(rng).unwrap();
        let mut v2;
        loop {
            v2 = *vertices.choose(rng).unwrap();
            if v2 != v1 {
                break;
            }
        }

        let tmp = breadth_first_connect(&graph, v1, v2);
        match tmp {
            Some(path) => {
                for (v1, v2) in path.iter().skip(1).zip(path.iter()) {
                    edge_visit.increment_edge([*v1, *v2]);
                }
            }
            None => {
                println!("{:?}", (i, v1, v2));
                panic!("Original graph should be fully connected")
            }
        }
    }

    let mut most_traveled = edge_visit.iter().collect_vec();
    most_traveled.sort_by_key(|(_, amount)| **amount);
    most_traveled.reverse();
    let most_traveled = most_traveled
        .into_iter()
        // .take(3)
        .map(|(pair, _)| pair.map(|s| s.to_owned()))
        .collect_vec();

    let original_len = graph.len();
    for triplet in most_traveled.iter().combinations(3) {
        let mut graph = graph.clone();
        for (i, [a, b]) in triplet.iter().enumerate() {
            // assert_eq!(connected_to(&graph, &a), original_len);
            graph.remove_edge(a, b);
            if i == 2 {
                let first_half = connected_to(&graph, &a);
                if first_half < original_len {
                    assert_eq!(first_half + connected_to(&graph, &b), original_len);
                    return true;
                }
            }
        }
    }
    false
}

fn part1_brute_force(text: &str) -> bool {
    let graph = parse_graph(text);
    let edges = graph.all_edges();

    let original_len = graph.len();
    for (i, triplet) in edges.iter().combinations(3).enumerate() {
        if i % 100 == 0 {
            println!("inner_run {:?}", (i));
        }
        let mut graph = graph.clone();
        for (i, (a, b)) in triplet.iter().enumerate() {
            // assert_eq!(connected_to(&graph, &a), original_len);
            graph.remove_edge(a, b);
            if i == 2 {
                let first_half = connected_to(&graph, &a);
                if first_half < original_len {
                    assert_eq!(first_half + connected_to(&graph, &b), original_len);
                    return true;
                }
            }
        }
    }
    false
}

struct CombinationsPreferFirst<V> {
    reference: Vec<V>,
    indices: Combinations<Range<usize>>,
}

fn combinations_prefer_first<V>(of: Vec<V>, k: usize) -> CombinationsPreferFirst<V> {
    let len = of.len();
    CombinationsPreferFirst {
        reference: of,
        indices: (0..len).combinations(len - k),
    }
}

impl<V: Copy + Debug> Iterator for CombinationsPreferFirst<V> {
    type Item = Vec<V>;

    fn next(&mut self) -> Option<Self::Item> {
        let indices = self.indices.next()?;
        let tmp = self
            .reference
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if indices.contains(&(self.reference.len() - i - 1)) {
                    None
                } else {
                    Some(*x)
                }
            })
            .collect_vec();
        // println!("{:?}", (indices, &tmp));
        return Some(tmp);
    }
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day25.txt")?;

    // Only looking at the top 3 most traveled after 100 random walks between
    // two points had 792/1000 successful runs after 133.557 seconds (had
    // 808/1000 before I timed it).
    //
    // Looking at the top ten most traveled (10 choose 3 = 120 graph copies and
    // mutations) after 100 random walks had 994/1000 successful runs after
    // 145.234 seconds.
    //
    // Looking at the top three most traveled after 100 random walks had
    // 1956/2000 successful runs after 732.749 seconds (~467 seconds for 1000
    // runs).
    //
    // Looking at every possible triplet while preferring the most traveled
    // after 100 random walks had 2000/2000 successful runs after 735.559
    // seconds
    //
    // Brute force didn't even complete once after an hour, which makes sense
    // since it's `O(edges.len() ** 3)`.
    let now = Instant::now();
    let mut success = 0;
    let runs = 100;
    for run in 0..runs {
        if true {
            println!("status: {run}");
        }

        if part1_brute_force(&text) {
            success += 1;
        } else {
            println!("failure on run={}", run);
        }
    }
    println!(
        "{success}/{runs} successful runs after {:.3} seconds",
        now.elapsed().as_secs_f32()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use std::collections::HashSet;

    use indoc::indoc;
    use itertools::Itertools;
    use rstest::rstest;

    use crate::{
        breadth_first_connect, combinations_prefer_first, connected_to, parse_graph,
        UndirectedGraph,
    };

    #[test]
    fn test_parse_graph() {
        let text = indoc! {"
        a: b c
        b: c d
        c: e"};
        let mut expected = UndirectedGraph::new();
        expected.extend([
            ("a", HashSet::from_iter(["b", "c"])),
            ("b", HashSet::from_iter(["a", "c", "d"])),
            ("c", HashSet::from_iter(["a", "b", "e"])),
            ("d", HashSet::from_iter(["b"])),
            ("e", HashSet::from_iter(["c"])),
        ]);
        assert_eq!(expected, parse_graph(text));
    }

    #[rstest]
    fn test_connected_graph(#[values(true, false)] breadth_first: bool) {
        let text = indoc! {"
        a1: b1 c1
        d1: b1 c1 d2
        d2: b2 c2
        a2: b2 c2"};
        let graph = &parse_graph(text);

        if breadth_first {
            let result = breadth_first_connect(graph, "a1", "a2");
            assert!(result.is_some());
            let path = result.unwrap();

            assert_matches!(
                &path[..],
                &["a1", "b1" | "c1", "d1", "d2", "b2" | "c2", "a2"]
            );
        } else {
            assert_eq!(connected_to(graph, "a1"), 8);
        }
    }

    #[rstest]
    fn test_disconnected_graph(#[values(true, false)] breadth_first: bool) {
        let text = indoc! {"
        a1: b1 c1
        d1: b1 c1
        d2: b2 c2
        a2: b2 c2"};
        let graph = &parse_graph(text);

        if breadth_first {
            let path = breadth_first_connect(graph, "a1", "a2");
            assert_eq!(path, None);
        } else {
            assert_eq!(connected_to(graph, "a1"), 4);
            assert_eq!(connected_to(graph, "a2"), 4);
        }
    }

    #[test]
    fn test_combinations_prefer_first() {
        let tmp = combinations_prefer_first(Vec::from_iter("abcdef".chars()), 3).collect_vec();
        assert_eq!(
            &tmp,
            &[
                //              abcdef
                Vec::from_iter("abc   ".replace(" ", "").chars()),
                Vec::from_iter("ab d  ".replace(" ", "").chars()),
                Vec::from_iter("a cd  ".replace(" ", "").chars()),
                Vec::from_iter(" bcd  ".replace(" ", "").chars()),
                Vec::from_iter("ab  e ".replace(" ", "").chars()),
                Vec::from_iter("a c e ".replace(" ", "").chars()),
                Vec::from_iter(" bc e ".replace(" ", "").chars()),
                Vec::from_iter("a  de ".replace(" ", "").chars()),
                Vec::from_iter(" b de ".replace(" ", "").chars()),
                Vec::from_iter("  cde ".replace(" ", "").chars()),
                Vec::from_iter("ab   f".replace(" ", "").chars()),
                Vec::from_iter("a c  f".replace(" ", "").chars()),
                Vec::from_iter(" bc  f".replace(" ", "").chars()),
                Vec::from_iter("a  d f".replace(" ", "").chars()),
                Vec::from_iter(" b d f".replace(" ", "").chars()),
                Vec::from_iter("  cd f".replace(" ", "").chars()),
                Vec::from_iter("a   ef".replace(" ", "").chars()),
                Vec::from_iter(" b  ef".replace(" ", "").chars()),
                Vec::from_iter("  c ef".replace(" ", "").chars()),
                Vec::from_iter("   def".replace(" ", "").chars()),
            ],
        )
    }
}
