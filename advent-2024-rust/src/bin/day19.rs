use std::collections::{HashMap, VecDeque};

use advent_2024_rust::Zipper;
use itertools::Itertools;

type Output = usize;

#[derive(Debug, Clone, Default)]
struct Trie {
    map: HashMap<char, Self>,
    is_terminal: bool,
}

#[derive(Debug, Clone)]
struct TrieZipper {
    parents: Vec<(Trie, char)>,
    current: Trie,
}

impl Zipper for TrieZipper {
    type Source = Trie;

    type Index = char;

    fn new(root: Self::Source) -> Self {
        Self {
            current: root,
            parents: vec![],
        }
    }

    fn source(&mut self) -> &mut Self::Source {
        &mut self.current
    }

    fn child(mut self, index: Self::Index) -> Result<Self, Self> {
        match self.current.map.remove(&index) {
            None => Err(self),
            Some(child) => {
                self.parents.push((self.current, index));
                self.current = child;
                Ok(self)
            }
        }
    }

    fn parent(mut self) -> Result<Self, Self> {
        match self.parents.pop() {
            None => Err(self),
            Some((mut parent, key)) => {
                parent.map.insert(key, self.current);
                self.current = parent;
                Ok(self)
            }
        }
    }

    fn unwrap_source(self) -> Self::Source {
        self.current
    }
}

fn part1(text: &str) -> Output {
    let towels = text.split_once('\n').expect("Missing towels").0.split(", ");

    let mut trie = TrieZipper::new(Trie::default());
    for towel in towels {
        for char in towel.chars() {
            trie.source().map.entry(char).or_default();
            trie = trie.child(char).unwrap();
        }
        trie.source().is_terminal = true;
        trie = trie.to_root();
    }
    let trie = trie.unzip();

    // Sanity check
    assert!(!trie.is_terminal, "There shouldn't be empty towel patterns");
    let towels = text.split_once('\n').expect("Missing towels").0.split(", ");
    for towel in towels {
        let mut tmp = &trie;
        for char in towel.chars() {
            tmp = tmp.map.get(&char).expect("towels to be init'ed correctly");
        }
        assert!(tmp.is_terminal);
    }

    let mut total = 0;
    'patterns: for pattern in text.lines().skip(2) {
        println!("pattern: {}", pattern);
        let pattern = pattern.chars().collect_vec();
        let mut frontier = VecDeque::from([(&pattern[..], &trie)]);

        while let Some((pattern, current_towel)) = frontier.pop_back() {
            match (pattern.first(), current_towel.is_terminal) {
                // Perfect match
                (None, true) => {
                    total += 1;
                    continue 'patterns;
                }
                // Leftover towel
                (None, false) => continue,
                // Need another towel to continue the pattern
                (Some(char), is_terminal) => {
                    if is_terminal {
                        frontier.push_back((pattern, &trie));
                    }
                    match current_towel.map.get(char) {
                        None => continue,
                        Some(child) => {
                            frontier.push_back((&pattern[1..], child));
                        }
                    }
                }
            }
        }
    }

    total
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day19.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        bwu
        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 7);
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
