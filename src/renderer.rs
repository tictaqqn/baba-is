use crate::board::Board;

pub trait Renderer {
    fn render<T: Renderer>(&self, board: &Board<T>);
    fn render_win<T: Renderer>(&self, board: &Board<T>);
    fn render_defeat<T: Renderer>(&self, board: &Board<T>);
}
