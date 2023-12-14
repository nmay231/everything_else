use std::marker::PhantomData;
use std::ops::{Bound, Range, RangeBounds};
use std::slice::SliceIndex;

type Output = usize;

pub fn count_spring_row_configs(springs: &str, contiguous_broken_count: Vec<usize>) -> usize {
    0
}

enum Which<T> {
    Broken(T),
    Working(T),
    Unknown(T),
}

impl<T> Which<T> {
    fn unwrap(&self) -> &T {
        match self {
            Which::Broken(n) => n,
            Which::Working(n) => n,
            Which::Unknown(n) => n,
        }
    }
}

trait IndexList<'a, T, I, R: RangeBounds<usize>>: std::ops::Index<I, Output = T> {
    fn iter_range(&'a self, range: R) -> IterIndexes<'a, T, I, R, dyn IndexList<'a, T, I, R>>;
    fn unwrap_index(&self, index: usize) -> I;
}

struct IterIndexes<'a, T, I, R: RangeBounds<usize>, IL: IndexList<'a, T, I, R> + ?Sized> {
    refer: &'a IL,
    range: R,
    index: usize,
    value_type: PhantomData<T>,
    index_type: PhantomData<I>,
}

impl<'a, T, I, R: RangeBounds<usize>, IL: IndexList<'a, T, I, R> + ?Sized>
    IterIndexes<'a, T, I, R, IL>
{
    fn new(refer: &'a IL, range: R) -> Self {
        Self {
            refer,
            range,
            index: 0,
            value_type: PhantomData,
            index_type: PhantomData,
        }
    }
}

impl<'a, T, I, R: RangeBounds<usize>, IL: IndexList<'a, T, I, R> + ?Sized> Iterator
    for IterIndexes<'a, T, I, R, IL>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let start = match self.range.start_bound() {
            Bound::Unbounded => usize::MIN,
            Bound::Included(s) => *s,
            Bound::Excluded(s) => s.saturating_add(1),
        };
        let end = match self.range.end_bound() {
            Bound::Unbounded => usize::MAX,
            Bound::Included(&e) => e,
            Bound::Excluded(e) => e.saturating_sub(1),
        };

        // TODO: Don't construct a range each time I need to get the next item
        todo!();
        // let index = self.index;
        // self.index += 1;
        // return (start..end).nth(index).map(|i| self.refer.unwrap_index(i));
    }
}

impl<'a, R: RangeBounds<usize>> IndexList<'a, usize, usize, R> for [usize]
where
    Self: Sized,
{
    fn iter_range(
        &'a self,
        range: R,
    ) -> IterIndexes<'a, usize, usize, R, dyn IndexList<'a, usize, usize, R>> {
        return IterIndexes::new(*Box::new(self).as_ref(), range);
    }

    fn unwrap_index(&self, index: usize) -> usize {
        return self[index];
    }
}

/// Returns the number of configurations possible given a set of runs of `Which`
/// # Arguments
///
/// * `runs`
/// * `working_then_broken` - The alternating length of runs of working and broken or unknown, starting with the number of working.
///                           Any amount of working springs at the end are irrelevant and are not passed to this function.
fn _count_spring_row_configs(
    springs: &str,
    runs: &[Which<usize>],
    maybe_broken: &[usize],
    known_broken: &[usize],
) -> usize {
    if known_broken.len() == 0 {
        // There is one way to satisfy no requirements
        return 1;
    } else if maybe_broken.len() == 0 || maybe_broken.iter().sum() < known_broken.iter().sum() {
        // There must be a configuration where the known broken can fit into possible slots
        return 0;
    }

    let next_broken_block = runs
        .iter()
        .enumerate()
        .find_map(|(i, run)| match run {
            Which::Working(_) => Some(i + 1), // The index after the first working group
            Which::Broken(_) | Which::Unknown(_) => None,
        })
        .or(Some(runs.len()))
        .unwrap();
    let next_broken_start = *runs[next_broken_block].unwrap();

    if known_broken[0] > maybe_broken[0] {
        // The slot is too small for this block of broken springs. Skip it.
        return _count_spring_row_configs(
            &springs[next_broken_start..],
            &runs[next_broken_block..],
            maybe_broken,
            known_broken,
        );
    // } else if known_broken[0] == maybe_broken[0] {
    //     // This block doesn't affect the total count. TODO: NOT TRUE, it could also start on the next block...
    //     return _count_spring_row_configs(
    //         &springs[next_broken_start..],
    //         &runs[next_broken_block..],
    //         &maybe_broken[1..],
    //         &known_broken[1..],
    //     );
    } else {
        let if_skipping_this_block = _count_spring_row_configs(
            &springs[next_broken_start..],
            &runs[next_broken_block..],
            maybe_broken,
            known_broken,
        );

        return if_skipping_this_block
            + (0..=(maybe_broken[0] - known_broken[0]))
                .map(|offset| {
                    let end = known_broken[0] + offset;
                })
                .sum();
        // for offset in  {

        // }

        let mut run_index;
        let mut broken = known_broken[0];
        for (run_index_, run) in runs.iter().enumerate() {
            let run = run.unwrap();
            if *run < broken {
                broken -= run;
            } else {
                run_index = run_index_;
                break;
            }
        }
        // let index_to_runs_indexes = (index into runs, index into that run of springs);

        return (known_broken[0]..maybe_broken[0]).map(|end| {}).sum();
    }
}

fn part1(text: &str) -> Output {
    0
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day12.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::count_spring_row_configs;

    #[rstest]
    #[case("???.###", vec![1,1,3], 1)]
    #[case(".??..??...?##.", vec![1,1,3], 4)]
    #[case("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6], 1)]
    #[case("????.#...#...", vec![4, 1, 1], 1)]
    #[case("????.######..#####.", vec![1,6,5], 4)]
    #[case("?###????????", vec![3,2,1], 10)]
    fn test_count_spring_row_configs(
        #[case] springs: &str,
        #[case] broken: Vec<usize>,
        #[case] expected: usize,
    ) {
        let actual = count_spring_row_configs(springs, broken);
        assert_eq!(actual, expected);
    }
}
