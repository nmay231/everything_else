/// The point of this solver is not to find the solution, but to help me solve
/// it by checking the difficulty by using human solving techniques (rather than
/// brute force enumeration) and ranking it by how sparse the results are per
/// iteration step. At least, that's the purpose I have at the moment.
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GridCell {
    Unknown,
    // Settled(bool),
    Shaded,
    Unshaded,
}

impl GridCell {
    /// Returns None if Unknown
    pub fn is_shaded(&self) -> Option<bool> {
        match self {
            GridCell::Unknown => None,
            GridCell::Shaded => Some(true),
            GridCell::Unshaded => Some(false),
        }
    }
}

impl From<GridCell> for char {
    fn from(value: GridCell) -> Self {
        match value {
            GridCell::Unknown => ' ',
            GridCell::Shaded => '■',
            GridCell::Unshaded => 'X',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum RowOrCol {
    Row(usize),
    Col(usize),
}

#[derive(Debug, Clone)]
struct Nonograms {
    // TODO: Assume square grid
    size: usize,
    // grid: Vec<GridCell>,
    grid: Vec<Vec<GridCell>>,
    row_clues: Vec<Vec<u8>>,
    col_clues: Vec<Vec<u8>>,
    unchecked: HashSet<RowOrCol>,
}

impl Nonograms {
    fn new(size: usize, row_clues: Vec<Vec<u8>>, col_clues: Vec<Vec<u8>>) -> Self {
        assert_eq!(size, row_clues.len());
        assert_eq!(size, col_clues.len());

        Self {
            size,
            grid: vec![vec![GridCell::Unknown; size]; size],
            row_clues,
            col_clues,
            unchecked: HashSet::new(),
        }
    }

    /// Would the new cell value increase the number of settled cells? Panics if
    /// new cell value isn't a settled value or differs from the existing
    /// settled value
    pub fn is_new_settled(existing: &GridCell, cell: &GridCell) -> bool {
        match (cell.is_shaded(), existing.is_shaded()) {
            (Some(_), None) => true,
            (Some(will_be_shaded), Some(shaded)) if will_be_shaded == shaded => false,

            _ => unreachable!("Could not replace grid cell {:?} with {:?}", existing, cell),
        }
    }

    // TODO: remove print statements and instead return a puzzle difficulty
    // summary object or something.
    pub fn solve(&mut self) {
        let settled = self.fill_simple_overlap();
        println!("There were {} cells settled by simple overlap", settled);
        assert!(settled == 0 || (self.unchecked.len() > 0 && self.unchecked.len() <= settled));

        let settled = self.apply_identities();
        println!("New settled{}", settled);

        self.debug_print();
    }

    /// At the start of a puzzle, there are cells that cannot be unshaded
    /// without making that row/column of clues impossible, e.g. an 18 clue in a
    /// 30x30 puzzle fills the middle 6 cells.
    pub fn fill_simple_overlap(&mut self) -> usize {
        let mut settled = 0;
        for (row, clues) in self.row_clues.iter().enumerate() {
            Self::_fill_simple_overlap(self.size, clues, |index, replacement| match replacement {
                None => Some(self.grid[row][index]),
                Some(replace) => {
                    assert!(Self::is_new_settled(&self.grid[row][index], &replace));
                    settled += 1;
                    self.grid[row][index] = replace;
                    self.unchecked.insert(RowOrCol::Col(index));
                    None
                }
            });
        }
        for (col, clues) in self.col_clues.iter().enumerate() {
            Self::_fill_simple_overlap(self.size, clues, |index, replacement| match replacement {
                None => Some(self.grid[index][col]),
                Some(replace) => {
                    assert!(Self::is_new_settled(&self.grid[index][col], &replace));
                    settled += 1;
                    self.grid[index][col] = replace;
                    self.unchecked.insert(RowOrCol::Row(index));
                    None
                }
            });
        }
        return settled;
    }

    fn _fill_simple_overlap<'a: 'b, 'b>(
        size: usize,
        clues: &'a Vec<u8>,
        mut get_or_set: impl FnMut(usize, Option<GridCell>) -> Option<GridCell>,
    ) {
        let leeway = Self::calc_leeway(clues, size);

        let mut start = 0;
        for clue in clues {
            let clue = *clue as usize;
            for index in start + leeway..start + clue {
                let cell =
                    get_or_set(index, None).expect("should've return something when passed None");
                assert_ne!(cell, GridCell::Unshaded);
                assert_eq!(None, get_or_set(index, Some(GridCell::Shaded)))
            }

            start += clue + 1;
        }
    }

    fn calc_leeway(clues: &Vec<u8>, size: usize) -> usize {
        let sum = clues.iter().sum::<u8>() as usize;
        return size
            .checked_sub(sum + clues.len() - 1)
            .expect("clues were too large for the grid size");
    }

    pub fn apply_identities(&mut self) -> usize {
        let mut settled = 0;
        let mut next_round = HashSet::new();
        for row_or_col in self.unchecked.drain() {
            // TODO: Do I want to use sentinel values for this?
            let cell_unknown = self.size;
            let cell_known_empty = self.size + 1;
            // Vec<clue_index_of_filled_cell | unknown_identity | known_empty_cell>
            let mut identities = vec![cell_unknown; self.size];

            let clues = match row_or_col {
                RowOrCol::Row(row) => self.row_clues[row].clone(),
                RowOrCol::Col(col) => self.col_clues[col].clone(),
            };
            let leeway = Self::calc_leeway(&clues, self.size);
            if leeway == 0 {
                continue; // Line already solved
            }

            // TODO: I should really provide a way to reference the original
            // data rather than copying it. I could either switch to a 1D array
            // or always work on the grid row-wise and just transpose every time
            // I switch.
            let mut line = (0..self.size)
                .map(|i| match row_or_col {
                    RowOrCol::Row(row) => self.grid[row][i],
                    RowOrCol::Col(col) => self.grid[i][col],
                })
                .collect::<Vec<_>>();
            let grid = &mut self.grid;
            let mut set_index: Box<dyn FnMut(usize, GridCell)> = match row_or_col {
                RowOrCol::Row(row) => Box::new(move |i: usize, cell: GridCell| grid[row][i] = cell),
                RowOrCol::Col(col) => Box::new(move |i: usize, cell: GridCell| grid[i][col] = cell),
            };

            let Some((first_unknown_cell, _)) = line
                .iter()
                .find_position(|cell| cell == &&GridCell::Unknown)
            else {
                continue;
            };

            let mut start = 0;
            for (clue_index, clue) in clues.iter().enumerate() {
                let clue = *clue as usize;

                let mut shaded_length = 0;
                for known_index in 0..first_unknown_cell {
                    start = known_index;
                    match (shaded_length, line[known_index]) {
                        (_, GridCell::Unknown) => {
                            unreachable!("We should know the index of the first unknown cell")
                        }
                        (_, GridCell::Shaded) => shaded_length += 1,
                        (0, GridCell::Unshaded) => {}
                        (1.., GridCell::Unshaded) => break,
                    }
                }

                let empty = GridCell::Unshaded;
                if shaded_length > 0 {
                    if Self::is_new_settled(&line[start], &empty) {
                        identities[start] = cell_known_empty;
                        set_index(start, empty);
                    }
                }

                while start < line.len() && line[start] == GridCell::Unshaded {
                    identities[start] = cell_known_empty;
                    start += 1;
                }
                assert!(
                    start < self.size,
                    "Too many clues for the line: {:?}",
                    (&line, &clues)
                );

                if clue_index == 0 {
                    // Example for a clue of 4: [unknown unknown x ...] => [x x x ...]
                    while let Some(rightmost_blocker) =
                        line[start..start + clue].iter().enumerate().rev().find_map(
                            |(index, cell)| (cell == &GridCell::Unshaded).then_some(index),
                        )
                    {
                        let empty = GridCell::Unshaded;
                        for set_to_empty in start..rightmost_blocker {
                            if Self::is_new_settled(&line[set_to_empty], &empty) {
                                settled += 1;
                                set_index(set_to_empty, empty);
                            }
                            identities[set_to_empty] = cell_known_empty;
                        }
                        start = rightmost_blocker + 1;
                        assert!(start < self.size);
                    }
                }

                // TODO: Perhaps I should actually change this to use an array
                // of slots and clues and work with numbers exclusively at first.
            }

            if line[clues[0] as usize] == GridCell::Shaded {
                let empty = GridCell::Unshaded;
                if Self::is_new_settled(&line[0], &empty) {
                    settled += 1;
                    line[0] = empty;
                }
            }

            next_round.insert(row_or_col);
        }
        self.unchecked = next_round;
        return settled;
    }

    pub fn get_holes(&self, at: RowOrCol) -> Vec<u8> {
        match at {
            RowOrCol::Row(row) => (0..self.size)
                .map(|i| self.grid[row][i])
                .group_by(|cell| cell == &GridCell::Unshaded)
                .into_iter()
                .filter_map(|(is_hole, group)| (!is_hole).then(|| group.count() as u8))
                .collect(),
            RowOrCol::Col(col) => (0..self.size)
                .map(|i| self.grid[i][col])
                .group_by(|cell| cell == &GridCell::Unshaded)
                .into_iter()
                .filter_map(|(is_hole, group)| (!is_hole).then(|| group.count() as u8))
                .collect(),
        }
    }

    // TODO: I kinda hate that everything is being dumped into this class. I
    // should move some of these methods into a module called holy_rocks (yes, I
    // did just do that. You're welcome).

    /// Revser
    pub(crate) fn reverse_and_invert(
        indices: Vec<HashSet<usize>>,
        x: usize,
    ) -> Vec<HashSet<usize>> {
        return indices
            .into_iter()
            .rev()
            .map(|indices| indices.into_iter().map(|index| x - index).collect())
            .collect();
    }

    /// By placing rocks (aka clues) from left to right and then right to left,
    /// some of the rocks end up in the same hole. This means that rock can only
    /// go in that hole, which can be useful if the hole is smaller than twice
    /// the length of the hole since that requires at least one shaded cell.
    pub fn hole_identities_intersection(holes: &[u8], rocks: &[u8]) -> Vec<HashSet<usize>> {
        assert!(
            holes.len() > 1,
            "No point in knowing which hole the rocks fall in if there is only one hole"
        );

        let leftmost = Self::_one_direction(holes.iter(), rocks.iter());
        let rightmost = Self::_one_direction(holes.iter().rev(), rocks.iter().rev());
        let rightmost = Self::reverse_and_invert(rightmost, holes.len() - 1);
        assert_eq!(leftmost.len(), rightmost.len());
        assert_eq!(leftmost.len(), holes.len());

        return leftmost
            .into_iter()
            .zip(rightmost)
            .map(|(left, right)| {
                left.intersection(&right)
                    .map(ToOwned::to_owned)
                    .collect::<HashSet<usize>>()
            })
            .collect();
    }

    /// As opposed to hole_identities_intersection(), this method returns the
    /// indices of *all* rocks that can possibly reside in each hole rather than
    /// the rocks that must reside in that hole.
    pub fn hole_identities_union(holes: &[u8], rocks: &[u8]) -> Vec<HashSet<usize>> {
        assert!(
            holes.len() > 1,
            "No point in knowing which hole the rocks fall in if there is only one hole"
        );
        let leftmost = Self::_one_direction(holes.iter(), rocks.iter());
        let rightmost = Self::_one_direction(holes.iter().rev(), rocks.iter().rev());
        let rightmost = Self::reverse_and_invert(rightmost, holes.len() - 1);
        assert_eq!(leftmost.len(), rightmost.len());
        assert_eq!(leftmost.len(), holes.len());

        let mut available = HashSet::<usize>::new();
        return holes
            .iter()
            .enumerate()
            .map(|(hole_i, hole)| {
                available.extend(&leftmost[hole_i]);

                let union = available
                    .iter()
                    .filter(|rock_i| rocks[**rock_i] <= *hole)
                    .map(ToOwned::to_owned)
                    .collect();

                rightmost[hole_i].iter().for_each(|out_of_date| {
                    assert!(
                        available.remove(&out_of_date),
                        "each rock should be accounted for exactly once"
                    )
                });

                return union;
            })
            .collect();
    }

    pub(crate) fn _one_direction<'a>(
        mut holes: impl Iterator<Item = &'a u8>,
        rocks: impl Iterator<Item = &'a u8>,
    ) -> Vec<HashSet<usize>> {
        let mut rocks = rocks.enumerate();
        let Some((mut rock_i, mut rock)) = rocks.next() else {
            return holes.map(|_| HashSet::new()).collect();
        };
        let mut alignments = vec![];
        let mut last_alignment = HashSet::new();

        'holes_loop: for hole in &mut holes {
            let mut hole = *hole;
            while hole >= *rock {
                last_alignment.insert(rock_i);

                if hole == *rock {
                    hole = 0;
                } else {
                    hole -= *rock + 1;
                }

                let Some(next_rock) = rocks.next() else {
                    break 'holes_loop;
                };
                (rock_i, rock) = next_rock;
            }

            alignments.push(last_alignment);
            last_alignment = HashSet::new();
        }

        assert_eq!(rocks.next(), None);
        alignments.push(last_alignment);
        alignments.extend(holes.map(|_| HashSet::new()));
        return alignments;
    }

