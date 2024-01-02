use std::collections::HashMap;

use global::Entity;

pub mod board;
pub mod global;
pub mod interactor;
pub mod renderer;

fn main() {
    let mut map = HashMap::new();
    map.insert([0, 0], vec![Entity::BabaB]);
    map.insert([0, 1], vec![Entity::Is]);
    map.insert([0, 2], vec![Entity::You]);
    map.insert([1, 0], vec![Entity::Baba]);
    let mut interactor =
        interactor::Interactor::new(board::Board::new(map, renderer::CuiRenderer));
    interactor.run();
}
