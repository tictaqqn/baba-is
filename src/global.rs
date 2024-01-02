#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Entity {
    Baba,
    Flag,
    Wall,
    Rock,
    Water,
    Lava,
    Skull,

    Is,

    You,
    Push,
    Stop,
    Win,
    Defeat,

    BabaB,
    FlagB,
    WallB,
    RockB,
    WaterB,
    LavaB,
    SkullB,
}

const ENTITY_COUNT: usize = 30;

impl Entity {
    #[inline]
    pub fn is_subject(self) -> bool {
        self as usize >= Self::BabaB as usize
    }

    #[inline]
    pub fn is_verb(self) -> bool {
        (Self::Is as usize..Self::You as usize).contains(&(self as usize))
    }

    #[inline]
    pub fn is_object(self) -> bool {
        (Self::You as usize..Self::BabaB as usize).contains(&(self as usize))
    }
}

pub struct IsState {
    pub is_you: [bool; ENTITY_COUNT],
    pub is_push: [bool; ENTITY_COUNT],
    pub is_stop: [bool; ENTITY_COUNT],
    pub is_win: [bool; ENTITY_COUNT],
    pub is_defeat: [bool; ENTITY_COUNT],
}

impl Default for IsState {
    fn default() -> Self {
        fn create_array() -> [bool; ENTITY_COUNT] {
            let mut array = [true; ENTITY_COUNT];
            (0..Entity::Is as usize).for_each(|i| {
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
    use crate::global::*;

    #[test]
    fn test_is_subject() {
        use Entity::*;
        assert!(BabaB.is_subject());
        assert!(FlagB.is_subject());
        assert!(WallB.is_subject());
        assert!(RockB.is_subject());
        assert!(WaterB.is_subject());
        assert!(LavaB.is_subject());
        assert!(SkullB.is_subject());
        assert!(!Baba.is_subject());
        assert!(!Flag.is_subject());
        assert!(!Wall.is_subject());
        assert!(!Rock.is_subject());
        assert!(!Water.is_subject());
        assert!(!Lava.is_subject());
        assert!(!Skull.is_subject());
        assert!(!Is.is_subject());
        assert!(!You.is_subject());
        assert!(!Push.is_subject());
        assert!(!Stop.is_subject());
        assert!(!Win.is_subject());
        assert!(!Defeat.is_subject());
    }

    #[test]
    fn test_is_verb() {
        use Entity::*;
        assert!(Is.is_verb());
        assert!(!You.is_verb());
        assert!(!Push.is_verb());
        assert!(!Stop.is_verb());
        assert!(!Win.is_verb());
        assert!(!Defeat.is_verb());
        assert!(!BabaB.is_verb());
        assert!(!FlagB.is_verb());
        assert!(!WallB.is_verb());
        assert!(!RockB.is_verb());
        assert!(!WaterB.is_verb());
        assert!(!LavaB.is_verb());
        assert!(!SkullB.is_verb());
        assert!(!Baba.is_verb());
        assert!(!Flag.is_verb());
        assert!(!Wall.is_verb());
        assert!(!Rock.is_verb());
        assert!(!Water.is_verb());
        assert!(!Lava.is_verb());
        assert!(!Skull.is_verb());
    }

    #[test]
    fn test_is_object() {
        use Entity::*;
        assert!(!BabaB.is_object());
        assert!(!FlagB.is_object());
        assert!(!WallB.is_object());
        assert!(!RockB.is_object());
        assert!(!WaterB.is_object());
        assert!(!LavaB.is_object());
        assert!(!SkullB.is_object());
        assert!(!Baba.is_object());
        assert!(!Flag.is_object());
        assert!(!Wall.is_object());
        assert!(!Rock.is_object());
        assert!(!Water.is_object());
        assert!(!Lava.is_object());
        assert!(!Skull.is_object());
        assert!(!Is.is_object());
        assert!(You.is_object());
        assert!(Push.is_object());
        assert!(Stop.is_object());
        assert!(Win.is_object());
        assert!(Defeat.is_object());
    }

    #[test]
    fn test_direction() {
        assert_eq!(Direction::Up.next([0, 0]), [-1, 0]);
        assert_eq!(Direction::Down.next([0, 0]), [1, 0]);
        assert_eq!(Direction::Left.next([0, 0]), [0, -1]);
        assert_eq!(Direction::Right.next([0, 0]), [0, 1]);
    }
}
