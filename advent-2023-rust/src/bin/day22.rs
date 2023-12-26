use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::str::FromStr;

type Output = usize;

#[derive(Debug, PartialEq, Eq, Ord)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn can_see_linearly(&self, other: &Self) -> bool {
        (self.x == other.x && self.y == other.y)
            || (self.x == other.x && self.z == other.z)
            || (self.y == other.y && self.z == other.z)
    }
}

impl FromStr for Point {
    type Err = (); // TODO: Too lazy, right now

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, s) = s.split_once(',').ok_or(())?;
        let (y, z) = s.split_once(',').ok_or(())?;
        Ok(Self::new(
            x.parse().or(Err(()))?,
            y.parse().or(Err(()))?,
            z.parse().or(Err(()))?,
        ))
    }
}

impl PartialOrd for Point {
    /// Sort by z axis first
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.z.partial_cmp(&other.z) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.x.partial_cmp(&other.x)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Line {
    XRange(Range<usize>, usize, usize),
    YRange(usize, Range<usize>, usize),
    ZRange(usize, usize, Range<usize>),
}

#[derive(Debug, PartialEq, Eq)]
struct PointPointLine(Point, Point, Line);

impl Line {
    fn all_points(&self) -> Vec<Point> {
        match self {
            Line::XRange(range, y, z) => range.clone().map(|x| Point::new(x, *y, *z)).collect(),
            Line::YRange(x, range, z) => range.clone().map(|y| Point::new(*x, y, *z)).collect(),
            Line::ZRange(x, y, range) => range.clone().map(|z| Point::new(*x, *y, z)).collect(),
        }
    }
}

impl TryInto<PointPointLine> for (Point, Point) {
    type Error = ();

    fn try_into(self) -> Result<PointPointLine, Self::Error> {
        let (this, other) = self;
        if this.x == other.x && this.y == other.y {
            let range = std::cmp::min(this.z, other.z)..1 + std::cmp::max(this.z, other.z);
            let line = Line::ZRange(this.x, this.y, range);
            Ok(PointPointLine(this, other, line))
        } else if this.x == other.x && this.z == other.z {
            let range = std::cmp::min(this.y, other.y)..1 + std::cmp::max(this.y, other.y);
            let line = Line::YRange(this.x, range, this.z);
            Ok(PointPointLine(this, other, line))
        } else if this.y == other.y && this.z == other.z {
            let range = std::cmp::min(this.x, other.x)..1 + std::cmp::max(this.x, other.x);
            let line = Line::XRange(range, this.y, this.z);
            Ok(PointPointLine(this, other, line))
        } else {
            Err(())
        }
    }
}

impl PartialOrd for PointPointLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.z.partial_cmp(&other.0.z)
    }
}

impl Ord for PointPointLine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part1(text: &str) -> Output {
    let mut bricks: Vec<PointPointLine> = vec![];
    for line in text.lines() {
        let (a, b) = line.split_once('~').unwrap();
        let a = a.parse::<Point>().unwrap();
        let b = b.parse::<Point>().unwrap();

        assert!(a.can_see_linearly(&b));

        // TODO: Need to see if I can make the following work instead of matching on .cmp()
        // let mut pair = [&a, &b];
        // pair.sort_unstable();
        // bricks.push((*pair[0], *pair[1]));

        match a.cmp(&b) {
            std::cmp::Ordering::Greater => bricks.push((b, a).try_into().unwrap()),
            // While Ordering is unlikely to add/remove variants, it's good practice to explicitly list all variants for ones that might
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {
                bricks.push((a, b).try_into().unwrap())
            }
        }
    }

    bricks.sort();
    let mut max = Point::new(0, 0, 0);
    for PointPointLine(a, b, _line) in &bricks {
        max.x = std::cmp::max(max.x, std::cmp::max(a.x, b.x));
        max.y = std::cmp::max(max.y, std::cmp::max(a.y, b.y));
    }

