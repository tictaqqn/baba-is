use crate::global::*;

pub trait Entity {
    fn is_you(&self) -> bool;
    fn is_push(&self) -> bool;
    fn is_stop(&self) -> bool;
    fn is_win(&self) -> bool;
    fn is_defeat(&self) -> bool;
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
        pub struct $name {}
        impl Entity for $name {
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
