//this file is the main.rs file with the main function in it
mod ai;
mod case;
mod save;
mod game;
mod board;
mod player;

use std::io;
use ai::AI;
use case::Case;
use save::Save;
use player::Type;
use game::{Game, GameState};

//main function necessary in every Rust programs
fn main() {
    println!("Hello, welcome to my Rust tic tac toe game ! If you want the tic tac toe rules, here they are: https://www.exploratorium.edu/explore/puzzles/tictactoe");
    println!("After that, good luck and have fun !");
    loop {
        if let Err(e) = Save::delete_previous() {
            eprintln!("Failed to save the game: {}", e);
        }
        let mut g = Game::init();
        g.board().print();
        let mut case: String;
        while *g.state() == GameState::Ongoing {
            match g.current_turn().p_type() {
                Type::Human => {
                    println!("Player {}, please input the box you want to play on.", match g.current_turn().sign() {
                        Case::Cross => "X",
                        Case::Circle => "O",
                        _ => "?",
                    });
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    case = input.trim().to_string();
                }
                Type::Bot => {
                    println!("Bot playing.");
                    case = AI::bot_playing(g.board());
                }
            }
            if g.play(&case){
                if let Err(e) = Save::write_to_file(&g) {
                    eprintln!("Failed to save the game: {}", e);
                }
            }
        }
        match g.state() {
            GameState::Win(_) => {
                if let Err(e) = Save::rename_file("history/save.txt", format!("history/{}_win({})", format!("{}", g.current_turn().sign().result()).as_str(), Save::get_new_file_index("{}_win").as_str()).as_str()) {
                    eprintln!("WFailed to save the game: {}", e);
                }
            },
            GameState::Draw => {
                if let Err(e) = Save::rename_file("history/save.txt", format!("history/Draw({}).txt", Save::get_new_file_index("Draw").as_str()).as_str()) {
                    eprintln!("DFailed to save the game: {}", e);
                }
            },
            _ => println!("error"),
        }

        println!("Play again ? y/n");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();
        if choice.to_lowercase() != "y" {
            break;
        }
    }
}
