#[allow(deprecated)]
use crate::{Point, UsizePoint};

use super::point::MyNumber;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direc {
    North,
    East,
    South,
    West,
}

impl Direc {
    pub const POWERS_OF_I: [Direc; 4] = [Direc::East, Direc::North, Direc::West, Direc::South];
    /// Following the rotation of polar coordinates
    pub const EIGHT_WAYS: [&[Direc]; 8] = [
        &[Direc::East],
        &[Direc::East, Direc::North],
        &[Direc::North],
        &[Direc::West, Direc::North],
        &[Direc::West],
        &[Direc::West, Direc::South],
        &[Direc::South],
        &[Direc::East, Direc::South],
    ];

    pub fn rotate(&self, rotation_counter_clockwise: i32) -> Self {
        let current_index = Direc::POWERS_OF_I
            .iter()
            .enumerate()
            .find_map(|(i, x)| if x == self { Some(i as i32) } else { None })
            .unwrap();
        return Direc::POWERS_OF_I
            [(rotation_counter_clockwise + current_index).rem_euclid(4) as usize];
    }

    #[inline]
    pub fn cmp_points<T: MyNumber>(&self, a: &Point<T>, b: &Point<T>) -> std::cmp::Ordering {
        match self {
            Direc::South => a.y.cmp(&b.y),
            Direc::North => b.y.cmp(&a.y),
            Direc::East => a.x.cmp(&b.x),
            Direc::West => b.x.cmp(&a.x),
        }
    }

    #[inline]
    pub fn to_power_of_i(&self) -> usize {
        match self {
            Direc::East => 0,
            Direc::North => 1,
            Direc::West => 2,
            Direc::South => 3,
        }
    }

    #[inline]
    #[deprecated]
    #[allow(deprecated)]
    pub fn cmp_points_old(&self, a: &UsizePoint, b: &UsizePoint) -> std::cmp::Ordering {
        match self {
            Direc::South => a.0.cmp(&b.0),
            Direc::North => b.0.cmp(&a.0),
            Direc::East => a.1.cmp(&b.1),
            Direc::West => b.1.cmp(&a.1),
        }
    }
}
