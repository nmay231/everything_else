use std::cmp::Ordering;
use std::ops::Range;

fn parse_int(x: &str) -> Option<usize> {
    if x.len() == 0 {
        None
    } else {
        Some(x.parse().unwrap())
    }
}

struct OverlappingMatches<T> {
    ours: Range<T>,
    sub_range: Range<T>,
}

struct OverlapPair<T> {
    does: Range<T>,
    not: Range<T>,
}

enum CheckInterleave<T> {
    OverlapsFirst {
        overlap: Vec<Range<T>>,
        none: Vec<Range<T>>,
    },
    OverlapsSecond {
        overlap: Vec<Range<T>>,
        none: Vec<Range<T>>,
    },
}
use CheckInterleave as CI;

// enum CheckInterleave{
//     NoOverlap(Range<T>),
//     StartOverlaps(Range<T>, Vec<OverlapPair<T>>, Option<Range<T>>),
//     EndOverlaps(Option<Range<T>>, Vec<OverlapPair<T>>, Range<T>),
// }

trait RangeMath<T> {
    fn sort_ranges(&mut self);
    fn find_range(&self, range: &Range<T>) -> Range<usize>;
    fn insert_range(&mut self, new_range: Range<T>);
    // fn check_interleave(&self, new_range: Range<T>) -> CheckInterleave<T>;
    // fn overlapping_matches(&self, range: Range<T>) -> Vec<OverlappingMatches<T>>;
}

impl<T: Ord + Copy> RangeMath<T> for Vec<Range<T>> {
    fn sort_ranges(&mut self) {
        self.sort_by(|a, b| {
            if a.end <= b.start {
                return Ordering::Less;
            } else if b.end <= a.start {
                return Ordering::Greater;
            } else {
                //  if (a.start <= b.start && b.end <= a.end) || (b.start <= a.start && a.end <= b.end)
                panic!("cannot sort ranges that overlap!");
            }
        })
    }
    fn find_range(&self, range: &Range<T>) -> Range<usize> {
        let new_range = range;
        if self.len() == 0 || new_range.end < self[0].start {
            return 0..0;
        } else if new_range.end <= self[0].end {
            return 0..1;
        }

        let mut start = self.len();
        let mut overlapping = false;

        for (i, range) in self.iter().enumerate() {
            if !overlapping {
                if new_range.end < range.start {
                    return i..i;
                } else if new_range.end == range.start {
                    return i..i + 1;
                } else if new_range.start <= range.end {
                    overlapping = true;
                    start = i;
                }
            }

            if overlapping {
                if new_range.end < range.start {
                    return start..i;
                } else if new_range.end <= range.end {
                    return start..i + 1;
                }
            }
        }

        return start..self.len();
    }

    fn insert_range(&mut self, new_range: Range<T>) {
        let Range { start, end } = self.find_range(&new_range);

        if start == end {
            self.insert(start, new_range);
        } else {
            self.splice(
                start..end,
                [std::cmp::min(self[start].start, new_range.start)
                    ..std::cmp::max(self[end - 1].end, new_range.end)],
            );
        }
    }
}

