use rand::rng;
use rand::seq::IteratorRandom;
use crate::case::Case;
use crate::board::Board;

pub struct AI;

impl AI {
    pub fn bot_playing(board: &Board) -> String {
        let boxes = Self::check_available(board);

        if boxes.is_empty() {
            println!("No available moves");
        }

        let mut rng = rng();
        let test = boxes.into_iter().choose(&mut rng).unwrap();
        println!("{}", test);
        test
    }

    pub fn check_available(board: &Board) -> Vec<String> {
        let mut boxes: Vec<String> = Vec::new();

        for (i, row) in board.board().iter().enumerate() {
            for (j, board_box) in row.iter().enumerate() {
                if *board_box == Case::Void && *board_box != Case::Circle && *board_box != Case::Cross {
                    let coor = match (i, j) {
                        (0, 0) => "a1",
                        (1, 0) => "a2",
                        (2, 0) => "a3",
                        (0, 1) => "b1",
                        (1, 1) => "b2",
                        (2, 1) => "b3",
                        (0, 2) => "c1",
                        (1, 2) => "c2",
                        (2, 2) => "c3",
                        _ => "?",
                    };
                    if coor != "?" {
                        boxes.push(coor.to_string())
                    }
                }
            }
        }
        boxes
    }
}