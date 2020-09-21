use crate::types;
use crate::types::{Direction, Piece, Color, PieceType, Square, Rank, File, CastlingRights, Board, Move, GamePhase};
use crate::bitboard;
use crate::utils;
use crate::move_search;

pub fn eval_move(b: &Board, m: Move, depth: i32) -> i32 {
    if b.turn == Color::White {
        let eval = eval_move_min(b, m, depth-1, -1_000_000_000, 1_000_000_000);
        return eval;
    } else {
        let eval = eval_move_max(b, m, depth-1, -1_000_000_000, 1_000_000_000);
        return -eval;
    }
}

pub fn eval_move_max(b: &Board, m: Move, rem_depth: i32, alpha: i32, beta: i32) -> i32 {
    let board = utils::apply_move(b, m);

    if rem_depth == 0 {
        return eval_move_quick(&board);
    }

    let next_moves = move_search::calc_moves(&board);

    if next_moves.len() == 0 {
        let test_board = utils::apply_null_move(&board);
        if move_search::test_pmoves(&test_board) {
            // test_pmoves returns true if there are no king captures
            // in this case, that means we're stalemated
            return 0;
        } else {
            // else we've checkmated
            // to make it prefer slower mates, add in the "rem_depth"
            return -1_000_000 - rem_depth;
        }
    }

    let mut new_alpha = alpha;
    for next_move in next_moves.iter() {
        let m_eval = eval_move_min(&board, *next_move, rem_depth-1, new_alpha, beta);
        if m_eval >= beta {
            return beta;
        }
        if m_eval > new_alpha {
            new_alpha = m_eval;
        }
    }

    return new_alpha;
}

pub fn eval_move_min(b: &Board, m: Move, rem_depth: i32, alpha: i32, beta: i32) -> i32 {
    let board = utils::apply_move(b, m);

    if rem_depth == 0 {
        return eval_move_quick(&board);
    }

    let next_moves = move_search::calc_moves(&board);

    if next_moves.len() == 0 {
        let test_board = utils::apply_null_move(&board);
        if move_search::test_pmoves(&test_board) {
            // test_pmoves returns true if there are no king captures
            // in this case, that means we're stalemated
            return 0;
        } else {
            // else we've checkmated
            // to make it prefer quicker mates, add in the "rem_depth"
            return 1_000_000 + rem_depth;
        }
    }

    let mut new_beta = beta;
    for next_move in next_moves.iter() {
        let m_eval = eval_move_max(&board, *next_move, rem_depth-1, alpha, new_beta);
        if m_eval <= alpha {
            return alpha;
        }
        if m_eval < new_beta {
            new_beta = m_eval;
        }
    }

    return new_beta;
}

fn eval_move_quick(b: &Board) -> i32 {
    let mut white_eval = eval_move_quick_color(b, Color::White);
    let mut black_eval = eval_move_quick_color(b, Color::Black);

    return white_eval - black_eval;
}

fn eval_move_quick_color(b: &Board, c: Color) -> i32 {
    let mut eval = 0;

    let pieces = if c == Color::White { b.white_pieces } else { b.black_pieces };
    let bitboard_pieces = if c == Color::White { b.white_bitboard_pieces } else { bitboard::flip_bitboard_pieces(b.black_bitboard_pieces) };
    let enemy_bitboard_pieces = if c == Color::White { b.black_bitboard_pieces } else { bitboard::flip_bitboard_pieces(b.white_bitboard_pieces) };



    // add value for castling and for castling rights, more for castling short
    let can_castle_long = if b.turn == Color::White { b.castling_rights.white_long } else { b.castling_rights.black_long };
    let can_castle_short = if b.turn == Color::White { b.castling_rights.white_short } else { b.castling_rights.black_short };
    if can_castle_long {
        eval += 50;
    }
    if can_castle_short {
        eval += 75;
    }

    // value pawn islands and doubled pawns
    let mut last_had_pawns = false;
    let mut num_pawn_islands = 0;
    for f in bitboard::FILES.iter() {
        let pawns = (f & bitboard_pieces.pawns).count_ones() as i32;
        if pawns > 0 {
            if !last_had_pawns {
                num_pawn_islands += 1;
            }
            eval -= 40 * (pawns-1); // double or tripled pawns are less valuable
            last_had_pawns = true;
        }
    }
    eval -= 30 * num_pawn_islands;

    // if the material of each player isn't that high, use the endgame eval
    let mut gamephase = GamePhase::Middlegame;
    let num_minor_pieces = bitboard_pieces.bishops.count_ones() + bitboard_pieces.knights.count_ones();
    let num_major_pieces = bitboard_pieces.queens.count_ones() + bitboard_pieces.rooks.count_ones();
    let opp_num_minor_pieces = enemy_bitboard_pieces.bishops.count_ones() + enemy_bitboard_pieces.knights.count_ones();
    let opp_num_major_pieces = enemy_bitboard_pieces.queens.count_ones() + enemy_bitboard_pieces.rooks.count_ones();
    let cond = ((num_major_pieces + (num_minor_pieces/2)) <= 2) && ((opp_num_major_pieces + (opp_num_minor_pieces/2)) <= 2);
    let gamephase = if cond { GamePhase::Endgame } else { GamePhase::Middlegame };

    // add value for material (note: this value is relative based on location and gamephase)
    for p in pieces.iter() {
        eval += p.get_material_value(gamephase);
    }

    // if we're not in the endgame and we're castled, add value for the king having a pawn shield
    if (gamephase != GamePhase::Endgame) && ((bitboard_pieces.king & bitboard::CASTLED_KING_BITBOARD) != 0) {
        let king_shield_diag1 = bitboard::slide(bitboard_pieces.king, Direction::NW, 1) | bitboard::slide(bitboard_pieces.king, Direction::NE, 1);
        let king_shield_diag2 = bitboard::slide(bitboard_pieces.king, Direction::NW, 2) | bitboard::slide(bitboard_pieces.king, Direction::NE, 2);
        eval += 35 * ((king_shield_diag1 & bitboard_pieces.pawns).count_ones() as i32);
        eval += 20 * ((king_shield_diag2 & bitboard_pieces.pawns).count_ones() as i32);
        if (bitboard::slide(bitboard_pieces.king, Direction::N, 1) & bitboard_pieces.pawns) != 0 {
            eval += 50;
        }
        if (bitboard::slide(bitboard_pieces.king, Direction::N, 2) & bitboard_pieces.pawns) != 0 {
            eval += 30;
        }
    }

    return eval;
}