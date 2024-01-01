use crate::{
    board::Board,
    global::{DEFEAT, PUSH, STOP, WIN, YOU},
};

pub trait Renderer {
    fn render<T: Renderer>(&self, board: &Board<T>);
    fn render_win<T: Renderer>(&self, board: &Board<T>);
    fn render_defeat<T: Renderer>(&self, board: &Board<T>);
}

pub struct CuiRenderer;

impl Renderer for CuiRenderer {
    fn render<T: Renderer>(&self, board: &Board<T>) {
        let mut max_i = 0;
        let mut max_j = 0;
        for &[i, j] in board.map.keys() {
            max_i = max_i.max(i);
            max_j = max_j.max(j);
        }
        for i in 0..=max_i {
            for j in 0..=max_j {
                let entities = board.map.get(&[i, j]);
                if let Some(entities) = entities {
                    for &entity in entities {
                        if entity == YOU {
                            print!("Y");
                        } else if entity == WIN {
                            print!("W");
                        } else if entity == DEFEAT {
                            print!("D");
                        } else if entity == STOP {
                            print!("S");
                        } else if entity == PUSH {
                            print!("P");
                        } else {
                            print!("?");
                        }
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn render_win<T: Renderer>(&self, board: &Board<T>) {
        println!("You win!");
        self.render(board);
    }

    fn render_defeat<T: Renderer>(&self, board: &Board<T>) {
        println!("You lose!");
        self.render(board);
    }
}