    fn debug_print(&self) {
        for row in 0..=2 * self.size {
            let chars = match row {
                0 => "┏━┯┳┓",
                _ if row == 2 * self.size => "┗━┷┻┛",
                _ if row % 10 == 0 => "┣━┿╋┫",
                _ if row & 1 == 1 => "┃ │┃┃",
                _ if row & 1 == 0 => "┠─┼╂┨",
                _ => unreachable!(),
            };
            let chars = chars.chars().collect::<Vec<_>>();
            print!("{}", chars[0]);
            for col in 1..2 * self.size {
                let mut c = if col & 1 == 1 {
                    chars[1]
                } else if col % 10 == 0 {
                    chars[3]
                } else {
                    chars[2]
                };

                if c == ' ' {
                    c = self.grid[row / 2][col / 2].into();
                }
                print!("{}", c);
            }
            print!("{}", chars[4]);
            println!();
        }
    }
}

fn init_example_nonogram() -> Nonograms {
    // Source: A manual transcription of Nonogram Galaxies puzzle #1-318
    Nonograms::new(
        30,
        vec![
            vec![1, 2, 1, 2, 2, 2, 1, 3, 1, 1],
            vec![1, 1, 1, 1, 5, 1, 3, 2, 1],
            vec![1, 4, 2, 3, 3, 4, 1],
            vec![1, 8, 1, 1, 2, 4],
            vec![5, 2, 6, 1, 1],
            vec![1, 2, 1, 2, 2, 2, 2, 1],
            vec![1, 2, 2, 5, 1, 1, 1],
            vec![1, 5, 3, 2, 1, 3],
            vec![2, 1, 3, 1, 5, 3, 1],
            vec![2, 2, 1, 2, 2, 1, 3, 1],
            vec![4, 1, 6, 1, 1, 3],
            vec![1, 2, 2, 1, 2],
            vec![2, 1, 2, 2, 1, 1, 2],
            vec![1, 2, 2, 1, 1, 1, 1, 1, 3, 3],
            vec![5, 1, 1, 2, 1],
            vec![2, 3, 1, 1, 2, 1, 1, 2, 1],
            vec![2, 1, 1, 6, 1, 1, 1],
            vec![2, 2, 2, 2, 1],
            vec![1, 1, 6, 3, 2, 1],
            vec![1, 1, 2, 13, 2, 2],
            vec![1, 2, 1, 3, 2],
            vec![1, 3, 6, 6, 2],
            vec![1, 2, 1, 1, 1, 4, 1, 1, 2, 3],
            vec![1, 3, 2, 1, 1, 2, 1, 3, 2, 1],
            vec![1, 1, 1, 2, 1, 1, 2, 1, 2, 3],
            vec![3, 2, 5, 4, 3, 2, 2],
            vec![4, 7, 2, 1, 2, 2],
            vec![1, 5, 1, 1, 1, 4],
            vec![1, 1, 2, 2, 2, 7, 1],
            vec![1, 7, 2, 6, 2, 2, 1],
        ],
        vec![
            vec![1, 6, 3, 1, 2, 5, 1, 1],
            vec![1, 1, 1, 2, 1, 2, 2, 1],
            vec![1, 2, 2, 2, 5, 1],
            vec![1, 4, 1, 2, 7, 4],
            vec![7, 1, 3, 3, 3, 1],
            vec![2, 2, 2, 3, 2, 2, 1, 1],
            vec![2, 3, 3, 2, 1, 2, 1],
            vec![1, 1, 2, 2, 2, 1, 1],
            vec![1, 1, 1, 2, 1],
            vec![3, 2, 9, 6],
            vec![3, 3, 4, 5, 3, 1],
            vec![1, 1, 2, 1, 2, 1],
            vec![2, 2, 1, 2, 8],
            vec![3, 2, 2, 1, 2, 1, 1, 2],
            vec![3, 3, 1, 1, 1, 1, 2, 2, 1],
            vec![3, 3, 1, 2, 1, 2, 2, 2],
            vec![2, 3, 1, 2, 1, 5, 2],
            vec![2, 1, 1, 1, 1, 2, 2, 1],
            vec![5, 2, 1, 2, 1, 1, 2],
            vec![1, 1, 1, 2, 1, 2, 8],
            vec![2, 1, 1, 1, 2, 1, 1, 1],
            vec![2, 2, 5, 5, 2, 2],
            vec![1, 5, 7, 5, 3],
            vec![1, 2, 2, 1, 2, 1, 2, 1],
            vec![1, 2, 1, 6, 2, 4, 1],
            vec![3, 2, 2, 2, 2, 2, 1, 2],
            vec![5, 1, 2, 3, 2, 3, 3],
            vec![1, 2, 2, 4, 2, 2, 4],
            vec![1, 5, 1, 7, 1, 2, 1],
            vec![1, 2, 4, 1, 2, 2],
        ],
    )
}