fn main() -> std::io::Result<()> {
    // println!("{:?}", std::env::current_dir());
    let text = std::fs::read_to_string("./assets/day5.txt")?;
    let mut lines = text.lines().peekable();
    let mut current = lines.next().unwrap()[7..]
        .split(" ")
        .filter_map(parse_int)
        .collect::<Vec<_>>()
        .windows(2)
        .step_by(2)
        .map(|window| {
            if let [end, start] = window {
                return *start..*end;
            } else {
                panic!("Non-pair of windows");
            }
        })
        .collect::<Vec<_>>();

    lines.next(); // Skip empty line

    while lines.peek() != None {
        lines.next(); // Skip header

        let mut maps = vec![];
        loop {
            let triple = match lines.next() {
                None => break,
                Some(x) => x.split(" ").filter_map(parse_int).collect::<Vec<_>>(),
            };

            if triple.len() == 0 {
                break;
            }

            if let [dest_start, source_start, count] = triple[..3] {
                maps.push((source_start, dest_start, count));
            } else {
                panic!("Expected a number triple");
            }
        }

        let len = current.len();
        let mut maps_domain = maps
            .iter()
            .map(|(source, _, count)| source.to_owned()..source + count)
            .collect::<Vec<_>>();
        maps_domain.sort_ranges();

        current = current
            .into_iter()
            .flat_map(|seeds| {
                let mut outputs = vec![];
                let mut seeds = seeds;
                for i in maps_domain.find_range(&seeds) {
                    let range = &maps_domain[i];
                    if seeds.start < range.start {
                        outputs.push(seeds.start..range.start);
                    }
                    let (dest, count) = maps
                        .iter()
                        .find_map(|(source, dest, count)| {
                            if *source == range.start && source + count == range.end {
                                Some((dest, count))
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    outputs.push(
                        std::cmp::max(*dest, seeds.start + dest - range.start)
                            ..std::cmp::min(dest + count, seeds.end + dest - range.start),
                    );
                    seeds = range.end..seeds.end;
                }
                if seeds.len() > 0 {
                    outputs.push(seeds);
                }
                return outputs;
                // let x = ;

                let mut inputs = vec![seeds.to_owned()];
                let mut outputs = vec![];

                // Some inputs don't fully overlap with the source => destination map, so we chop them up into pieces that do.
                while let Some(input) = inputs.pop() {
                    // Too lazy to care about copying a few numbers
                    for (source, dest, count) in maps.to_owned().into_iter() {
                        if source <= input.start && input.end <= source + count {
                            outputs.push(input.start + dest - source..input.end + dest - source);
                        } else if input.start < source && source + count < input.end {
                            outputs.push(source..source + count);
                            inputs.push(input.start..source);
                            inputs.push(source + count..input.end);
                        } else if input.start < source && source < input.end {
                            outputs.push(source..input.end);
                            inputs.push(input.start..source - 1);
                        } else if input.start <= source + count && source + count < input.end - 1 {
                            outputs.push(input.start..source + count);
                            inputs.push(source + count + 1..input.end);
                        } else {
                            continue;
                        }
                        break;
                    }

                    // If the input doesn't overlap with any maps, it's added as is.
                    if input.len() > 0 {
                        outputs.push(input);
                    }

                    println!("{:?}", outputs);
                }
                return outputs;
            })
            .collect();
        println!("{}", current.len());
        // break;
    }

    // 35355313 too high
    println!(
        "lowest final value = {}",
        current.iter().map(|range| range.start).min().unwrap()
    );

    return Ok(());
}

#[cfg(test)]
mod test {
    use std::ops::Range;

    use crate::RangeMath;
    use rstest::rstest;

    #[rstest]
    #[case::empty_start(vec![], 0..10, vec![0..10])]
    #[case::before_all(vec![5..10, 11..15], 0..4, vec![0..4, 5..10, 11..15])]
    #[case::after_all(vec![5..10, 11..15], 16..20, vec![5..10, 11..15, 16..20])]
    #[case::right_before_all(vec![5..10, 11..15], 0..5, vec![0..10, 11..15])]
    #[case::right_after_all(vec![5..10, 11..15], 15..20, vec![5..10, 11..20])]
    #[case::between_no_overlap(vec![1..5, 11..15], 6..10, vec![1..5, 6..10, 11..15])]
    #[case::between_overlap_start(vec![1..5, 11..15], 5..10, vec![1..10, 11..15])]
    #[case::between_overlap_end(vec![1..5, 11..15], 6..11, vec![1..5, 6..15])]
    #[case::between_overlap_both(vec![1..5, 11..15], 5..11, vec![1..15])]
    #[case::between_overshoot_both(vec![3..5, 11..13], 1..15, vec![1..15])]
    #[case::overlap_multiple(vec![1..5, 6..10, 11..15], 3..12, vec![1..15])]
    fn single_insert(
        #[case] mut start: Vec<Range<usize>>,
        #[case] range: Range<usize>,
        #[case] expected: Vec<Range<usize>>,
    ) {
        start.insert_range(range);
        assert_eq!(start, expected);
    }
}
