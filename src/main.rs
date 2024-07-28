use std::io;

const player_X: char = 'X';
const player_O: char = 'O';
const BOARD_SIZE: usize = 3;
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> Board {
    return [[' '; BOARD_SIZE]; BOARD_SIZE];
}

fn print_board(board: &Board) {
    for row in board.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}
fn get_move(board: &Board, current_player: char) -> (usize, usize) {
    loop {
        println!("Player {} input row col", current_player);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut coordinates: Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();
        if coordinates.len() == 2 {
            let row = coordinates.remove(0);
            let col = coordinates.remove(0);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }
        println!("Invalid input Enter Again")
    }
}
fn checkWinner(board: &Board,current_player: char)->bool{
    
    for i in 0..BOARD_SIZE{
        // row check
        if board[i][0] == current_player && board[i][1] == current_player && board[i][2] == current_player{
            return true;
        }
        // column check
        if board[0][i] == current_player && board[1][i] == current_player && board[2][i] == current_player{
            return true;
        }
    }
    // diagonal check
    if board[0][0] == current_player && board[1][1] == current_player && board[2][2] == current_player{
        return true;
    }
    if board[0][2] == current_player && board[1][1] == current_player && board[2][0] == current_player{
        return true;
    }
    return false;
}
fn check_draw(board: &Board)->bool{
    for i in 0..BOARD_SIZE{
        for j in 0..BOARD_SIZE{
            if board[i][j] == ' '{
                return false;
            }
        }
    }
    return true;
}
fn play_game() {
    let mut board = initialize_board();
    let mut current_player = player_X;
    loop {
        print_board(&board);
        println!("current_player {}'s turn.", current_player);
        let (row, col) = get_move(&board, current_player);
        board[row][col] = current_player;

        if checkWinner(&board,current_player){
            println!("Player {} wins the game",current_player);
            break;
        }
        if check_draw(&board){
            println!("Game is draw !! No one is the winner");
            break;
        }
        current_player = if current_player == player_X {
            player_O
        } else {
            player_X
        };
    }
}

fn main() {
    println!("Welcome to tic tac toe game !!");
    play_game();
}