fn main() {
    let mut grid = init_example_nonogram();

    grid.solve();
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{init_example_nonogram, GridCell, Nonograms, RowOrCol};

    #[rstest::rstest]
    #[case(vec![5, 6], vec![], vec![vec![], vec![]])]
    #[case(vec![5, 6], vec![4], vec![vec![0], vec![]])]
    #[case(vec![5, 6], vec![5], vec![vec![0], vec![]])]
    #[case(vec![5, 6], vec![6], vec![vec![], vec![0]])]
    #[case(vec![5, 6], vec![2, 2, 2], vec![vec![0, 1], vec![2]])]
    #[case(vec![5, 1], vec![1, 1, 1, 1], vec![vec![0, 1, 2], vec![3]])]
    #[case(vec![5, 1], vec![3, 1, 1], vec![vec![0, 1], vec![2]])]
    #[case(vec![5, 1], vec![1, 3, 1], vec![vec![0, 1], vec![2]])]
    #[case(vec![6, 1], vec![3, 1, 1], vec![vec![0, 1], vec![2]])]
    #[case(vec![6, 1], vec![1, 3, 1], vec![vec![0, 1], vec![2]])]
    #[case(vec![1, 2, 3, 4, 5], vec![5], vec![vec![], vec![], vec![], vec![], vec![0]])]
    #[case(vec![8, 8, 8], vec![1, 8, 1], vec![vec![0], vec![1], vec![2]])]
    #[case(vec![8, 8, 8], vec![1, 7, 1], vec![vec![0], vec![1], vec![2]])]
    #[case(vec![8, 8, 8], vec![1, 6, 1], vec![vec![0, 1], vec![2], vec![]])]
    fn test_one_direction(
        #[case] holes: Vec<u8>,
        #[case] rocks: Vec<u8>,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        assert_eq!(
            Nonograms::_one_direction(holes.iter(), rocks.iter()),
            expected
                .into_iter()
                .map(HashSet::from_iter)
                .collect::<Vec<_>>()
        );
    }

    #[rstest::rstest]
    #[case(vec![8, 8, 8], vec![1, 8, 1], vec![vec![0], vec![1], vec![2]])]
    #[case(vec![8, 8, 8], vec![1, 7, 1], vec![vec![0], vec![1], vec![2]])]
    #[case(vec![8, 8, 8], vec![1, 6, 1], vec![vec![], vec![], vec![]])]
    fn hole_intersection(
        #[case] holes: Vec<u8>,
        #[case] rocks: Vec<u8>,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        let actual = Nonograms::hole_identities_intersection(&holes, &rocks);
        assert_eq!(
            actual,
            expected
                .into_iter()
                .map(HashSet::from_iter)
                .collect::<Vec<_>>()
        )
    }

    #[rstest::rstest]
    #[case(vec![8, 8, 8], vec![1, 8, 1], vec![vec![0], vec![1], vec![2]])]
    #[case(vec![8, 8, 8], vec![1, 7, 1], vec![vec![0], vec![1], vec![2]])]
    #[case(vec![8, 8, 8], vec![1, 6, 1], vec![vec![0, 1], vec![0, 1, 2], vec![1, 2]])]
    fn hole_union(
        #[case] holes: Vec<u8>,
        #[case] rocks: Vec<u8>,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        let actual = Nonograms::hole_identities_union(&holes, &rocks);
        assert_eq!(
            actual,
            expected
                .into_iter()
                .map(HashSet::from_iter)
                .collect::<Vec<_>>()
        )
    }

    // TODO: This is not the best way to write this test (the initialization is
    // separate from the test, but whatever)
    #[rstest::fixture]
    #[once]
    fn get_holes_example_nonograms() -> Nonograms {
        // I don't actually need the clues for this test, so this counts as a
        // generic 30x30 grid.
        let mut puzzle = init_example_nonogram();

        let shaded = GridCell::Shaded;
        let empty = GridCell::Unshaded;

        // Row(0)
        puzzle.grid[0][5] = empty;
        puzzle.grid[0][7] = shaded;
        // Row(3)
        puzzle.grid[3][0] = empty;
        puzzle.grid[3][1] = empty;
        puzzle.grid[3][2] = empty;
        puzzle.grid[3][3] = empty;

        puzzle.grid[3][7] = empty;
        puzzle.grid[3][8] = empty;
        puzzle.grid[3][9] = empty;

        puzzle.grid[3][11] = empty;
        puzzle.grid[3][12] = shaded;
        puzzle.grid[3][13] = empty;

        puzzle.grid[3][16] = empty;

        puzzle.grid[3][27] = empty;
        puzzle.grid[3][28] = empty;
        puzzle.grid[3][29] = empty;
        // Col(6)
        puzzle.grid[1][6] = empty;
        puzzle.grid[28][6] = empty;
        // Row(4)
        puzzle.grid[4][0] = shaded;
        puzzle.grid[4][1] = empty;
        puzzle.grid[4][28] = empty;
        puzzle.grid[4][29] = shaded;

        return puzzle;
    }

    #[rstest::rstest]
    #[case(RowOrCol::Row(0), vec![5, 24])]
    #[case(RowOrCol::Row(3), vec![3, 1, 1, 2, 10])]
    #[case(RowOrCol::Col(6), vec![1, 26, 1])]
    #[case(RowOrCol::Row(4), vec![1, 26, 1])]
    fn get_holes(
        get_holes_example_nonograms: &Nonograms,
        #[case] row_or_col: RowOrCol,
        #[case] expected: Vec<u8>,
    ) {
        let puzzle = get_holes_example_nonograms;
        assert_eq!(puzzle.get_holes(row_or_col), expected);
    }
}
