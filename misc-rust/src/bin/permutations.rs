// Source: https://www.youtube.com/watch?v=jUM_Dpt6yu0
// This is going to (mostly) be implemented from memory

use std::fmt::Display;
use std::mem;

// I really wish I could use recursive generators to gain an understanding of this before reimplementing it this way
// I probably could enable generators using rust nightly, but I don't want to go down that rabbit hole yet...
struct Permute<T> {
    copy: Vec<T>,
    indices: Vec<usize>,
    // sub: Option<Box<Permute<'a, T>>>,
}

impl<T: Copy> Permute<T> {
    fn from(vec: &Vec<T>) -> Self {
        Permute {
            // Why the @!(&%$#*#& do I have to copy and deref the values myself?!
            copy: vec.into_iter().map(|x| *x).collect(),
            indices: (0..vec.len()).collect(),
            // sub: None,
        }
    }
}

impl<T> Permute<T> {
    fn test(&mut self) -> Option<T> {
        self.copy.pop()
    }
}
// impl<T> Iterator for Permute<T> {
//     type Item = Vec<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.copy.pop();
//         // if self.copy.len() > 0 {
//         //     mem::swap
//         // }
//     }
// }

// I need to read all of what I was doing, maybe make a note with all the in-progress things. Decide on to do, what not to, and in what order

fn main() {
    let mut v = vec![1, 2, 3];
    let mut perm = Permute::from(&v);
    println!("{};; {}", perm.test().unwrap(), v.pop().unwrap());
    //     let v: Vec<i32> = (0..5).collect();
    //     let per = Permute::from(&v);
    //     for x in per {
    //         println!("{}", format_vec(&x));
    //     }
    // }
}

fn format_vec<T: Display>(vec: &Vec<T>) -> String {
    format!(
        "vec![{}]",
        vec.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

mod test {
    use itertools::assert_equal;

    #[test]
    fn compare_iters() {
        assert_equal(vec![0, 1, 2, 3, 4], 0..=5);
    }
}
