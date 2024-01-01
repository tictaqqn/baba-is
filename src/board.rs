use std::collections::HashMap;

use crate::global::{Direction, IS_DEFEAT, IS_PUSH, IS_STOP, IS_WIN, IS_YOU};

pub struct Board {
    map: HashMap<[i32; 2], Vec<usize>>,
}

impl Board {
    pub fn new(map: HashMap<[i32; 2], Vec<usize>>) -> Self {
        Self { map }
    }

    fn move_entity(&mut self, xy: [i32; 2], entity: usize, direction: Direction) -> bool {
        let [x, y] = xy;
        let next_move = direction.next([x, y]);
        let next_move_entities = self.map.get(&next_move).cloned();
        if let Some(next_move_entities) = next_move_entities {
            for y in next_move_entities {
                if IS_WIN[y] {
                    println!("You win!");
                    std::process::exit(0);
                }
                if IS_DEFEAT[y] {
                    println!("You lose!");
                    std::process::exit(0);
                }
                if IS_STOP[y] {
                    return false;
                }
                if IS_PUSH[y] {
                    let success = self.move_entity(next_move, entity, direction);
                    if !success {
                        return false;
                    }
                }
            }
        }
        self.map.get_mut(&xy).unwrap().retain(|&x| x != entity);
        self.map.entry(next_move).or_default().push(entity);
        true
    }

    pub fn player_input(&mut self, input: Direction) {
        // clones here because entities of map changes while iteration
        for (xy, xs) in self.map.clone().into_iter() {
            for x in xs {
                if IS_YOU[x] {
                    self.move_entity(xy, x, input);
                }
            }
        }
        // TODO: parse board and update IS_YOU, IS_PUSH, IS_STOP, IS_WIN, IS_DEFEAT
    }
}
