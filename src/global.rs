pub const BABA: usize = 0;
pub const FLAG: usize = 1;
pub const WALL: usize = 2;
pub const ROCK: usize = 3;
pub const WATER: usize = 4;
pub const LAVA: usize = 5;
pub const SKULL: usize = 6;

pub const IS: usize = 100;

pub const YOU: usize = 200;
pub const PUSH: usize = 201;
pub const STOP: usize = 202;
pub const WIN: usize = 203;
pub const DEFEAT: usize = 204;

pub const BABA_B: usize = 300;
pub const FLAG_B: usize = 301;
pub const WALL_B: usize = 302;
pub const ROCK_B: usize = 303;
pub const WATER_B: usize = 304;
pub const LAVA_B: usize = 305;
pub const SKULL_B: usize = 306;

#[inline]
pub fn is_subject(entity: usize) -> bool {
    entity >= 300
}

#[inline]
pub fn is_verb(entity: usize) -> bool {
    (100..200).contains(&entity)
}

#[inline]
pub fn is_object(entity: usize) -> bool {
    (200..300).contains(&entity)
}

pub struct IsState {
    pub is_you: [bool; 308],
    pub is_push: [bool; 308],
    pub is_stop: [bool; 308],
    pub is_win: [bool; 308],
    pub is_defeat: [bool; 308],
}

impl Default for IsState {
    fn default() -> Self {
        fn create_array() -> [bool; 308] {
            let mut array = [true; 308];
            (0..100).for_each(|i| {
                array[i] = false;
            });
            array
        }
        Self {
            is_you: create_array(),
            is_push: create_array(),
            is_stop: create_array(),
            is_win: create_array(),
            is_defeat: create_array(),
        }
    }
}

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
