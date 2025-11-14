// This file manages the evaluation of a board state by determining whether or
// not white or black is winning and assigning a numerical score to a specific
// board.

use cozy_chess::*;

pub type EvalInt = i32;

fn get_val(piece: Piece) -> EvalInt {
    if piece == Piece::Pawn {
        return 100;
    } 
    if piece == Piece::Knight {
        return 250;
    }
    if piece == Piece::Bishop {
        return 300;
    }
    if piece == Piece::Rook {
        return 500;
    }
    if piece == Piece::Queen {
        return 900;
    }
    if piece == Piece::King {
        return 20000;
    }
    return 0;
}

pub trait Eval {
    fn eval(&self) -> EvalInt;
}

impl Eval for Board {

    fn eval(&self) -> EvalInt {
        let mut white_score = 0;
        let mut black_score = 0;


        let bb = BitBoard::FULL;
        let squares = &Square::ALL;
        for (s1, &_s2) in bb.iter().zip(squares) {
            if self.color_on(s1) == Some(Color::White) {
                white_score += get_val(self.piece_on(s1).unwrap());
            } else if self.color_on(s1) == Some(Color::Black) {
                black_score += get_val(self.piece_on(s1).unwrap());
            }
        }

        let score = white_score - black_score;
        if self.side_to_move() == Color::Black {
            return score * -1;
        }
        return score;
    }

}