use std::collections::HashMap;

use crate::{
    global::{is_object, is_subject, is_verb, Direction, IsState, DEFEAT, PUSH, STOP, WIN, YOU},
    renderer::Renderer,
};

pub struct Board<R: Renderer> {
    map: HashMap<[i32; 2], Vec<usize>>,
    is_state: IsState,
    renderer: R,
}

impl<R: Renderer> Board<R> {
    pub fn new(map: HashMap<[i32; 2], Vec<usize>>, renderer: R) -> Self {
        Self {
            map,
            is_state: IsState::default(),
            renderer,
        }
    }

    fn move_entity(&mut self, ij: [i32; 2], entity: usize, direction: Direction) -> bool {
        let [i, j] = ij;
        let next_move = direction.next([i, j]);
        let next_move_entities = self.map.get(&next_move).cloned();
        if let Some(next_move_entities) = next_move_entities {
            for y in next_move_entities {
                if self.is_state.is_win[y] && self.is_state.is_you[entity] {
                    self.renderer.render_win(self);
                    std::process::exit(0);
                }
                if self.is_state.is_defeat[y] && self.is_state.is_you[entity] {
                    self.renderer.render_defeat(self);
                    std::process::exit(0);
                }
                if self.is_state.is_defeat[y] && !self.is_state.is_you[entity] {
                    self.map.get_mut(&next_move).unwrap().retain(|&x| x != y);
                    return true;
                }
                if self.is_state.is_stop[y] {
                    return false;
                }
                if self.is_state.is_push[y] {
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
                if !is_subject(x) {
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
                            if !is_verb(y) {
                                continue;
                            }
                            for &z in next_next_move_entities {
                                if !is_object(z) {
                                    continue;
                                }
                                if z == YOU {
                                    is_state.is_you[x] = true;
                                } else if z == PUSH {
                                    is_state.is_push[x] = true;
                                } else if z == STOP {
                                    is_state.is_stop[x] = true;
                                } else if z == WIN {
                                    is_state.is_win[x] = true;
                                } else if z == DEFEAT {
                                    is_state.is_defeat[x] = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn player_input(&mut self, input: Direction) {
        // clones here because entities of map changes while iteration
        for (ij, xs) in self.map.clone().into_iter() {
            for x in xs {
                if self.is_state.is_you[x] {
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
