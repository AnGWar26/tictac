use std::fmt;

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum Tile {
    X,
    O,
    #[default]
    Empty,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Tile::X => "X",
            Tile::O => "O",
            Tile::Empty => " ",
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameStatus {
    Victor(Tile),
    Draw,
    InProgress,
}

#[derive(Debug, Clone)]
pub struct Board {
    board: [[Tile; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Tile::Empty; 3], [Tile::Empty; 3], [Tile::Empty; 3]],
        }
    }
    pub fn insert_move(&self, mutate_with: Move, fill_with: Tile) -> Result<Board, String> {
        let proposed_move = self.board[mutate_with.0 as usize][mutate_with.1 as usize];
        match proposed_move {
            Tile::Empty => {
                let mut new_board = self.board;
                new_board[mutate_with.0 as usize][mutate_with.1 as usize] = fill_with;
                Ok(Board { board: new_board })
            }
            _ => Err("You can't make a move where someone has already played a move".to_owned()),
        }
    }
    pub fn check_victory(&self) -> GameStatus {
        for r in 0..3 {
            if self.board[r][0] != Tile::Empty
                && self.board[r][0] == self.board[r][1]
                && self.board[r][1] == self.board[r][2]
            {
                return GameStatus::Victor(self.board[r][0]);
            }
        }
        // columns
        for c in 0..3 {
            if self.board[0][c] != Tile::Empty
                && self.board[0][c] == self.board[1][c]
                && self.board[1][c] == self.board[2][c]
            {
                return GameStatus::Victor(self.board[0][c]);
            }
        }

        // diagonals
        if self.board[0][0] != Tile::Empty
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
        {
            return GameStatus::Victor(self.board[0][0]);
        }
        if self.board[0][2] != Tile::Empty
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
        {
            return GameStatus::Victor(self.board[0][2]);
        }

        // Check for draw (if board is full) or in progress
        for r in 0..3 {
            for c in 0..3 {
                if self.board[r][c] == Tile::Empty {
                    return GameStatus::InProgress;
                }
            }
        }

        // If no empty tiles are found and there is no victor, it's a draw
        GameStatus::Draw
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " {} | {} | {}\n---|---|---\n {} | {} | {}\n---|---|---\n {} | {} | {}",
            self.board[0][0],
            self.board[1][0],
            self.board[2][0],
            self.board[0][1],
            self.board[1][1],
            self.board[2][1],
            self.board[0][2],
            self.board[1][2],
            self.board[2][2]
        )
    }
}

pub struct Move(u8, u8);

impl Move {
    pub fn new(x: u8, y: u8) -> Result<Move, String> {
        match x {
            0..3 => (),
            _ => return Err("Move is outside of the board".to_owned()),
        };
        match y {
            0..3 => (),
            _ => return Err("Move is outside of the board".to_owned()),
        };
        Ok(Move(x, y))
    }
}
