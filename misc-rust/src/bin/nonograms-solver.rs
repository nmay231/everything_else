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

struct Nonograms {
    // TODO: Assume square grid
    size: usize,
    // grid: Vec<GridCell>,
    grid: Vec<Vec<GridCell>>,
    row_clues: Vec<Vec<u8>>,
    col_clues: Vec<Vec<u8>>,
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
        }
    }

    // TODO: remove print statements and instead return a puzzle difficulty
    // summary object or something.
    pub fn solve(&mut self) {
        let settled = self.fill_simple_overlap();
        println!("There were {} cells settled by simple overlap", settled);
        self.debug_print();
    }

    pub fn fill_simple_overlap(&mut self) -> usize {
        let mut settled = 0;
        for (row, clues) in self.row_clues.iter().enumerate() {
            Self::_fill_simple_overlap(self.size, clues, |index, replacement| match replacement {
                None => Some(self.grid[row][index]),
                Some(replace) => {
                    self.grid[row][index] = replace;
                    settled += 1;
                    None
                }
            });
        }
        for (col, clues) in self.col_clues.iter().enumerate() {
            Self::_fill_simple_overlap(self.size, clues, |index, replacement| match replacement {
                None => Some(self.grid[index][col]),
                Some(replace) => {
                    self.grid[index][col] = replace;
                    settled += 1;
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
        let sum = clues.iter().sum::<u8>() as usize;
        let leeway = size
            .checked_sub(sum + clues.len() - 1)
            .expect("clues were too large for the grid size");

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
