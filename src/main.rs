use std::collections::HashMap;

pub mod board;
pub mod global;
pub mod interactor;
pub mod renderer;

fn main() {
    let mut map = HashMap::new();
    map.insert([0, 0], vec![global::BABA_B]);
    map.insert([0, 1], vec![global::IS]);
    map.insert([0, 2], vec![global::YOU]);
    map.insert([1, 0], vec![global::BABA]);
    let mut interactor =
        interactor::Interactor::new(board::Board::new(map, renderer::CuiRenderer));
    interactor.run();
}
