use crate::board::Board;
use std::io::Write;

pub trait Renderer {
    fn render<T: Renderer>(&self, board: &Board<T>, writer: &mut dyn Write);
    fn render_win<T: Renderer>(&self, board: &Board<T>, writer: &mut dyn Write);
    fn render_defeat<T: Renderer>(&self, board: &Board<T>, writer: &mut dyn Write);
}

pub struct CuiRenderer;

impl Renderer for CuiRenderer {
    fn render<T: Renderer>(&self, board: &Board<T>, writer: &mut dyn Write) {
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
                            Baba => write!(writer, "B").unwrap(),
                            Flag => write!(writer, "F").unwrap(),
                            Wall => write!(writer, "W").unwrap(),
                            Rock => write!(writer, "R").unwrap(),
                            Water => write!(writer, "w").unwrap(),
                            Lava => write!(writer, "L").unwrap(),
                            Skull => write!(writer, "S").unwrap(),

                            Is => write!(writer, "i").unwrap(),
                            You => write!(writer, "y").unwrap(),
                            Win => write!(writer, "w").unwrap(),
                            Defeat => write!(writer, "d").unwrap(),
                            Stop => write!(writer, "s").unwrap(),
                            Push => write!(writer, "p").unwrap(),

                            BabaB => write!(writer, "b").unwrap(),
                            FlagB => write!(writer, "f").unwrap(),
                            WallB => write!(writer, "w").unwrap(),
                            RockB => write!(writer, "r").unwrap(),
                            WaterB => write!(writer, "w").unwrap(),
                            LavaB => write!(writer, "l").unwrap(),
                            SkullB => write!(writer, "s").unwrap(),
                        }
                    }
                } else {
                    write!(writer, " ").unwrap();
                }
            }
            writeln!(writer).unwrap();
        }
    }

    fn render_win<T: Renderer>(&self, board: &Board<T>, writer: &mut dyn Write) {
        writeln!(writer, "You win!").unwrap();
        self.render(board, writer);
    }

    fn render_defeat<T: Renderer>(&self, board: &Board<T>, writer: &mut dyn Write) {
        writeln!(writer, "You lose!").unwrap();
        self.render(board, writer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board; // Removed MoveResult
    use crate::global::Entity; // Removed Direction, IsState, ENTITY_COUNT
    use std::collections::HashMap;
    use std::io::Cursor; // Using Cursor for a Write-enabled Vec<u8>

    // Helper function to create a board for testing
    fn create_test_board(entities: Vec<([i32; 2], Vec<Entity>)>) -> Board<CuiRenderer> {
        let mut map = HashMap::new();
        for (pos, ents) in entities {
            map.insert(pos, ents);
        }
        Board::new(map, CuiRenderer) // CuiRenderer is a ZST, can be constructed directly
    }

    #[test]
    fn test_render_simple_board() {
        let board = create_test_board(vec![
            ([0, 0], vec![Entity::Baba]),
            ([0, 1], vec![Entity::Wall]),
            ([1, 0], vec![Entity::Flag]),
        ]);
        let renderer = CuiRenderer;
        let mut buffer = Cursor::new(Vec::new()); // Using Cursor as it implements Write

        renderer.render(&board, &mut buffer);

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        // Expected:
        // BW
        // F 
        // (Note: trailing space after F because [1,1] is empty but within max_j bounds)
        assert_eq!(output, "BW\nF \n");
    }

    #[test]
    fn test_render_empty_board() {
        let board = create_test_board(vec![]);
        let renderer = CuiRenderer;
        let mut buffer = Cursor::new(Vec::new());

        renderer.render(&board, &mut buffer);

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "\n"); // Only a newline for 0x0 empty board
    }

    #[test]
    fn test_render_board_with_multiple_entities_on_tile() {
        let board = create_test_board(vec![
            ([0, 0], vec![Entity::Baba, Entity::Rock]),
        ]);
        let renderer = CuiRenderer;
        let mut buffer = Cursor::new(Vec::new());

        renderer.render(&board, &mut buffer);

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        // The current renderer prints one char per entity, last one wins.
        // If Baba is B and Rock is R, it would print R if Rock is last in Vec.
        // Let's check the actual rendering logic: it iterates and prints. So, it will print "BR".
        assert_eq!(output, "BR\n");
    }

    #[test]
    fn test_render_win() {
        let board = create_test_board(vec![([0,0], vec![Entity::Baba])]);
        let renderer = CuiRenderer;
        let mut buffer = Cursor::new(Vec::new());

        renderer.render_win(&board, &mut buffer);
        
        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "You win!\nB\n");
    }

    #[test]
    fn test_render_defeat() {
        let board = create_test_board(vec![([0,0], vec![Entity::Skull])]);
        let renderer = CuiRenderer;
        let mut buffer = Cursor::new(Vec::new());

        renderer.render_defeat(&board, &mut buffer);

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "You lose!\nS\n");
    }
}