    // Which bricks lay on top of others
    let mut dependencies = HashMap::<usize, HashSet<usize>>::new();
    // (height above ground, label of brick or 0 for ground)
    let mut height_map = vec![vec![(0, 0); max.x + 1]; max.y + 1];
    for (label, PointPointLine(a, b, line)) in bricks.iter().enumerate() {
        let label = label + 1;
        // println!("{:?}", (label, a, b, line));
        if a.z != b.z {
            let (prev_height, prev_label) = height_map[a.y][a.x];
            height_map[a.y][a.x] = (prev_height + b.z.abs_diff(a.z) + 1, label);
            dependencies.entry(label).or_default().insert(prev_label);
        } else {
            let points = line.all_points();
            let max_height = points.iter().fold(0, |max, point| {
                std::cmp::max(max, height_map[point.y][point.x].0)
            });
            for point in &points {
                let (prev_height, prev_label) = height_map[point.y][point.x];
                height_map[point.y][point.x] = (max_height + 1, label);
                if prev_height == max_height {
                    dependencies.entry(label).or_default().insert(prev_label);
                }
            }
        }

        // let colored = line
        //     .all_points()
        //     .iter()
        //     .map(|point| ((point.x, point.y)))
        //     .collect::<Vec<_>>();
        // for (row_i, row) in height_map.iter().enumerate() {
        //     let line = row
        //         .iter()
        //         .enumerate()
        //         .map(|(col_i, (height, _))| {
        //             if colored.contains(&(col_i, row_i)) {
        //                 format!("\x1b[1;31m{: >3}\x1b[0m", height)
        //             } else {
        //                 format!("{: >3}", height)
        //             }
        //         })
        //         .collect::<Vec<_>>()
        //         .join(";");
        //     println!("{}", line);
        // }
        // println!("{:?}", (dependencies.get(&label), &dependencies));
    }

    let mut king_pins = dependencies
        .values()
        .filter_map(|underneath| {
            (underneath.len() == 1).then(|| *underneath.iter().next().unwrap())
        })
        .collect::<HashSet<_>>();

    king_pins.remove(&0); // The ground doesn't count
    bricks.len() - king_pins.len()
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day22.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::part1;

    #[test]
    fn part1_given_example() {
        let input = indoc! {"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9"};
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn part1_given_example_shuffled() {
        let input = indoc! {"
        2,0,5~2,2,5
        0,2,3~2,2,3
        1,1,8~1,1,9
        0,0,2~2,0,2
        0,1,6~2,1,6
        0,0,4~0,2,4
        1,0,1~1,2,1"};
        assert_eq!(part1(input), 5);
    }

    // https://www.reddit.com/r/adventofcode/comments/18oboe8/2023_day_22_part_1/
    #[test]
    fn part1_community_example_1() {
        let input = indoc! {"
        0,0,1~0,1,1
        1,1,1~1,1,1
        0,0,2~0,0,2
        0,1,2~1,1,2"};
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn part1_community_example_2() {
        let input = indoc! {"
        0,0,1~1,0,1
        0,1,1~0,1,2
        0,0,5~0,0,5
        0,0,4~0,1,4"};
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part1_my_example_t() {
        let input = indoc! {"
        1,1,1~1,1,2
        0,1,4~2,1,4"};
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn part1_my_example_short_t() {
        let input = indoc! {"
        1,1,1~1,1,1
        0,1,4~2,1,4"};
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn part1_my_example_stone_hedge() {
        let input = indoc! {"
        0,1,2~0,1,2
        2,1,2~2,1,2
        0,1,4~2,1,4"};
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn part1_my_example_check_held_together() {
        let input = indoc! {"
        0,0,1~0,0,1
        1,1,1~1,1,1
        0,0,2~0,1,2
        0,1,3~1,1,3"};
        assert_eq!(part1(input), 2);
    }
}
