use crate::board::Board;

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
                        use crate::global::Entity::*;
                        match entity {
                            Baba => print!("B"),
                            Flag => print!("F"),
                            Wall => print!("W"),
                            Rock => print!("R"),
                            Water => print!("w"),
                            Lava => print!("L"),
                            Skull => print!("S"),

                            Is => print!("i"),
                            You => print!("y"),
                            Win => print!("w"),
                            Defeat => print!("d"),
                            Stop => print!("s"),
                            Push => print!("p"),

                            BabaB => print!("b"),
                            FlagB => print!("f"),
                            WallB => print!("w"),
                            RockB => print!("r"),
                            WaterB => print!("w"),
                            LavaB => print!("l"),
                            SkullB => print!("s"),
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
