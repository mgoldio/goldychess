use crate::bitboard;
use crate::types;
use crate::types::{Piece, Color, PieceType, Square, CastlingRights, Board, Move};

const WHITE_START_PIECES: [Piece; 16] = [
    Piece{color: Color::White, piece_type: PieceType::King, square: Square::E1},
    Piece{color: Color::White, piece_type: PieceType::Queen, square: Square::D1},
    Piece{color: Color::White, piece_type: PieceType::Rook, square: Square::A1},
    Piece{color: Color::White, piece_type: PieceType::Rook, square: Square::H1},
    Piece{color: Color::White, piece_type: PieceType::Bishop, square: Square::C1},
    Piece{color: Color::White, piece_type: PieceType::Bishop, square: Square::F1},
    Piece{color: Color::White, piece_type: PieceType::Knight, square: Square::B1},
    Piece{color: Color::White, piece_type: PieceType::Knight, square: Square::G1},
    Piece{color: Color::White, piece_type: PieceType::Pawn, square: Square::A2},
    Piece{color: Color::White, piece_type: PieceType::Pawn, square: Square::B2},
    Piece{color: Color::White, piece_type: PieceType::Pawn, square: Square::C2},
    Piece{color: Color::White, piece_type: PieceType::Pawn, square: Square::D2},
    Piece{color: Color::White, piece_type: PieceType::Pawn, square: Square::E2},
    Piece{color: Color::White, piece_type: PieceType::Pawn, square: Square::F2},
    Piece{color: Color::White, piece_type: PieceType::Pawn, square: Square::G2},
    Piece{color: Color::White, piece_type: PieceType::Pawn, square: Square::H2}
];

const BLACK_START_PIECES: [Piece; 16] = [
    Piece{color: Color::Black, piece_type: PieceType::King, square: Square::E8},
    Piece{color: Color::Black, piece_type: PieceType::Queen, square: Square::D8},
    Piece{color: Color::Black, piece_type: PieceType::Rook, square: Square::A8},
    Piece{color: Color::Black, piece_type: PieceType::Rook, square: Square::H8},
    Piece{color: Color::Black, piece_type: PieceType::Bishop, square: Square::C8},
    Piece{color: Color::Black, piece_type: PieceType::Bishop, square: Square::F8},
    Piece{color: Color::Black, piece_type: PieceType::Knight, square: Square::B8},
    Piece{color: Color::Black, piece_type: PieceType::Knight, square: Square::G8},
    Piece{color: Color::Black, piece_type: PieceType::Pawn, square: Square::A7},
    Piece{color: Color::Black, piece_type: PieceType::Pawn, square: Square::B7},
    Piece{color: Color::Black, piece_type: PieceType::Pawn, square: Square::C7},
    Piece{color: Color::Black, piece_type: PieceType::Pawn, square: Square::D7},
    Piece{color: Color::Black, piece_type: PieceType::Pawn, square: Square::E7},
    Piece{color: Color::Black, piece_type: PieceType::Pawn, square: Square::F7},
    Piece{color: Color::Black, piece_type: PieceType::Pawn, square: Square::G7},
    Piece{color: Color::Black, piece_type: PieceType::Pawn, square: Square::H7}
];

pub const START_POSITION: Board = Board {
    white_bitboard_pieces: bitboard::WHITE_START,
    black_bitboard_pieces: bitboard::BLACK_START,
    castling_rights: CastlingRights {
        white_long: true,
        white_short: true,
        black_long: true,
        black_short: true
    },
    turn: Color::White
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
    let mask_from = !from_bitboard;
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
        return *b;
    }

    if (enemy_pieces.rooks & to_bitboard) != 0 {
        // if a rook was captured, we potentially need to clear castling rights
        match m.from_square {
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

    // save off the next board turn here - castling makes pseudomoves which will change the color
    let next_turn = if board.turn == Color::White {Color::Black} else {Color::White};

    // handle castling
    if (piece_type == PieceType::King) {
        // everything should automatically be handled elsewhere except moving the rook...
        // to properly handle that we simply recurse to generate a pseudo-rook-move prior to returning
        match (m.from_square, m.to_square) {
            (Square::E1, Square::G1) => {
                let rook_move = Move {from_square: Square::H1, to_square: Square::F1, promote_type: PieceType::Null};
                board = apply_move(&board, rook_move);
            }
            (Square::E1, Square::C1) => {
                let rook_move = Move {from_square: Square::A1, to_square: Square::D1, promote_type: PieceType::Null};
                board = apply_move(&board, rook_move);
            }
            (Square::E8, Square::G8) => {
                let rook_move = Move {from_square: Square::H8, to_square: Square::F8, promote_type: PieceType::Null};
                board = apply_move(&board, rook_move);
            }
            (Square::E8, Square::C8) => {
                let rook_move = Move {from_square: Square::A8, to_square: Square::D8, promote_type: PieceType::Null};
                board = apply_move(&board, rook_move);
            }
            _ => { }
        }
    }

    // update the board's color
    board.turn = next_turn;

    // and, we're done!
    return board;
}