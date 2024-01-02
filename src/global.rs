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

#[inline]
pub fn is_subject(entity: Entity) -> bool {
    entity as usize >= Entity::BabaB as usize
}

#[inline]
pub fn is_verb(entity: Entity) -> bool {
    (Entity::Is as usize..Entity::You as usize).contains(&(entity as usize))
}

#[inline]
pub fn is_object(entity: Entity) -> bool {
    (Entity::You as usize..Entity::BabaB as usize).contains(&(entity as usize))
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
    use crate::global::Direction;

    #[test]
    fn test_direction() {
        assert_eq!(Direction::Up.next([0, 0]), [-1, 0]);
        assert_eq!(Direction::Down.next([0, 0]), [1, 0]);
        assert_eq!(Direction::Left.next([0, 0]), [0, -1]);
        assert_eq!(Direction::Right.next([0, 0]), [0, 1]);
    }
}
