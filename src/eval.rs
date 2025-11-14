// This file manages the evaluation of a board state by determining whether or
// not white or black is winning and assigning a numerical score to a specific
// board.

use cozy_chess::*;

pub type EvalInt = i16;

fn getVal(piece: Option<Piece>) -> EvalInt {
    if piece == Some(Piece::Pawn) {
        return 100;
    } 
    if piece == Some(Piece::Knight) {
        return 250;
    }
    if piece == Some(Piece::Bishop) {
        return 300;
    }
    if piece == Some(Piece::Rook) {
        return 500;
    }
    if piece == Some(Piece::Queen) {
        return 900;
    }
    if piece == Some(Piece::King) {
        return 20000;
    }
    return 0;
}

pub trait Eval {
    fn eval(&self) -> EvalInt;
}

impl Eval for Board {

    fn eval(&self) -> EvalInt {
        let mut whiteScore = 0;
        let mut blackScore = 0;


        let bb = BitBoard::FULL;
        let squares = &Square::ALL;
        for (s1, &s2) in bb.iter().zip(squares) {
            if self.color_on(s1) == Some(Color::White) {
                whiteScore += getVal(self.piece_on(s1));
            } else if self.color_on(s1) == Some(Color::Black) {
                blackScore += getVal(self.piece_on(s1));
            }
        }

        let score = whiteScore - blackScore;
        return score;
    }

}