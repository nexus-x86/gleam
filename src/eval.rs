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

// eg = end game
const eg_pawn_pst: psqt = [
    0,   0,   0,   0,   0,   0,   0,   0,
    178, 173, 158, 134, 147, 132, 165, 187,
    94, 100,  85,  67,  56,  53,  82,  84,
    32,  24,  13,   5,  -2,   4,  17,  17,
    13,   9,  -3,  -7,  -7,  -8,   3,  -1,
    4,   7,  -6,   1,   0,  -5,  -1,  -8,
    13,   8,   8,  10,  13,   0,   2,  -7,
    0,   0,   0,   0,   0,   0,   0,   0,
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

const eg_knight_pst : psqt = [
    -58, -38, -13, -28, -31, -27, -63, -99,
    -25,  -8, -25,  -2,  -9, -25, -24, -52,
    -24, -20,  10,   9,  -1,  -9, -19, -41,
    -17,   3,  22,  22,  22,  11,   8, -18,
    -18,  -6,  16,  25,  16,  17,   4, -18,
    -23,  -3,  -1,  15,  10,  -3, -20, -22,
    -42, -20, -10,  -5,  -2, -20, -23, -44,
    -29, -51, -23, -15, -22, -18, -50, -64,
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

const eg_bishop_pst : psqt = [
    -14, -21, -11,  -8, -7,  -9, -17, -24,
    -8,  -4,   7, -12, -3, -13,  -4, -14,
    2,  -8,   0,  -1, -2,   6,   0,   4,
    -3,   9,  12,   9, 14,  10,   3,   2,
    -6,   3,  13,  19,  7,  10,  -3,  -9,
    -12,  -3,   8,  10, 13,   3,  -7, -15,
    -14, -18,  -7,  -1,  4,  -9, -15, -27,
    -23,  -9, -23,  -5, -9, -16,  -5, -17,
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

const eg_rook_pst : psqt = [
    13, 10, 18, 15, 12,  12,   8,   5,
    11, 13, 13, 11, -3,   3,   8,   3,
    7,  7,  7,  5,  4,  -3,  -5,  -3,
    4,  3, 13,  1,  2,   1,  -1,   2,
    3,  5,  8,  4, -5,  -6,  -8, -11,
    -4,  0, -5, -1, -7, -12,  -8, -16,
    -6, -6,  0,  2, -9,  -9, -11,  -3,
    -9,  2,  3, -1, -5, -13,   4, -20,
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

const eg_queen_pst : psqt = [
    -9,  22,  22,  27,  27,  19,  10,  20,
    -17,  20,  32,  41,  58,  25,  30,   0,
    -20,   6,   9,  49,  47,  35,  19,   9,
    3,  22,  24,  45,  57,  40,  57,  36,
    -18,  28,  19,  47,  31,  34,  39,  23,
    -16, -27,  15,   6,   9,  17,  10,   5,
    -22, -23, -30, -16, -16, -23, -36, -32,
    -33, -28, -22, -43,  -5, -32, -20, -41,
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

const eg_king_pst : psqt = [
    -74, -35, -18, -18, -11,  15,   4, -17,
    -12,  17,  14,  17,  17,  38,  23,  11,
    10,  17,  23,  15,  20,  45,  44,  13,
    -8,  22,  24,  27,  26,  33,  26,   3,
    -18,  -4,  21,  24,  27,  23,   9, -11,
    -19,  -3,  11,  21,  23,  16,   7,  -9,
    -27, -11,   4,  13,  14,   4,  -5, -17,
    -53, -34, -21, -11, -28, -14, -24, -43
];

fn convertFile(file: File) -> usize {
    return file as usize;
}

fn convertRank(rank: Rank) -> usize {
    return rank as usize;
}

fn get_val(piece: Piece, index: usize, endgame: bool) -> EvalInt {
    
    if piece == Piece::Pawn {
        if endgame {return 100 + eg_pawn_pst[index];}
        return 100 + pawn_pst[index];
    } 
    if piece == Piece::Knight {
        if endgame {return 250 + eg_knight_pst[index];}
        return 250 + knight_pst[index];
    }
    if piece == Piece::Bishop {
        if endgame {return 300 + eg_bishop_pst[index];}
        return 300 + bishop_pst[index];
    }
    if piece == Piece::Rook {
        if endgame {return 500 + eg_rook_pst[index];}
        return 500 + rook_pst[index];
    }
    if piece == Piece::Queen {
        if endgame {return 900 + eg_queen_pst[index];}
        return 900 + queen_pst[index];
    }
    if piece == Piece::King {
        if endgame {return 20000 + eg_king_pst[index];}
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

        let mut endgame = false;
        if self.occupied().len() <= 16 {
            endgame = true
        }

        let bb = BitBoard::FULL;
        for square in bb.iter() {
            if self.color_on(square) == Some(Color::White) {
                let index = convertFile(square.file()) * 8 + convertRank(square.rank());
    
                white_score += get_val(self.piece_on(square).unwrap(),index, endgame);
            } else if self.color_on(square) == Some(Color::Black) {
                let blackIndex = (7 - convertFile(square.file())) * 8 + (7 - convertRank(square.rank()));
                black_score += get_val(self.piece_on(square).unwrap(), blackIndex, endgame);
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