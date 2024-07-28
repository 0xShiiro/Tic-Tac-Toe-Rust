# Tic Tac Toe in Rust

Welcome to the Tic Tac Toe game implemented in Rust! This is a simple command-line version of the classic game, where two players take turns to mark the spaces in a 3×3 grid with X and O. The player who succeeds in placing three of their marks in a horizontal, vertical, or diagonal row wins the game.

## Features

- Two-player game mode
- Simple and intuitive command-line interface
- Validates player moves
- Detects win conditions and draws

## Installation

To play the Tic Tac Toe game, you need to have Rust installed on your system. If you don't have Rust installed, you can install it from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:
    ```sh
    git clone https://github.com/Codefreakpriyanshu/Tic-Tac-Toe-Rust.git
    ```
2. Navigate to the project directory:
    ```sh
    cd tic-tac-toe-rust
    ```
3. Build the project:
    ```sh
    cargo build --release
    ```
4. Run the game:
    ```sh
    cargo run
    ```

## Usage

After running the game, follow the on-screen instructions to play. Players will take turns to enter their move by specifying the row and column numbers (both starting from 0).

Example of a move:


This will place the player's mark in the second row and third column.

## Game Rules

1. The game is played on a 3x3 grid.
2. One player is X and the other player is O. Players take turns to place their marks.
3. The first player to get three of their marks in a row (vertically, horizontally, or diagonally) is the winner.
4. If all nine squares are filled and neither player has three in a row, the game is a draw.


## Acknowledgements

- Rust Programming Language: [rust-lang.org](https://www.rust-lang.org/)


Enjoy playing Tic Tac Toe in Rust!

