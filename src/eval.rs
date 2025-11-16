// This file manages the evaluation of a board state by determining whether or
// not white or black is winning and assigning a numerical score to a specific
// board.

use cozy_chess::*;

pub type EvalInt = i32;

type psqt = [EvalInt;64];

// Piece Square Tables from the pov of White
const pawn_pst : psqt = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

const knight_pst : psqt = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const bishop_pst : psqt = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const rook_pst : psqt = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

const queen_pst : psqt = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const king_pst : psqt = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];

fn convertFile(file: File) -> usize {
    return file as usize;
}

fn convertRank(rank: Rank) -> usize {
    return rank as usize;
}

fn get_val(piece: Piece, index: usize) -> EvalInt {
    
    if piece == Piece::Pawn {
        return 100 + pawn_pst[index];
    } 
    if piece == Piece::Knight {
        return 250 + knight_pst[index];
    }
    if piece == Piece::Bishop {
        return 300 + bishop_pst[index];
    }
    if piece == Piece::Rook {
        return 500 + rook_pst[index];
    }
    if piece == Piece::Queen {
        return 900 + queen_pst[index];
    }
    if piece == Piece::King {
        return 20000 + king_pst[index];
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
        for square in bb.iter() {
            if self.color_on(square) == Some(Color::White) {
                let index = convertFile(square.file()) * 8 + convertRank(square.rank());
    
                white_score += get_val(self.piece_on(square).unwrap(),index);
            } else if self.color_on(square) == Some(Color::Black) {
                let blackIndex = (7 - convertFile(square.file())) * 8 + (7 - convertRank(square.rank()));
                black_score += get_val(self.piece_on(square).unwrap(), blackIndex);
            }
        }

        let score = white_score - black_score;
        if self.side_to_move() == Color::Black {
            return score * -1;
        }
        return score;
    }

}

#[cfg(test)]
mod tests {
    use cozy_chess::*;
    use crate::eval::*;
    #[test]
    fn test_eval() {
        let board1 = Board::from_fen("rnbq1b2/4kQ2/4B3/1N6/1N6/P5P1/P5PP/R1B1R1K1 b - - 2 29", false).unwrap();
        let eval1 = board1.eval();
        let board2 = Board::from_fen("8/8/8/8/7k/K1n5/8/8 b - - 0 81", false).unwrap();
        let eval2 = board2.eval();

        assert!(eval1 < 0, "got eval {eval1}");
        assert!(eval2 > 0, "got eval {eval2}");
    }
}