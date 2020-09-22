use crate::types;
use crate::types::{Direction, Piece, Color, PieceType, Square, Rank, File, CastlingRights, Board, Move, GamePhase};
use crate::bitboard;
use crate::utils;
use crate::move_search;

pub const EVAL_MATE: i32 = 1_000_000;

pub const KING_EVAL: [i32; 64] = [
    50050, 50050, 50050, 50000, 50000, 50000, 50050, 50050, 
    50000, 50000, 50000, 50000, 50000, 50000, 50000, 50000, 
    50000, 50000, 50000, 50000, 50000, 50000, 50000, 50000,
    50000, 50000, 50000, 50000, 50000, 50000, 50000, 50000,
    50000, 50000, 50000, 50000, 50000, 50000, 50000, 50000,
    50000, 50000, 50000, 50000, 50000, 50000, 50000, 50000,
    50000, 50000, 50000, 50000, 50000, 50000, 50000, 50000,
    50000, 50000, 50000, 50000, 50000, 50000, 50000, 50000
];

pub const KING_ENDGAME_EVAL: [i32; 64] = [
    49940, 49950, 49960, 49970, 49970, 49960, 49950, 49940, 
    49950, 49960, 49970, 49980, 49980, 49970, 49960, 49950, 
    49960, 49970, 49980, 49990, 49990, 49980, 49970, 49960,
    49970, 49980, 49990, 50000, 50000, 49990, 49980, 49970,
    49970, 49980, 49990, 50000, 50000, 49990, 49980, 49970,
    49960, 49970, 49980, 49990, 49990, 49980, 49970, 49960,
    49950, 49960, 49970, 49980, 49980, 49970, 49960, 49950,
    49940, 49950, 49960, 49970, 49970, 49960, 49950, 49940
];

pub const QUEEN_EVAL: [i32; 64] = [
    940, 940, 940, 940, 940, 940, 940, 940, 
    940, 940, 940, 940, 940, 940, 940, 940, 
    940, 940, 940, 940, 940, 940, 940, 940,
    940, 940, 940, 940, 940, 940, 940, 940,
    940, 940, 940, 940, 940, 940, 940, 940,
    940, 940, 940, 940, 940, 940, 940, 940,
    940, 940, 940, 940, 940, 940, 940, 940,
    940, 940, 940, 940, 940, 940, 940, 940
];

pub const ROOK_EVAL: [i32; 64] = [
    510, 510, 510, 510, 510, 510, 510, 510, 
    510, 510, 510, 510, 510, 510, 510, 510, 
    510, 510, 510, 510, 510, 510, 510, 510,
    510, 510, 510, 510, 510, 510, 510, 510,
    510, 510, 510, 510, 510, 510, 510, 510,
    510, 510, 510, 510, 510, 510, 510, 510,
    510, 510, 510, 510, 510, 510, 510, 510,
    510, 510, 510, 510, 510, 510, 510, 510
];

pub const BISHOP_EVAL: [i32; 64] = [
    315, 325, 325, 325, 325, 325, 325, 315, 
    325, 335, 336, 340, 340, 336, 335, 325, 
    325, 336, 337, 341, 341, 337, 336, 325, 
    325, 340, 341, 345, 345, 341, 340, 325, 
    325, 340, 341, 345, 345, 341, 340, 325, 
    325, 336, 337, 341, 341, 337, 336, 325, 
    325, 335, 336, 340, 340, 336, 335, 325, 
    315, 325, 325, 325, 325, 325, 325, 315
];

pub const KNIGHT_EVAL: [i32; 64] = [
    250, 265, 280, 280, 280, 280, 265, 250, 
    280, 295, 315, 325, 325, 315, 295, 280, 
    280, 295, 315, 325, 325, 315, 295, 280, 
    280, 305, 335, 345, 345, 335, 305, 280, 
    280, 305, 335, 345, 345, 335, 305, 280, 
    280, 305, 335, 345, 345, 335, 305, 280, 
    280, 295, 315, 325, 325, 315, 295, 280, 
    280, 295, 310, 310, 310, 310, 295, 280
];

pub const PAWN_EVAL: [i32; 64] = [
    90,  90, 100, 110, 110, 100,  90,  90, 
    90,  90, 100,  45,  45, 100,  90,  90, 
    90,  90, 100, 110, 110, 100,  90,  90, 
    95,  95, 105, 115, 115, 105,  95,  95, 
   100, 100, 110, 120, 120, 110, 100, 100, 
   105, 105, 115, 125, 125, 115, 105, 105, 
   150, 150, 160, 170, 170, 160, 150, 150, 
    90,  90, 100, 110, 110, 100,  90,  90   
];

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
        return eval_pos_quick(&board);
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
            return -EVAL_MATE - rem_depth;
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
        return eval_pos_quick(&board);
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
            return EVAL_MATE + rem_depth;
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

fn eval_pos_quick(b: &Board) -> i32 {
    let mut white_eval = eval_pos_quick_color(b, Color::White);
    let mut black_eval = eval_pos_quick_color(b, Color::Black);

    return white_eval - black_eval;
}

fn eval_pos_quick_color(b: &Board, c: Color) -> i32 {
    let mut eval = 0;

    let bitboard_pieces = if c == Color::White { b.white_bitboard_pieces } else { bitboard::flip_bitboard_pieces(b.black_bitboard_pieces) };
    let enemy_bitboard_pieces = if c == Color::White { b.black_bitboard_pieces } else { bitboard::flip_bitboard_pieces(b.white_bitboard_pieces) };

    // add value for castling rights; more for castling short
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
    let num_minor_pieces = bitboard_pieces.bishops.count_ones() + bitboard_pieces.knights.count_ones();
    let num_major_pieces = bitboard_pieces.queens.count_ones() + bitboard_pieces.rooks.count_ones();
    let opp_num_minor_pieces = enemy_bitboard_pieces.bishops.count_ones() + enemy_bitboard_pieces.knights.count_ones();
    let opp_num_major_pieces = enemy_bitboard_pieces.queens.count_ones() + enemy_bitboard_pieces.rooks.count_ones();
    let cond = ((num_major_pieces + (num_minor_pieces/2)) <= 2) && ((opp_num_major_pieces + (opp_num_minor_pieces/2)) <= 2);
    let gamephase = if cond { GamePhase::Endgame } else { GamePhase::Middlegame };

    // add value for material (note: this value is relative based on location and gamephase)
    eval += bitboard::get_pieces_material_value(bitboard_pieces, gamephase);

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