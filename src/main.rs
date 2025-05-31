use std::io;

mod structs;
use structs::{Board, GameStatus, Move, Tile};

fn game_loop(board: &Board, player: Tile) -> Result<Board, String> {
    let mut line = String::new();
    let _inp = io::stdin().read_line(&mut line).unwrap();
    let nums: Vec<&str> = line.trim().split(",").collect();
    let x = match nums[0].parse::<u8>() {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };
    let y = match nums[1].parse::<u8>() {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };
    let mve = Move::new(x, y)?;
    let board = board.insert_move(mve, player)?;
    Ok(board)
}

fn main() {
    let mut board: Board = Board::new();

    let mut tile = Tile::X;

    println!("{}", board);
    println!("Current turn: {}", tile);

    loop {
        board = match game_loop(&board, tile) {
            Ok(b) => b,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        match board.check_victory() {
            GameStatus::Victor(player) => {
                println!("{}", board);
                println!("{} wins", player);
                break;
            }
            GameStatus::Draw => {
                println!("Nobody wins!");
                break;
            }
            GameStatus::InProgress => {
                tile = match tile {
                    Tile::X => Tile::O,
                    Tile::O => Tile::X,
                    Tile::Empty => {
                        panic!("Should never be Tile::Empty here because this is for players!")
                    }
                }
            }
        };
        println!("{}", board);
        println!("Current turn: {}", tile);
    }
}
