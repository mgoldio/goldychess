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
    white_pieces: WHITE_START_PIECES,
    black_pieces: BLACK_START_PIECES,
    castling_rights: CastlingRights {
        white_long: true,
        white_short: true,
        black_long: true,
        black_short: true
    },
    turn: Color::White
};

// Functions

// get_piece_at_mut allows us to grab a mutable copy of the pieces associated with a move
// without locking down the whole board reference
pub fn get_pieces_for_move_mut <'a> (
    white_pieces : &'a mut [Piece; 16],
    black_pieces : &'a mut [Piece; 16],
    m: Move
) -> (Option<&'a mut Piece>, Option<&'a mut Piece>) {
    let mut from_piece: Option<&mut Piece> = None;
    let mut to_piece: Option<&mut Piece> = None;
    for p in white_pieces.iter_mut() {
        if p.square == m.from_square && p.piece_type != PieceType::Null {
            from_piece = Some(p);
        } else if p.square == m.to_square && p.piece_type != PieceType::Null {
            to_piece = Some(p);
        }
    }
    for p in black_pieces.iter_mut() {
        if p.square == m.from_square && p.piece_type != PieceType::Null {
            from_piece = Some(p);
        } else if p.square == m.to_square && p.piece_type != PieceType::Null {
            to_piece = Some(p);
        }
    }
    return (from_piece, to_piece);
}

pub fn apply_null_move(b : &Board) -> Board {
    let mut board = *b;
    board.turn = if board.turn == Color::White {Color::Black} else {Color::White};
    return board;
}

pub fn apply_move(b : &Board, m : Move) -> Board {
    let mut board = *b;

    // first, clear all the from squares and the to squares on our bitboard
    let to_bitboard = bitboard::square_to_bitboard(m.to_square);
    let mask_from = !bitboard::square_to_bitboard(m.from_square);
    let mask_to = !to_bitboard;
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

    // we use get_piece_at_mut so we can easily hold on to this and modify it to have the correct square
    let (fp, tp) = get_pieces_for_move_mut(&mut board.white_pieces, &mut board.black_pieces, m);
    let piece = match fp {
        Some(p) => p,
        _ => {
            println!("ERROR: invalid move: {:?}", m);
            return *b;
        }
    };

    // mark any captured pieces as captured
    match tp {
        Some(p) => {
            if (p.piece_type == PieceType::Rook) {
                // if a rook was captured, we potentially need to clear castling rights
                match p.square {
                    Square::A1 => board.castling_rights.white_long = false,
                    Square::H1 => board.castling_rights.white_short = false,
                    Square::A8 => board.castling_rights.black_long = false,
                    Square::H8 => board.castling_rights.black_short = false,
                    _ => { }
                }
            }
            p.piece_type = PieceType::Null
        },
        _ => { }
    };

    // Promote pieces
    if (m.promote_type != PieceType::Null) {
        piece.piece_type = m.promote_type;
    }
    // TODO: do we actually need to support this? well-formed UCI should include the promote type
    // let back_rank_bitboard = if piece.color == Color::White {bitboard::RANK_8} else {bitboard::RANK_1};
    // if (piece.piece_type == PieceType::Pawn) && ((to_bitboard & back_rank_bitboard) != 0) {
    //         piece.piece_type = promote_type;
    // }


    // next, let's set the bitboard for where it moved to
    match (piece.color, piece.piece_type) {
        (Color::White, PieceType::King) => board.white_bitboard_pieces.king |= to_bitboard,
        (Color::White, PieceType::Queen) => board.white_bitboard_pieces.queens |= to_bitboard,
        (Color::White, PieceType::Rook) => board.white_bitboard_pieces.rooks |= to_bitboard,
        (Color::White, PieceType::Bishop) => board.white_bitboard_pieces.bishops |= to_bitboard,
        (Color::White, PieceType::Knight) => board.white_bitboard_pieces.knights |= to_bitboard,
        (Color::White, PieceType::Pawn) => board.white_bitboard_pieces.pawns |= to_bitboard,
        (Color::Black, PieceType::King) => board.black_bitboard_pieces.king |= to_bitboard,
        (Color::Black, PieceType::Queen) => board.black_bitboard_pieces.queens |= to_bitboard,
        (Color::Black, PieceType::Rook) => board.black_bitboard_pieces.rooks |= to_bitboard,
        (Color::Black, PieceType::Bishop) => board.black_bitboard_pieces.bishops |= to_bitboard,
        (Color::Black, PieceType::Knight) => board.black_bitboard_pieces.knights |= to_bitboard,
        (Color::Black, PieceType::Pawn) => board.black_bitboard_pieces.pawns |= to_bitboard,
        _ => {
            println!("ERROR: invalid move: {:?}", m);
            return *b;
        }
    }

    // clear any lost castling privileges
    if piece.piece_type == PieceType::King {
        // any king move clears castling privileges
        if (piece.color == Color::White) {
            board.castling_rights.white_long = false;
            board.castling_rights.white_short = false;
        } else {
            board.castling_rights.black_long = false;
            board.castling_rights.black_short = false;
        }
    } else if piece.piece_type == PieceType::Rook {
        // if a rook moves, it clears castling privileges for its side
        match piece.square {
            Square::A1 => board.castling_rights.white_long = false,
            Square::H1 => board.castling_rights.white_short = false,
            Square::A8 => board.castling_rights.black_long = false,
            Square::H8 => board.castling_rights.black_short = false,
            _ => { }
        }
    }

    // update the piece's square
    piece.square = m.to_square;

    // save off the next board turn here - castling makes pseudomoves which will change the color
    let next_turn = if board.turn == Color::White {Color::Black} else {Color::White};

    // handle castling
    if (piece.piece_type == PieceType::King) {
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