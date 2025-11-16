use crate::eval::*;
use cozy_chess::*;
use std::cmp::max;
use std::collections::HashMap;

const EVAL_WORST: EvalInt = -(EvalInt::MAX);
const EVAL_BEST: EvalInt = EvalInt::MAX;

const magicConstant : usize = (1 << 20) / 12; 


#[derive(Clone, Copy, Debug)]
pub struct TranspositionEntry {
    pub best_move: Move,
    pub eval: EvalInt,
    pub depth: u8,
}

pub struct HashTable {
    data: Vec<Option<(u64, TranspositionEntry)>>,
    size: usize,
}

pub fn mib_to_n<T: Sized>(mib: usize) -> usize {
    let bytes = mib * (1 << 20);
    let entry_size = std::mem::size_of::<(u64, Option<T>)>();

    bytes / entry_size
}

impl Default for HashTable {
    fn default() -> Self {
        Self::new(8)
    }
}

impl HashTable {
    pub fn new(size_mib: usize) -> Self {
        let size = mib_to_n::<TranspositionEntry>(size_mib);
        Self::new_n(size)
    }

    pub fn new_n(size: usize) -> Self {
        HashTable {
            data: vec![None; size],
            size,
        }
    }

    pub fn trunc_hash(&self, hash: u64) -> usize {
        hash as usize % self.size
    }

    pub fn probe(&self, board: &Board) -> Option<TranspositionEntry> {
        let hash = board.hash();
        let idx = self.trunc_hash(hash);
        if let Some((e_hash, entry)) = self.data[idx] {
            if e_hash == hash {
                return Some(entry);
            }
        }
        None
    }

    pub fn set(&mut self, board: &Board, entry: TranspositionEntry) {
        let hash = board.hash();
        let idx = self.trunc_hash(hash);
        self.data[idx] = Some((hash, entry));
    }
}
// Does quiescence search
// was advised to implement sprt before quies
fn quiesce(board: &mut Board, alpha: Option<EvalInt>, beta: Option<EvalInt>, transpositionTable: &mut HashTable) -> EvalInt {
    let static_eval = board.eval();
    let mut best_value = static_eval;
    let mut move_list = Vec::new();
    board.generate_moves(|moves| {
        move_list.extend(moves);
        false
    });
    let mut best_mv: Option<Move> = move_list.first().copied();

    let mut alpha = alpha.unwrap_or(EVAL_WORST);
    let beta = beta.unwrap_or(EVAL_BEST);

    if let Some(entry) = transpositionTable.probe(&board) {
        if entry.depth as usize >= 0 {
            return entry.eval;
        }
    }

    if best_value >= beta {
        return best_value;
    }
    if best_value > alpha {
        alpha = best_value;
    }

    let enemy_pieces = board.colors(!board.side_to_move());
    let mut captures = Vec::new();
    board.generate_moves(|moves| {
        let mut captures2 = moves.clone();
        // Bitmask to efficiently get all captures set-wise.
        // Excluding en passant square for convenience.
        captures2.to &= enemy_pieces;
        captures.extend(captures2);
        false
    });

    for mv in captures {
        let mut new_board = board.clone();
        new_board.play(mv);
        let cur_score = -quiesce(&mut new_board, Some(-alpha), Some(-beta), transpositionTable);

        if cur_score >= beta {
            return cur_score;
        }
        if cur_score > best_value {
            best_mv = Some(mv);
            best_value = cur_score;
        }
        if cur_score > alpha {
            alpha = cur_score;
        }
    }
    if !best_mv.is_none() {
        transpositionTable.set(
            &board,
            TranspositionEntry {
                best_move: best_mv.unwrap(),
                eval: best_value,
                depth: 0,
            },
        );
    }

    //setHash(board, best_value, transpositionTable);

    return best_value;
}

// Search the game tree to find the best outcome for the player
// Uses the negamax algorithm.
fn minmax(board: &mut Board, depth: usize, alpha: Option<EvalInt>, beta: Option<EvalInt>, transpositionTable: &mut HashTable) -> EvalInt {
    if depth == 0 {
        //return board.eval();
        return quiesce(board, alpha, beta, transpositionTable);
    }

    if let Some(entry) = transpositionTable.probe(&board) {
        if entry.depth as usize >= depth {
            return entry.eval;
        }
    }

    let mut alpha = alpha.unwrap_or(EVAL_WORST);
    let beta = beta.unwrap_or(EVAL_BEST);

    let mut move_list = Vec::new();
    board.generate_moves(|moves| {
        move_list.extend(moves);
        false
    });

    let mut abs_best = EVAL_WORST;
    let mut best_mv: Option<Move> = move_list.first().copied();

    if board.status() == GameStatus::Won {
        return EVAL_WORST;
    } else if board.status() == GameStatus::Drawn {
        return 0;
    }
    

    for mv in move_list {
        let mut new_board = board.clone();
        new_board.play(mv);
        let mut abs_score = 0;
        if new_board.checkers() == BitBoard::EMPTY { // is someone in check
            abs_score = -minmax(&mut new_board, depth-1, Some(-beta),Some(-alpha), transpositionTable);
        } else {
            abs_score = -minmax(&mut new_board, depth, Some(-beta),Some(-alpha), transpositionTable);
        }
        if abs_score > abs_best {
            abs_best = abs_score;
            best_mv = Some(mv);
        }
        alpha = max(alpha,abs_best);
        if alpha >= beta {
            break;
        }
    }

    if !best_mv.is_none() {
        transpositionTable.set(
            &board,
            TranspositionEntry {
                best_move: best_mv.unwrap(),
                eval: abs_best,
                depth: depth as u8,
            },
        );
    }



    return abs_best;
}



// Finds the best move for a position
fn search(board: &mut Board, transpositionTable: &mut HashTable) -> Option<Move> {
    const DEPTH: usize = 2;
    let mut move_list = Vec::new();
    board.generate_moves(|moves| {
        move_list.extend(moves);
        false
    });

    let mut best_eval = EVAL_WORST;
    let mut best_mv: Option<Move> = move_list.first().copied();

    if let Some(entry) = transpositionTable.probe(&board) {
        if entry.depth as usize >= DEPTH {
            return Some(entry.best_move);
        }
    }

    for mv in move_list {
        let mut new_board = board.clone();
        new_board.play(mv);

        

        let abs_eval = -minmax(&mut new_board, DEPTH, None, None, transpositionTable);

        
        if abs_eval > best_eval {
            best_eval = abs_eval;
            best_mv = Some(mv);
        }
    }
    transpositionTable.set(
        &board,
        TranspositionEntry {
            best_move: best_mv.unwrap(),
            eval: best_eval,
            depth: DEPTH as u8,
        },
    );

    return best_mv;
}

/// Find the best move.
pub fn best_move(board: &mut Board, transpositionTable: &mut HashTable) -> Move {
    return search(board, transpositionTable).unwrap();
}