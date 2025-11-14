use crate::eval::*;
use cozy_chess::*;
use std::cmp::max;

const EVAL_WORST: EvalInt = -(EvalInt::MAX);
const EVAL_BEST: EvalInt = EvalInt::MAX;



// Search the game tree to find the best outcome for the player
fn minmax(board: &mut Board, depth: usize, alpha: Option<EvalInt>, beta: Option<EvalInt>) -> EvalInt {
    if depth == 0 {
        return board.eval();
    }

    let mut alpha = alpha.unwrap_or(EVAL_WORST);
    let beta = beta.unwrap_or(EVAL_BEST);

    let mut move_list = Vec::new();
    board.generate_moves(|moves| {
        move_list.extend(moves);
        false
    });

    let mut abs_best = EVAL_WORST;

    if board.status() == GameStatus::Won {
        return EVAL_WORST;
    } else if board.status() == GameStatus::Drawn {
        return 0;
    }
    

    for mv in move_list {
        let mut new_board = board.clone();
        new_board.play(mv);
        let abs_score = -minmax(&mut new_board, depth-1, Some(-beta),Some(-alpha));
        abs_best = max(abs_best, abs_score);
        alpha = max(alpha,abs_best);
        if alpha >= beta {
            break;
        }
    }

    abs_best
}



// Finds the best move for a position
fn search(board: &mut Board) -> Option<Move> {
    const DEPTH: usize = 4;
    let mut move_list = Vec::new();
    board.generate_moves(|moves| {
        move_list.extend(moves);
        false
    });

    let mut best_eval = EVAL_WORST;
    let mut best_mv: Option<Move> = None;

    for mv in move_list {
        let mut new_board = board.clone();
        new_board.play(mv);
        let abs_eval = -minmax(&mut new_board, DEPTH, None, None);
        if abs_eval > best_eval {
            best_eval = abs_eval;
            best_mv = Some(mv);
        }
    }

    best_mv
}

/// Find the best move.
pub fn best_move(board: &mut Board) -> Move {
    return search(board).unwrap();
}