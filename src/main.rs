// This is the file that runs UCI
use cozy_chess::*;
use cozy_chess::util::*;
use crate::eval::*;
use std::io;
mod eval;

macro_rules! ignore {
    () => {
        continue
    };
}

// Function just to display the info required by UCI for the GUI about the
// gleam engine.
// Arguments: None
// Returns: None
fn cmd_uci() -> String {
    let str = "id name gleam\n\
               id author rain\n\
               uciok";
    str.into()
}

// Loads the board with moves
// Arguments: Takes in the tokens from the terminal and the board
// Returns: The board state with the moves being made.
fn cmd_position_moves(mut tokens: std::str::SplitWhitespace<'_>, mut board: Board) -> Board {
    while let Some(token) = tokens.next() {
        match token {
            "moves" => {
                for mv in tokens.by_ref() {
                    board.play(parse_uci_move(&board,mv).unwrap());
                    
                }
            }
            _ => ignore!(),
        }
    }

    board
}

// Sets the position of a board.
// Arguments: Tokens
// Returns: Board
fn set_position(mut tokens: std::str::SplitWhitespace<'_>) -> Board {
    while let Some(token) = tokens.next() {
        match token {
            "fen" => {
                let mut fen = String::with_capacity(64);
                
                for i in 0..6 {
                    fen.push_str(tokens.next().expect("FEN missing fields"));
                    if i < 5 {
                        fen.push(' ')
                    }
                }

                let mut board = Board::from_fen(&fen,false)
                    .unwrap_or_else(|e| panic!("failed to parse fen '{fen}': {e:?}"));
                board = cmd_position_moves(tokens,board);

                return board;
            }
            "startpos" => {
                let mut board = Board::default();
                board = cmd_position_moves(tokens,board);
                return board;
            }
            _ => ignore!(),
        }
    }
    panic!("position command was empty")
}

// Function for doing a move. Prints to terminal the move the computer chooses
// Arguments: Tokens and a board
// Returns: None
fn cmd_go(mut _tokens: std::str::SplitWhitespace<'_>, board: &mut Board) {
    let mut vec = Vec::new();

    board.generate_moves(|moves| {
        for _mv in moves {
            vec.push(_mv);
        }
        false
    });
    
    let chosen = vec.last();

    match chosen {
        Some(mv) => {
            let thing = display_uci_move(&board,*mv);
            println!("bestmove {}", thing);
        }
        None => println!("bestmove 0000"),
    }
}



fn main() {
    let stdin = io::stdin();
    
    let mut board = Board::default();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut tokens = line.split_whitespace();
        while let Some(token) = tokens.next() {
            match token {
                "uci" => {
                    println!("{}", cmd_uci());
                }
                "isready" => {
                    println!("readyok");
                }
                "ucinewgame" => {
                    board = Board::default();
                }
                "quit" => {
                    return;
                }
                "position" => {
                    board = set_position(tokens);
                }
                "go" => {
                    cmd_go(tokens, &mut board);
                }
                _ => ignore!(),
            }
            break;
        }
    }
}

