use std::collections::HashMap;

use crate::{
    global::{Direction, Entity, IsState},
    renderer::Renderer,
};

pub struct Board<R: Renderer> {
    pub map: HashMap<[i32; 2], Vec<Entity>>,
    is_state: IsState,
    pub renderer: R,
}

impl<R: Renderer> Board<R> {
    pub fn new(map: HashMap<[i32; 2], Vec<Entity>>, renderer: R) -> Self {
        Self {
            map,
            is_state: IsState::default(),
            renderer,
        }
    }

    fn move_entity(&mut self, ij: [i32; 2], entity: Entity, direction: Direction) -> bool {
        let [i, j] = ij;
        let next_move = direction.next([i, j]);
        let next_move_entities = self.map.get(&next_move).cloned();
        if let Some(next_move_entities) = next_move_entities {
            for y in next_move_entities {
                if self.is_state.is_win[y as usize] && self.is_state.is_you[entity as usize] {
                    self.renderer.render_win(self);
                    std::process::exit(0);
                }
                if self.is_state.is_defeat[y as usize] && self.is_state.is_you[entity as usize] {
                    self.renderer.render_defeat(self);
                    std::process::exit(0);
                }
                if self.is_state.is_defeat[y as usize] && !self.is_state.is_you[entity as usize] {
                    self.map.get_mut(&next_move).unwrap().retain(|&x| x != y);
                    return true;
                }
                if self.is_state.is_stop[y as usize] {
                    return false;
                }
                if self.is_state.is_push[y as usize] {
                    let success = self.move_entity(next_move, entity, direction);
                    if !success {
                        return false;
                    }
                }
            }
        }
        self.map.get_mut(&ij).unwrap().retain(|&x| x != entity);
        self.map.entry(next_move).or_default().push(entity);
        true
    }

    fn parse_box(&self, is_state: &mut IsState, ij: [i32; 2], direction: Direction) {
        let entities = self.map.get(&ij);
        if let Some(entities) = entities {
            for &x in entities {
                if !x.is_subject() {
                    continue;
                }
                let [i, j] = ij;
                let next_move = direction.next([i, j]);
                let next_move_entities = self.map.get(&next_move);
                if let Some(next_move_entities) = next_move_entities {
                    let next_next_move = direction.next(next_move);
                    let next_next_move_entities = self.map.get(&next_next_move);
                    if let Some(next_next_move_entities) = next_next_move_entities {
                        for &y in next_move_entities {
                            if !y.is_verb() {
                                continue;
                            }
                            for &z in next_next_move_entities {
                                if !z.is_object() {
                                    continue;
                                }
                                if z == Entity::You {
                                    is_state.is_you[x as usize] = true;
                                } else if z == Entity::Push {
                                    is_state.is_push[x as usize] = true;
                                } else if z == Entity::Stop {
                                    is_state.is_stop[x as usize] = true;
                                } else if z == Entity::Win {
                                    is_state.is_win[x as usize] = true;
                                } else if z == Entity::Defeat {
                                    is_state.is_defeat[x as usize] = true;
                                } else {
                                    unreachable!("Unknown object: {:?}", z);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn react_with_input(&mut self, input: Direction) {
        // clones here because entities of map changes while iteration
        for (ij, xs) in self.map.clone().into_iter() {
            for x in xs {
                if self.is_state.is_you[x as usize] {
                    self.move_entity(ij, x, input);
                }
            }
        }
        let mut is_state = IsState::default();
        for (&ij, _) in self.map.iter() {
            for direction in [Direction::Down, Direction::Right] {
                self.parse_box(&mut is_state, ij, direction);
            }
        }
        self.is_state = is_state;
        self.renderer.render(self);
    }
}
