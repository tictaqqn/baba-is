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
