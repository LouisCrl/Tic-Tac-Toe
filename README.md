# Rust Tic Tac Toe

![image](https://github.com/user-attachments/assets/122bbfed-1e3b-43c8-bc65-4813e617a500)

## Installation

The game is written in [Rust](https://www.rust-lang.org). If you do not have a rust toolchain on your system 
yet, [install](https://www.rust-lang.org/tools/install) it, rustup is the preferred way to change that. When the toolchain is ready, go in your the file, build and run the
game with

    cargo run --release
    

## How to play

Here are the [rules](https://www.exploratorium.edu/explore/puzzles/tictactoe) (even if every body knows them). <br>
There is 2 players, player X and player O, the first to play is X and the second O. <br>
You have to give the coordinates (they go from a1 to c3) and the board will update itself. <br>
Every game is saved in a folder name "history", the name of every partie is like that: X_Win(n).txt or Draw(n).txt, with X the symbol of the winner and n the number of time the file already exist. <br>
