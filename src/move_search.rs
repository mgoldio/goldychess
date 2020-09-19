use crate::types;
use crate::types::{Direction, Piece, Color, PieceType, Square, Rank, File, CastlingRights, Board, Move};
use crate::bitboard;
use crate::utils;

pub fn calc_moves(b : &Board) -> Vec<Move> {
    let mut vec = Vec::<Move>::new();
    for &m in calc_pmoves(b, false).iter() {
        let test_board = utils::apply_move(b, m);
        if test_pmoves(&test_board) {
            vec.push(m);
        }
    }
    return vec;
    //return calc_pmoves(b);
}

pub fn calc_pmoves(b : &Board, exclude_castles : bool) -> Vec<Move> {
    let pieces : [Piece; 16] = if b.turn == Color::White { b.white_pieces } else { b.black_pieces };
    let mut vec = Vec::<Move>::new();
    for p in pieces.iter() {
        let piece_moves = calc_pmoves_piece(b, p, exclude_castles);
        vec.extend(piece_moves);
    }
    return vec;
}

pub fn calc_pmoves_piece(b : &Board, p : &Piece, exclude_castles : bool) -> Vec<Move> {
    let mut vec = Vec::<Move>::new();
    let bitboard = bitboard::square_to_bitboard(p.square);
    let bitboard_rel = bitboard::get_bitboard_rel(bitboard, b.turn);
    let white_bitboard = b.white_bitboard_pieces.king 
        | b.white_bitboard_pieces.queens 
        | b.white_bitboard_pieces.rooks 
        | b.white_bitboard_pieces.bishops
        | b.white_bitboard_pieces.knights
        | b.white_bitboard_pieces.pawns;
    let black_bitboard = b.black_bitboard_pieces.king 
        | b.black_bitboard_pieces.queens 
        | b.black_bitboard_pieces.rooks 
        | b.black_bitboard_pieces.bishops
        | b.black_bitboard_pieces.knights
        | b.black_bitboard_pieces.pawns;
    let all_bitboard = white_bitboard | black_bitboard; // bitboard of all pieces
    let friendly_bitboard = if b.turn == Color::White { white_bitboard } else { black_bitboard }; // bitboard of friendly pieces
    let enemy_bitboard = if b.turn == Color::White { black_bitboard } else { white_bitboard }; // bitboard of enemy pieces
    let enemy_bitboard_rel = bitboard::get_bitboard_rel(enemy_bitboard, b.turn);
    let all_bitboard_rel = bitboard::get_bitboard_rel(all_bitboard, b.turn); // bitboard of all pieces but flipped such that Rank 1 is always the current color's back rank
    match p.piece_type {
        PieceType::King => {
            for &dir in types::KING_DIRECTIONS.iter() {
                let moved_bitboard = bitboard::slide(bitboard, dir, 1);
                if (moved_bitboard & friendly_bitboard) == 0 {
                    let to_sq = p.square.slide(dir, 1);
                    if !to_sq.is_none() {
                        vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: PieceType::Null});
                    }
                }
            }
            if !exclude_castles {
                let can_castle_long = if b.turn == Color::White { b.castling_rights.white_long } else { b.castling_rights.black_long };
                let can_castle_short = if b.turn == Color::White { b.castling_rights.white_short } else { b.castling_rights.black_short };
                // if we still have long castling rights and no pieces are in the way, check further
                if can_castle_long && ((all_bitboard_rel & bitboard::LONG_CASTLE_BITBOARD) == 0) {
                    let null_board = utils::apply_null_move(b);
                    if test_pmoves(&null_board) {
                        let to_sq = p.square.slide(Direction::W, 2);
                        // because we can't castle through check, we must check that the square just west is not a check square
                        // we don't test the square where the king will end up because it will be checked later by calc_moves
                        let test_sq = p.square.slide(Direction::W, 1);
                        if !test_sq.is_none() && !to_sq.is_none() {
                            let test_move = Move {from_square: p.square, to_square: test_sq.unwrap(), promote_type: PieceType::Null};
                            let test_board = utils::apply_move(b, test_move);
                            if test_pmoves(&test_board) {
                                vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: PieceType::Null});
                            }
                        }
                    }
                }
                // if we still have short castling rights and no pieces are in the way, check further
                if can_castle_short && ((all_bitboard_rel & bitboard::SHORT_CASTLE_BITBOARD) == 0) {
                    let null_board = utils::apply_null_move(b);
                    if test_pmoves(&null_board) {
                        let to_sq = p.square.slide(Direction::E, 2);
                        // because we can't castle through check, we must check that the square just east is not a check square
                        // we don't test the square where the king will end up because it will be checked later by calc_moves
                        let test_sq = p.square.slide(Direction::E, 1);
                        if !test_sq.is_none() && !to_sq.is_none() {
                            let test_move = Move {from_square: p.square, to_square: test_sq.unwrap(), promote_type: PieceType::Null};
                            let test_board = utils::apply_move(b, test_move);
                            if test_pmoves(&test_board) {
                                vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: PieceType::Null});
                            }
                        }
                    }
                }
            }
        },
        PieceType::Queen => {
            for &dir in types::QUEEN_DIRECTIONS.iter() {
                for dist in 1..8 {
                    let moved_bitboard = bitboard::slide(bitboard, dir, dist);
                    if (moved_bitboard & friendly_bitboard) == 0 {
                        let to_sq = p.square.slide(dir, dist);
                        if !to_sq.is_none() {
                            vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: PieceType::Null});
                        }
                        if (moved_bitboard & enemy_bitboard) != 0 {
                            // if it's a capture, stop scanning further in this direction
                            break;
                        }
                    } else {
                        // if we're blocked by a friendly p, stop scanning further in this direction
                        break;
                    }
                }
            }
        },
        PieceType::Rook => {
            for &dir in types::ROOK_DIRECTIONS.iter() {
                for dist in 1..8 {
                    let moved_bitboard = bitboard::slide(bitboard, dir, dist);
                    if (moved_bitboard & friendly_bitboard) == 0 {
                        let to_sq = p.square.slide(dir, dist);
                        if !to_sq.is_none() {
                            vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: PieceType::Null});
                        }
                        if (moved_bitboard & enemy_bitboard) != 0 {
                            // if it's a capture, stop scanning further in this direction
                            break;
                        }
                    } else {
                        // if we're blocked by a friendly p, stop scanning further in this direction
                        break;
                    }
                }
            }
        },
        PieceType::Bishop => {
            for &dir in types::BISHOP_DIRECTIONS.iter() {
                for dist in 1..8 {
                    let moved_bitboard = bitboard::slide(bitboard, dir, dist);
                    if (moved_bitboard & friendly_bitboard) == 0 {
                        let to_sq = p.square.slide(dir, dist);
                        if !to_sq.is_none() {
                            vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: PieceType::Null});
                        }
                        if (moved_bitboard & enemy_bitboard) != 0 {
                            // if it's a capture, stop scanning further in this direction
                            break;
                        }
                    } else {
                        // if we're blocked by a friendly p, stop scanning further in this direction
                        break;
                    }
                }
            }
        },
        PieceType::Knight => {
            for &kh in types::KNIGHT_HOPS.iter() {
                let moved_bitboard = bitboard::knight_hop(bitboard, kh);
                if (moved_bitboard & friendly_bitboard) == 0 {
                    let to_sq = p.square.knight_hop(kh);
                    if !to_sq.is_none() {
                        vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: PieceType::Null});
                    }
                }
            }
        },
        PieceType::Pawn => {
            let promote_type = if (bitboard_rel & bitboard::RANK_7) != 0 { PieceType::Queen } else { PieceType::Null };
            if (bitboard::slide(bitboard_rel, Direction::N, 1) & all_bitboard_rel) == 0 {
                // if no pieces  are in front of a pawn, it can move there
                let to_sq = p.square.slide(Direction::N.rel(b.turn), 1);
                if !to_sq.is_none() {
                    vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: promote_type});
                }
                // if no pieces are directly in front of a pawn on the second rank, it might be able to move 2 spaces
                if (p.square.get_rank() == Rank::Rank2) {
                    if (bitboard::slide(bitboard_rel, Direction::N, 2) & all_bitboard_rel) == 0 {
                        // no pieces two squares in front; we can double push!
                        let to_sq = p.square.slide(Direction::N.rel(b.turn), 2);
                        if !to_sq.is_none() {
                            vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: promote_type});
                        }
                    }
                }
            }
            if (bitboard::slide(bitboard_rel, Direction::NW, 1) & enemy_bitboard_rel) != 0 {
                // if there is a piece NW to capture, we can move there
                let to_sq = p.square.slide(Direction::NW.rel(b.turn), 1);
                if !to_sq.is_none() {
                    vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: promote_type});
                }
            }
            if (bitboard::slide(bitboard_rel, Direction::NE, 1) & enemy_bitboard_rel) != 0 {
                // if there is a piece NE to capture, we can move there
                let to_sq = p.square.slide(Direction::NE.rel(b.turn), 1);
                if !to_sq.is_none() {
                    vec.push(Move {from_square: p.square, to_square: to_sq.unwrap(), promote_type: promote_type});
                }
            }
        }
        _ => {

        }
    }
    return vec;
}

// returns false if a pmove attacks a king, true otherwise
pub fn test_pmoves(b : &Board) -> bool {
    let pmoves = calc_pmoves(b, true);
    for m in pmoves {
        let dest_bitboard = bitboard::square_to_bitboard(m.to_square);
        let kings = b.black_bitboard_pieces.king | b.white_bitboard_pieces.king;
        if (dest_bitboard & kings) != 0 {
            return false;
        }
    }
    return true;
}