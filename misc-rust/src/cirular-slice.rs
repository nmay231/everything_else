pub trait CircularSliceTrait: Iterator {
    type Item;
    fn cycle_slice(self, range: std::ops::Range<isize>) -> CircularSlice<Self>
    where
        Self: Sized,
    {
        CircularSlice::new(self, range)
    }
}

pub struct CircularSlice<I> {
    inner: I,
    // iter: I,
}

impl<I: Clone + Iterator> CircularSlice<I> {
    pub fn new(iter: I, range: std::ops::Range<isize>) -> CircularSlice<I> {
        if 0 <= range.start && range.start <= range.end {
            CircularSlice {
                inner: iter
                    .skip(range.start as usize)
                    .take((range.end - range.start) as usize),
            }
        } else {
            CircularSlice { inner: iter }
        }
    }
}

impl<I> Iterator for CircularSlice<I> {
    type Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// trait CircularSlice {
//     fn circular_slice(&self, range: std::ops::Range<isize>) -> Into<Item = Self::Item>
//     where
//         Self: Iterator
//     //     ,
//     // {
//     //     let x = vec![1, 2, 3, 4];
//     //     (&x[..]).ro
//     //     [].iter().cycle()
//     //     return self.into_iter().cycle();
//     // }
// }
