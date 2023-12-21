/// (row, column)
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct UsizePoint(pub usize, pub usize);

impl UsizePoint {
    pub fn next_point(&self, direc: &Direc, grid_size: &UsizePoint) -> Option<UsizePoint> {
        match direc {
            Direc::North => {
                if self.0 > 0 {
                    Some(UsizePoint(self.0 - 1, self.1))
                } else {
                    None
                }
            }
            Direc::East => {
                if self.1 + 1 < grid_size.1 {
                    Some(UsizePoint(self.0, self.1 + 1))
                } else {
                    None
                }
            }
            Direc::South => {
                if self.0 + 1 < grid_size.0 {
                    Some(UsizePoint(self.0 + 1, self.1))
                } else {
                    None
                }
            }
            Direc::West => {
                if self.1 > 0 {
                    Some(UsizePoint(self.0, self.1 - 1))
                } else {
                    None
                }
            }
        }
    }

    #[inline(always)]
    pub fn as_index(&self, grid_size: &UsizePoint) -> usize {
        grid_size.1 * self.0 + self.1
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direc {
    North,
    East,
    South,
    West,
}

impl Direc {
    const POWERS_OF_I: [Direc; 4] = [Direc::East, Direc::North, Direc::West, Direc::South];

    pub fn rotate(&self, rotation_counter_clockwise: i32) -> Self {
        let current_index = Direc::POWERS_OF_I
            .iter()
            .enumerate()
            .find_map(|(i, x)| if x == self { Some(i as i32) } else { None })
            .unwrap();
        return Direc::POWERS_OF_I
            [(rotation_counter_clockwise + current_index).rem_euclid(4) as usize];
    }
}
