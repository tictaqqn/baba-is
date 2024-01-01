pub const BABA: usize = 0;
pub const FLAG: usize = 1;
pub const WALL: usize = 2;
pub const ROCK: usize = 3;
pub const WATER: usize = 4;
pub const LAVA: usize = 5;
pub const SKULL: usize = 6;

pub static IS_YOU: [bool; 8] = [false, false, false, false, false, false, false, false];
pub static IS_PUSH: [bool; 8] = [false, false, false, false, false, false, false, false];
pub static IS_STOP: [bool; 8] = [false, false, false, false, false, false, false, false];
pub static IS_WIN: [bool; 8] = [false, false, false, false, false, false, false, false];
pub static IS_DEFEAT: [bool; 8] = [false, false, false, false, false, false, false, false];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next(self, xy: [i32; 2]) -> [i32; 2] {
        let [x, y] = xy;
        match self {
            Direction::Up => [x - 1, y],
            Direction::Down => [x + 1, y],
            Direction::Left => [x, y - 1],
            Direction::Right => [x, y + 1],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::global::Direction;

    #[test]
    fn test_direction() {
        assert_eq!(Direction::Up.next([0, 0]), [-1, 0]);
        assert_eq!(Direction::Down.next([0, 0]), [1, 0]);
        assert_eq!(Direction::Left.next([0, 0]), [0, -1]);
        assert_eq!(Direction::Right.next([0, 0]), [0, 1]);
    }
}
