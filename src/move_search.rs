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

pub fn calc_pmoves(b: &Board, exclude_castles: bool) -> Vec<Move> {
    let mut capscasts = Vec::<Move>::new(); // captures and castles
    let mut moves = Vec::<Move>::new(); // other moves

    let pieces = if b.turn == Color::White { b.white_bitboard_pieces } else { b.black_bitboard_pieces };
    let enemy_pieces = if b.turn == Color::White { b.black_bitboard_pieces } else { b.white_bitboard_pieces };
    let pieces_rel = bitboard::get_bitboard_pieces_rel(pieces, b.turn);
    let friendly_bitboard_rel = pieces_rel.king
        | pieces_rel.queens
        | pieces_rel.rooks
        | pieces_rel.bishops
        | pieces_rel.knights
        | pieces_rel.pawns;
    let enemy_bitboard = enemy_pieces.king
        | enemy_pieces.queens
        | enemy_pieces.rooks
        | enemy_pieces.bishops
        | enemy_pieces.knights
        | enemy_pieces.pawns;
    let enemy_bitboard_rel = bitboard::get_bitboard_rel(enemy_bitboard, b.turn);
    let all_bitboard_rel = friendly_bitboard_rel | enemy_bitboard_rel;

    // pawns
    {
        // pawn pushing
        {
            let moves_bitboard = bitboard::slide(pieces_rel.pawns, Direction::N, 1) & !all_bitboard_rel;
            let mut bb = moves_bitboard;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(Direction::S, 1).unwrap();
                let promote_type = if (move_bb & bitboard::RANK_8) != 0 { PieceType::Queen } else { PieceType::Null };
                moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: promote_type});
                bb = bb & !move_bb;
            }
            // if a pawn can move 1 square, it might be able to move 2
            {
                let moves2_bitboard = bitboard::slide((moves_bitboard & bitboard::RANK_3), Direction::N, 1) & !all_bitboard_rel;
                bb = moves2_bitboard;
                while bb != 0 {
                    let idx = bb.trailing_zeros();
                    let move_bb = bitboard::bitboard_from_index(idx);
                    let to_square = Square::from_index(idx).unwrap();
                    let from_square = to_square.slide(Direction::S, 2).unwrap();
                    moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                    bb = bb & !move_bb;
                }
            }
        }
        // pawn capturing NW
        {
            let moves_bitboard = bitboard::slide(pieces_rel.pawns, Direction::NW, 1) & enemy_bitboard_rel;
            let mut bb = moves_bitboard;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(Direction::SE, 1).unwrap();
                let promote_type = if (move_bb & bitboard::RANK_8) != 0 { PieceType::Queen } else { PieceType::Null };
                moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: promote_type});
                bb = bb & !move_bb;
            }
        }
        // pawn capturing NE
        {
            let moves_bitboard = bitboard::slide(pieces_rel.pawns, Direction::NE, 1) & enemy_bitboard_rel;
            let mut bb = moves_bitboard;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(Direction::SW, 1).unwrap();
                let promote_type = if (move_bb & bitboard::RANK_8) != 0 { PieceType::Queen } else { PieceType::Null };
                moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: promote_type});
                bb = bb & !move_bb;
            }
        }
    }

    // knights
    for &kh in types::KNIGHT_HOPS.iter() {
        // captures
        {
            let moves_bitboard = bitboard::knight_hop(pieces_rel.knights, kh) & enemy_bitboard_rel;
            let mut bb = moves_bitboard;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.knight_hop(kh.reverse()).unwrap();
                moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                bb = bb & !move_bb;
            }
        }
        // non-capture knight moves
        {
            let moves_bitboard = bitboard::knight_hop(pieces_rel.knights, kh) & !all_bitboard_rel;
            let mut bb = moves_bitboard;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.knight_hop(kh.reverse()).unwrap();
                
                moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                bb = bb & !move_bb;
            }
        }
    }

    // bishops
    for &dir in types::BISHOP_DIRECTIONS.iter() {
        let mut bitboard = pieces_rel.bishops;
        for dist in 1..8 {
            bitboard = bitboard::slide(bitboard, dir, 1);

            // captures
            let cap_moves = bitboard & enemy_bitboard_rel;
            let mut bb = cap_moves;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(dir.reverse(), dist).unwrap();
                capscasts.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                bb = bb & !move_bb;
            }

            // non-capturing moves
            let open_moves = bitboard & !all_bitboard_rel;
            let mut bb = open_moves;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(dir.reverse(), dist).unwrap();
                moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                bb = bb & !move_bb;
            }
            if open_moves == 0 {
                break;
            }

            bitboard = open_moves; // update to use open_moves here since a piece can't slide past a square where it captures
        }
    }

    // queens
    for &dir in types::QUEEN_DIRECTIONS.iter() {
        let mut bitboard = pieces_rel.queens;
        for dist in 1..8 {
            bitboard = bitboard::slide(bitboard, dir, 1);

            // captures
            let cap_moves = bitboard & enemy_bitboard_rel;
            let mut bb = cap_moves;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(dir.reverse(), dist).unwrap();
                capscasts.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                bb = bb & !move_bb;
            }

            // non-capturing moves
            let open_moves = bitboard & !all_bitboard_rel;
            let mut bb = open_moves;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(dir.reverse(), dist).unwrap();
                moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                bb = bb & !move_bb;
            }
            if open_moves == 0 {
                break;
            }

            bitboard = open_moves; // update to use open_moves here since a piece can't slide past a square where it captures
        }
    }

    // rooks
    for &dir in types::ROOK_DIRECTIONS.iter() {
        let mut bitboard = pieces_rel.rooks;
        for dist in 1..8 {
            bitboard = bitboard::slide(bitboard, dir, 1);

            // captures
            let cap_moves = bitboard & enemy_bitboard_rel;
            let mut bb = cap_moves;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(dir.reverse(), dist).unwrap();
                capscasts.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                bb = bb & !move_bb;
            }

            // non-capturing moves
            let open_moves = bitboard & !all_bitboard_rel;
            let mut bb = open_moves;
            while bb != 0 {
                let idx = bb.trailing_zeros();
                let move_bb = bitboard::bitboard_from_index(idx);
                let to_square = Square::from_index(idx).unwrap();
                let from_square = to_square.slide(dir.reverse(), dist).unwrap();
                moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
                bb = bb & !move_bb;
            }
            if open_moves == 0 {
                break;
            }

            bitboard = open_moves; // update to use open_moves here since a piece can't slide past a square where it captures
        }
    }

    // king
    for &dir in types::KING_DIRECTIONS.iter() {
        let mut bitboard = pieces_rel.king;
        bitboard = bitboard::slide(bitboard, dir, 1);

        // captures
        let cap_moves = bitboard & enemy_bitboard_rel;
        let mut bb = cap_moves;
        while bb != 0 {
            let idx = bb.trailing_zeros();
            let move_bb = bitboard::bitboard_from_index(idx);
            let to_square = Square::from_index(idx).unwrap();
            let from_square = to_square.slide(dir.reverse(), 1).unwrap();
            capscasts.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
            bb = bb & !move_bb;
        }

        // non-capturing moves
        let open_moves = bitboard & !all_bitboard_rel;
        let mut bb = open_moves;
        while bb != 0 {
            let idx = bb.trailing_zeros();
            let move_bb = bitboard::bitboard_from_index(idx);
            let to_square = Square::from_index(idx).unwrap();
            let from_square = to_square.slide(dir.reverse(), 1).unwrap();
            moves.push(Move {from_square: from_square.rel(b.turn), to_square: to_square.rel(b.turn), promote_type: PieceType::Null});
            bb = bb & !move_bb;
        }
    }
    if !exclude_castles {
        let can_castle_long = if b.turn == Color::White { b.castling_rights.white_long } else { b.castling_rights.black_long };
        let can_castle_short = if b.turn == Color::White { b.castling_rights.white_short } else { b.castling_rights.black_short };
        // if we still have long castling rights and no pieces are in the way, check further
        if can_castle_long && ((all_bitboard_rel & bitboard::LONG_CASTLE_BITBOARD) == 0) {
            let null_board = utils::apply_null_move(b);
            if test_pmoves(&null_board) { // we can't castle if we are in check
                let from_sq = Square::E1.rel(b.turn);
                let to_sq1 = Square::D1.rel(b.turn);
                let test_move = Move {from_square: from_sq, to_square: to_sq1, promote_type: PieceType::Null};

                // we also can't castle through check so we need to check the square we are castling through
                // we don't test the square where the king will end up because it will be checked later by calc_moves
                let test_board = utils::apply_move(b, test_move);
                if test_pmoves(&test_board) {
                    let to_sq2 = Square::C1.rel(b.turn);
                    let castle_move = Move {from_square: from_sq, to_square: to_sq2, promote_type: PieceType::Null};
                    capscasts.push(castle_move);
                }
            }
        }
        // if we still have short castling rights and no pieces are in the way, check further
        if can_castle_short && ((all_bitboard_rel & bitboard::SHORT_CASTLE_BITBOARD) == 0) {
            let null_board = utils::apply_null_move(b);
            if test_pmoves(&null_board) { // we can't castle if we are in check
                let from_sq = Square::E1.rel(b.turn);
                let to_sq1 = Square::F1.rel(b.turn);
                let test_move = Move {from_square: from_sq, to_square: to_sq1, promote_type: PieceType::Null};

                // we also can't castle through check so we need to check the square we are castling through
                // we don't test the square where the king will end up because it will be checked later by calc_moves
                let test_board = utils::apply_move(b, test_move);
                if test_pmoves(&test_board) {
                    let to_sq2 = Square::G1.rel(b.turn);
                    let castle_move = Move {from_square: from_sq, to_square: to_sq2, promote_type: PieceType::Null};
                    capscasts.push(castle_move);
                }
            }
        }
    }

    capscasts.extend(moves);
    return capscasts;
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