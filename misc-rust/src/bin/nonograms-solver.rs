use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GridCell {
    Unknown,
    Settled(bool),
}

impl From<GridCell> for char {
    fn from(value: GridCell) -> Self {
        match value {
            GridCell::Unknown => ' ',
            GridCell::Settled(true) => '■',
            GridCell::Settled(false) => 'X',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum RowOrCol {
    Row(usize),
    Col(usize),
}

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
        match (cell, existing) {
            (GridCell::Settled(_), GridCell::Unknown) => true,
            (GridCell::Settled(will_be_shaded), GridCell::Settled(shaded))
                if will_be_shaded == shaded =>
            {
                false
            }

            _ => unreachable!("Could not replace grid cell {:?} with {:?}", existing, cell),
        }
    }

    // TODO: remove print statements and instead return a puzzle difficulty
    // summary object or something.
    pub fn solve(&mut self) {
        let settled = self.fill_simple_overlap();
        println!("There were {} cells settled by simple overlap", settled);
        assert!(settled == 0 || (self.unchecked.len() > 0 && self.unchecked.len() <= settled));
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
                assert_ne!(cell, GridCell::Settled(false));
                assert_eq!(None, get_or_set(index, Some(GridCell::Settled(true))))
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

            let mut start = 0;
            for (clue_index, clue) in clues.iter().enumerate() {
                let clue = *clue as usize;
                while start < line.len() && line[start] == GridCell::Settled(false) {
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
                            |(index, cell)| (cell == &GridCell::Settled(false)).then_some(index),
                        )
                    {
                        let empty = GridCell::Settled(false);
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

            if line[clues[0] as usize] == GridCell::Settled(true) {
                let empty = GridCell::Settled(false);
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

    pub fn apply_rock_slide(holes: &[u8], rocks: &[u8]) -> Vec<HashSet<usize>> {
        assert!(
            holes.len() > 1,
            "No point in knowing which hole the rocks fall in if there is only one hole"
        );

        let leftmost = Self::_one_direction(holes.iter(), rocks.iter());
        let rightmost = Self::_one_direction(holes.iter().rev(), rocks.iter().rev());

        assert_eq!(leftmost.len(), rightmost.len());
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

fn main() {
    // Source: A manual transcription of Nonogram Galaxies puzzle #1-318
    let mut grid = Nonograms::new(
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
    );

    grid.solve();
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::Nonograms;

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
    fn test_one_direction(
        #[case] holes: Vec<u8>,
        #[case] rocks: Vec<u8>,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        assert_eq!(
            Nonograms::_one_direction(holes.iter(), rocks.iter()),
            expected
                .into_iter()
                .map(HashSet::<usize>::from_iter)
                .collect::<Vec<_>>()
        );
    }
}
