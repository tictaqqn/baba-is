use crate::{
    board::Board,
    global::Direction,
    renderer::{CuiRenderer, Renderer},
};

pub struct Interactor<R: Renderer> {
    board: Board<R>,
}

impl Interactor<CuiRenderer> {
    pub fn new(board: Board<CuiRenderer>) -> Self {
        Self { board }
    }

    pub fn run(&mut self) {
        self.board.renderer.render(&self.board);
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let direction = match input {
                "w" => Direction::Up,
                "s" => Direction::Down,
                "a" => Direction::Left,
                "d" => Direction::Right,
                _ => continue,
            };
            self.board.react_with_input(direction);
        }
    }
}
