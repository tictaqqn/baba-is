use crate::global::*;

pub trait Entity {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn is_you(&self) -> bool;
    fn is_push(&self) -> bool;
    fn is_stop(&self) -> bool;
    fn is_win(&self) -> bool;
    fn is_defeat(&self) -> bool;
    fn go_right(&mut self);
    fn go_left(&mut self);
    fn go_up(&mut self);
    fn go_down(&mut self);
    fn player_input(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.go_up(),
            Direction::Down => self.go_down(),
            Direction::Left => self.go_left(),
            Direction::Right => self.go_right(),
        }
    }
}

pub trait Block: Entity {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn is_you(&self) -> bool {
        false
    }
    fn is_push(&self) -> bool {
        true
    }
    fn is_stop(&self) -> bool {
        false
    }
    fn is_win(&self) -> bool {
        false
    }
    fn is_defeat(&self) -> bool {
        false
    }
}

macro_rules! entity {
    ($name:ident, $index_name:ident) => {
        #[derive(Debug)]
        pub struct $name {
            _x: i32,
            _y: i32,
        }

        impl Entity for $name {
            fn x(&self) -> i32 {
                self._x
            }
            fn y(&self) -> i32 {
                self._y
            }
            fn go_right(&mut self) {
                self._x += 1;
            }
            fn go_left(&mut self) {
                self._x -= 1;
            }
            fn go_up(&mut self) {
                self._y -= 1;
            }
            fn go_down(&mut self) {
                self._y += 1;
            }
            fn is_you(&self) -> bool {
                IS_YOU[$index_name]
            }
            fn is_push(&self) -> bool {
                IS_PUSH[$index_name]
            }
            fn is_stop(&self) -> bool {
                IS_STOP[$index_name]
            }
            fn is_win(&self) -> bool {
                IS_WIN[$index_name]
            }
            fn is_defeat(&self) -> bool {
                IS_DEFEAT[$index_name]
            }
        }
    };
}

entity!(Baba, BABA);
entity!(Flag, FLAG);
entity!(Wall, WALL);
entity!(Rock, ROCK);
