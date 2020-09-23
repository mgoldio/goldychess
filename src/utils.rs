use crate::bitboard;
use crate::types;
use crate::types::{Color, PieceType, Square, CastlingRights, Board, Move, Direction};

pub const START_POSITION: Board = Board {
    white_bitboard_pieces: bitboard::WHITE_START,
    black_bitboard_pieces: bitboard::BLACK_START,
    turn: Color::White,
    castling_rights: CastlingRights {
        white_long: true,
        white_short: true,
        black_long: true,
        black_short: true
    },
    enpassant_files: bitboard::EMPTY_BITRANK,
    all_piece_history: [0u64, 1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64, 11u64, 12u64, 13u64, 14u64, 15u64],
    all_ptr: 0usize
};

// Functions

pub fn apply_null_move(b : &Board) -> Board {
    let mut board = *b;
    board.turn = if board.turn == Color::White {Color::Black} else {Color::White};
    return board;
}

pub fn apply_move(b : &Board, m : Move) -> Board {
    let mut board = *b;

    let friendly_pieces = if b.turn == Color::White { &board.white_bitboard_pieces } else { &board.black_bitboard_pieces };
    let enemy_pieces = if b.turn == Color::White { &board.black_bitboard_pieces } else { &board.white_bitboard_pieces };

    let to_bitboard = m.to_square.to_bitboard();
    let from_bitboard = m.from_square.to_bitboard();
    let mut mask_from = !from_bitboard;
    let mask_to = !to_bitboard;

    let mut piece_type = PieceType::Null;
    if (friendly_pieces.king & from_bitboard) != 0 {
        piece_type = PieceType::King;
    } else if (friendly_pieces.queens & from_bitboard) != 0 {
        piece_type = PieceType::Queen;
    } else if (friendly_pieces.rooks & from_bitboard) != 0 {
        piece_type = PieceType::Rook;
    } else if (friendly_pieces.bishops & from_bitboard) != 0 {
        piece_type = PieceType::Bishop;
    } else if (friendly_pieces.knights & from_bitboard) != 0 {
        piece_type = PieceType::Knight;
    } else if (friendly_pieces.pawns & from_bitboard) != 0 {
        piece_type = PieceType::Pawn;
    } else {
        println!("ERROR: invalid move: {:?}", m);
        b.pretty_print();
        return *b;
    }

    // handle enpassant captures and flag setting
    if piece_type == PieceType::Pawn {
        // captures
        if b.turn == Color::White && ((to_bitboard >> 8*5) & (board.enpassant_files as u64)) != 0 {
            // if this condition is true, there must have been an enpassant capture on rank 6
            // we'll clear the pawn by clearing the bit for the captured pawn in mask_from
            mask_from &= !bitboard::slide1(to_bitboard, Direction::S);
        }
        if b.turn == Color::Black && ((to_bitboard >> 8*2) & (board.enpassant_files as u64)) != 0 {
            // if this condition is true, there must have been an enpassant capture on rank 3
            // we'll clear the pawn by clearing the bit for the captured pawn in mask_from
            mask_from &= !bitboard::slide1(to_bitboard, Direction::N);
        }

        // flag setting
        board.enpassant_files = bitboard::EMPTY_BITRANK; // this gets cleared every move
        if ((to_bitboard & bitboard::RANK_4) != 0) && ((from_bitboard & bitboard::RANK_2) != 0) {
            board.enpassant_files = ((from_bitboard >> 8) & 0xFF) as bitboard::Bitrank;
        }
        if ((to_bitboard & bitboard::RANK_5) != 0) && ((from_bitboard & bitboard::RANK_7) != 0) {
            board.enpassant_files = ((from_bitboard >> 8*6) & 0xFF) as bitboard::Bitrank;
        }
    }

    if (enemy_pieces.rooks & to_bitboard) != 0 {
        // if a rook was captured, we potentially need to clear castling rights
        match m.to_square {
            Square::A1 => board.castling_rights.white_long = false,
            Square::H1 => board.castling_rights.white_short = false,
            Square::A8 => board.castling_rights.black_long = false,
            Square::H8 => board.castling_rights.black_short = false,
            _ => { }
        }
    }

    // clear any lost castling privileges
    if piece_type == PieceType::King {
        // any king move clears castling privileges
        if (b.turn == Color::White) {
            board.castling_rights.white_long = false;
            board.castling_rights.white_short = false;
        } else {
            board.castling_rights.black_long = false;
            board.castling_rights.black_short = false;
        }
    } else if piece_type == PieceType::Rook {
        // if a rook moves, it clears castling privileges for its side
        match m.from_square {
            Square::A1 => board.castling_rights.white_long = false,
            Square::H1 => board.castling_rights.white_short = false,
            Square::A8 => board.castling_rights.black_long = false,
            Square::H8 => board.castling_rights.black_short = false,
            _ => { }
        }
    }

    // save off the next board turn here - castling makes pseudomoves which will change the color
    let next_turn = if b.turn == Color::White {Color::Black} else {Color::White};

    // handle castling
    if piece_type == PieceType::King {
        // everything should automatically be handled elsewhere except moving the rook...
        // to properly handle that we simply recurse to generate a pseudo-rook-move prior to returning
        match (m.from_square, m.to_square) {
            (Square::E1, Square::G1) => {
                let rook_move = Move {from_square: Square::H1, to_square: Square::F1, promote_type: PieceType::Null};
                board = apply_move(&board, rook_move);
                board.all_ptr = (board.all_ptr-1) % 16;
            }
            (Square::E1, Square::C1) => {
                let rook_move = Move {from_square: Square::A1, to_square: Square::D1, promote_type: PieceType::Null};
                board = apply_move(&board, rook_move);
                board.all_ptr = (board.all_ptr-1) % 16;
            }
            (Square::E8, Square::G8) => {
                let rook_move = Move {from_square: Square::H8, to_square: Square::F8, promote_type: PieceType::Null};
                board = apply_move(&board, rook_move);
                board.all_ptr = (board.all_ptr-1) % 16;
            }
            (Square::E8, Square::C8) => {
                let rook_move = Move {from_square: Square::A8, to_square: Square::D8, promote_type: PieceType::Null};
                board = apply_move(&board, rook_move);
                board.all_ptr = (board.all_ptr-1) % 16;
            }
            _ => { }
        }
    }

    // clear all the from squares and the to squares on our bitboard
    board.white_bitboard_pieces.king &= mask_from & mask_to;
    board.white_bitboard_pieces.queens &= mask_from & mask_to;
    board.white_bitboard_pieces.rooks &= mask_from & mask_to;
    board.white_bitboard_pieces.bishops &= mask_from & mask_to;
    board.white_bitboard_pieces.knights &= mask_from & mask_to;
    board.white_bitboard_pieces.pawns &= mask_from & mask_to;
    board.black_bitboard_pieces.king &= mask_from & mask_to;
    board.black_bitboard_pieces.queens &= mask_from & mask_to;
    board.black_bitboard_pieces.rooks &= mask_from & mask_to;
    board.black_bitboard_pieces.bishops &= mask_from & mask_to;
    board.black_bitboard_pieces.knights &= mask_from & mask_to;
    board.black_bitboard_pieces.pawns &= mask_from & mask_to;

    // next, let's set the bitboard for where it moved to
    if b.turn == Color::White {
        match piece_type {
            PieceType::King => board.white_bitboard_pieces.king |= to_bitboard,
            PieceType::Queen => board.white_bitboard_pieces.queens |= to_bitboard,
            PieceType::Rook => board.white_bitboard_pieces.rooks |= to_bitboard,
            PieceType::Bishop => board.white_bitboard_pieces.bishops |= to_bitboard,
            PieceType::Knight => board.white_bitboard_pieces.knights |= to_bitboard,
            PieceType::Pawn => {
                match m.promote_type {
                    // handle promotions
                    PieceType::Queen => board.white_bitboard_pieces.queens |= to_bitboard,
                    PieceType::Rook => board.white_bitboard_pieces.rooks |= to_bitboard,
                    PieceType::Bishop => board.white_bitboard_pieces.bishops |= to_bitboard,
                    PieceType::Knight => board.white_bitboard_pieces.knights |= to_bitboard,
                    _ => board.white_bitboard_pieces.pawns |= to_bitboard
                }
            },
            _ => { }
        }
    } else {
        match piece_type {
            PieceType::King => board.black_bitboard_pieces.king |= to_bitboard,
            PieceType::Queen => board.black_bitboard_pieces.queens |= to_bitboard,
            PieceType::Rook => board.black_bitboard_pieces.rooks |= to_bitboard,
            PieceType::Bishop => board.black_bitboard_pieces.bishops |= to_bitboard,
            PieceType::Knight => board.black_bitboard_pieces.knights |= to_bitboard,
            PieceType::Pawn => {
                match m.promote_type {
                    // handle promotions
                    PieceType::Queen => board.black_bitboard_pieces.queens |= to_bitboard,
                    PieceType::Rook => board.black_bitboard_pieces.rooks |= to_bitboard,
                    PieceType::Bishop => board.black_bitboard_pieces.bishops |= to_bitboard,
                    PieceType::Knight => board.black_bitboard_pieces.knights |= to_bitboard,
                    _ => board.black_bitboard_pieces.pawns |= to_bitboard
                }
            },
            _ => { }
        }
    }
    
    // set the "all" bitboards
    board.white_bitboard_pieces.all = board.white_bitboard_pieces.king
        | board.white_bitboard_pieces.queens
        | board.white_bitboard_pieces.rooks
        | board.white_bitboard_pieces.bishops
        | board.white_bitboard_pieces.knights
        | board.white_bitboard_pieces.pawns;
    board.black_bitboard_pieces.all = board.black_bitboard_pieces.king
        | board.black_bitboard_pieces.queens
        | board.black_bitboard_pieces.rooks
        | board.black_bitboard_pieces.bishops
        | board.black_bitboard_pieces.knights
        | board.black_bitboard_pieces.pawns;
    
    // set the "all piece history"
    let all_pieces = board.white_bitboard_pieces.all | board.black_bitboard_pieces.all;
    board.all_piece_history[board.all_ptr] = all_pieces;
    board.all_ptr = (board.all_ptr+1) % 16;

    // update the board's color
    board.turn = next_turn;

    // and, we're done!
    return board;
}