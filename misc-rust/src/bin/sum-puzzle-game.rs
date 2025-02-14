// TODO: Potentially unclear example. I am just committing this, because I'm tired of so many leftover git stashes

use itertools;

struct Permutations(u32);

struct PermutationIter {
    n: u32,
    index: u32,
    cur: Vec<u32>,
    sub: Option<Box<PermutationIter>>,
}

impl Permutations {
    fn iter(&self) -> PermutationIter {
        PermutationIter {
            n: self.0,
            index: (),
            cur: (),
            sub: (),
        }
    }
}

impl PermutationIter {
    fn new(n: u32) -> Self {
        if n > 1 {
            let mut sub = Box::new(PermutationIter::new(n - 1));
            PermutationIter {
                n,
                index: 0,
                cur: vec![
                    vec![n],
                    sub.next().expect(
                        "Internal error: every permutation should have at least one element",
                    ),
                ]
                .concat(),
                sub: Some(sub),
            }
        } else {
            PermutationIter {
                n,
                index: 0,
                cur: vec![n],
                sub: None,
            }
        }
    }
}

impl Iterator for PermutationIter {
    type Item = &Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {}
}

// trait Permutations {}

// impl<T> Permutations for dyn Iterator<Item = T> {}

impl Iterator for Permutations {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    let a = Permutations(1);
    // let mut i = (0..5).peekable();
    (0..5).per;
    println!("{}", i.peek().unwrap());
    println!("{}", i.peek().unwrap());
    println!("{}", a.0);
}
