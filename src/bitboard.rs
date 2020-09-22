#![allow(dead_code)]

use crate::eval;
use crate::types;
use crate::types::{Direction, KnightHop, Color, GamePhase};

// types, enums, structs

pub type Bitboard = u64;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pieces {
    pub king: Bitboard,
    pub queens: Bitboard,
    pub rooks: Bitboard,
    pub bishops: Bitboard,
    pub knights: Bitboard,
    pub pawns: Bitboard
}

// constants

pub const FILE_A: Bitboard = 0x0101010101010101u64;
pub const FILE_B: Bitboard = 0x0202020202020202u64;
pub const FILE_C: Bitboard = 0x0404040404040404u64;
pub const FILE_D: Bitboard = 0x0808080808080808u64;
pub const FILE_E: Bitboard = 0x1010101010101010u64;
pub const FILE_F: Bitboard = 0x2020202020202020u64;
pub const FILE_G: Bitboard = 0x4040404040404040u64;
pub const FILE_H: Bitboard = 0x8080808080808080u64;

pub const FILES: [Bitboard; 8] = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H];

pub const RANK_1: Bitboard = 0x00000000000000FFu64;
pub const RANK_2: Bitboard = 0x000000000000FF00u64;
pub const RANK_3: Bitboard = 0x0000000000FF0000u64;
pub const RANK_4: Bitboard = 0x00000000FF000000u64;
pub const RANK_5: Bitboard = 0x000000FF00000000u64;
pub const RANK_6: Bitboard = 0x0000FF0000000000u64;
pub const RANK_7: Bitboard = 0x00FF000000000000u64;
pub const RANK_8: Bitboard = 0xFF00000000000000u64;

pub const SQUARE_A1: Bitboard = 0x0000000000000001u64;
pub const SQUARE_B1: Bitboard = 0x0000000000000002u64;
pub const SQUARE_C1: Bitboard = 0x0000000000000004u64;
pub const SQUARE_D1: Bitboard = 0x0000000000000008u64;
pub const SQUARE_E1: Bitboard = 0x0000000000000010u64;
pub const SQUARE_F1: Bitboard = 0x0000000000000020u64;
pub const SQUARE_G1: Bitboard = 0x0000000000000040u64;
pub const SQUARE_H1: Bitboard = 0x0000000000000080u64;
pub const SQUARE_A2: Bitboard = 0x0000000000000100u64;
pub const SQUARE_B2: Bitboard = 0x0000000000000200u64;
pub const SQUARE_C2: Bitboard = 0x0000000000000400u64;
pub const SQUARE_D2: Bitboard = 0x0000000000000800u64;
pub const SQUARE_E2: Bitboard = 0x0000000000001000u64;
pub const SQUARE_F2: Bitboard = 0x0000000000002000u64;
pub const SQUARE_G2: Bitboard = 0x0000000000004000u64;
pub const SQUARE_H2: Bitboard = 0x0000000000008000u64;
pub const SQUARE_A3: Bitboard = 0x0000000000010000u64;
pub const SQUARE_B3: Bitboard = 0x0000000000020000u64;
pub const SQUARE_C3: Bitboard = 0x0000000000040000u64;
pub const SQUARE_D3: Bitboard = 0x0000000000080000u64;
pub const SQUARE_E3: Bitboard = 0x0000000000100000u64;
pub const SQUARE_F3: Bitboard = 0x0000000000200000u64;
pub const SQUARE_G3: Bitboard = 0x0000000000400000u64;
pub const SQUARE_H3: Bitboard = 0x0000000000800000u64;
pub const SQUARE_A4: Bitboard = 0x0000000001000000u64;
pub const SQUARE_B4: Bitboard = 0x0000000002000000u64;
pub const SQUARE_C4: Bitboard = 0x0000000004000000u64;
pub const SQUARE_D4: Bitboard = 0x0000000008000000u64;
pub const SQUARE_E4: Bitboard = 0x0000000010000000u64;
pub const SQUARE_F4: Bitboard = 0x0000000020000000u64;
pub const SQUARE_G4: Bitboard = 0x0000000040000000u64;
pub const SQUARE_H4: Bitboard = 0x0000000080000000u64;
pub const SQUARE_A5: Bitboard = 0x0000000100000000u64;
pub const SQUARE_B5: Bitboard = 0x0000000200000000u64;
pub const SQUARE_C5: Bitboard = 0x0000000400000000u64;
pub const SQUARE_D5: Bitboard = 0x0000000800000000u64;
pub const SQUARE_E5: Bitboard = 0x0000001000000000u64;
pub const SQUARE_F5: Bitboard = 0x0000002000000000u64;
pub const SQUARE_G5: Bitboard = 0x0000004000000000u64;
pub const SQUARE_H5: Bitboard = 0x0000008000000000u64;
pub const SQUARE_A6: Bitboard = 0x0000010000000000u64;
pub const SQUARE_B6: Bitboard = 0x0000020000000000u64;
pub const SQUARE_C6: Bitboard = 0x0000040000000000u64;
pub const SQUARE_D6: Bitboard = 0x0000080000000000u64;
pub const SQUARE_E6: Bitboard = 0x0000100000000000u64;
pub const SQUARE_F6: Bitboard = 0x0000200000000000u64;
pub const SQUARE_G6: Bitboard = 0x0000400000000000u64;
pub const SQUARE_H6: Bitboard = 0x0000800000000000u64;
pub const SQUARE_A7: Bitboard = 0x0001000000000000u64;
pub const SQUARE_B7: Bitboard = 0x0002000000000000u64;
pub const SQUARE_C7: Bitboard = 0x0004000000000000u64;
pub const SQUARE_D7: Bitboard = 0x0008000000000000u64;
pub const SQUARE_E7: Bitboard = 0x0010000000000000u64;
pub const SQUARE_F7: Bitboard = 0x0020000000000000u64;
pub const SQUARE_G7: Bitboard = 0x0040000000000000u64;
pub const SQUARE_H7: Bitboard = 0x0080000000000000u64;
pub const SQUARE_A8: Bitboard = 0x0100000000000000u64;
pub const SQUARE_B8: Bitboard = 0x0200000000000000u64;
pub const SQUARE_C8: Bitboard = 0x0400000000000000u64;
pub const SQUARE_D8: Bitboard = 0x0800000000000000u64;
pub const SQUARE_E8: Bitboard = 0x1000000000000000u64;
pub const SQUARE_F8: Bitboard = 0x2000000000000000u64;
pub const SQUARE_G8: Bitboard = 0x4000000000000000u64;
pub const SQUARE_H8: Bitboard = 0x8000000000000000u64;

pub const ALL_SQUARES: Bitboard = 0xFFFF_FFFF_FFFF_FFFFu64;

// squares that need to be empty to castle
pub const SHORT_CASTLE_BITBOARD: Bitboard = SQUARE_F1 | SQUARE_G1;
pub const LONG_CASTLE_BITBOARD: Bitboard = SQUARE_B1 | SQUARE_C1 | SQUARE_D1;
pub const CASTLED_KING_BITBOARD: Bitboard = SQUARE_A1 | SQUARE_B1 | SQUARE_C1 | SQUARE_G1 | SQUARE_H1;

// which squares can move in a given direction
const NNW_MASK: Bitboard = !(FILE_A | RANK_7 | RANK_8);
const NNE_MASK: Bitboard = !(FILE_H | RANK_7 | RANK_8);
const NWW_MASK: Bitboard = !(FILE_A | FILE_B | RANK_8);
const NEE_MASK: Bitboard = !(FILE_G | FILE_H | RANK_8);
const SSW_MASK: Bitboard = !(FILE_A | RANK_1 | RANK_2);
const SSE_MASK: Bitboard = !(FILE_H | RANK_1 | RANK_2);
const SWW_MASK: Bitboard = !(FILE_A | FILE_B | RANK_1);
const SEE_MASK: Bitboard = !(FILE_G | FILE_H | RANK_1);

// const N_MASK: Bitboard = !RANK_8;
// const S_MASK: Bitboard = !RANK_1;
// const E_MASK: Bitboard = !FILE_H;
// const W_MASK: Bitboard = !FILE_A;
// const NW_MASK: Bitboard = !(FILE_A | RANK_8);
// const NE_MASK: Bitboard = !(FILE_H | RANK_8);
// const SW_MASK: Bitboard = !(FILE_A | RANK_1);
// const SE_MASK: Bitboard = !(FILE_H | RANK_1);

pub const WHITE_START: Pieces = Pieces {
    king: SQUARE_E1,
    queens: SQUARE_D1,
    rooks: (SQUARE_A1 | SQUARE_H1),
    bishops: (SQUARE_C1 | SQUARE_F1),
    knights: (SQUARE_B1 | SQUARE_G1),
    pawns: RANK_2
};

pub const BLACK_START: Pieces = Pieces {
    king: SQUARE_E8,
    queens: SQUARE_D8,
    rooks: (SQUARE_A8 | SQUARE_H8),
    bishops: (SQUARE_C8 | SQUARE_F8),
    knights: (SQUARE_B8 | SQUARE_G8),
    pawns: RANK_7
};

// Functions
pub fn square_to_bitboard(square: types::Square) -> Bitboard {
    return match square {
        types::Square::A1 => SQUARE_A1,
        types::Square::B1 => SQUARE_B1,
        types::Square::C1 => SQUARE_C1,
        types::Square::D1 => SQUARE_D1,
        types::Square::E1 => SQUARE_E1,
        types::Square::F1 => SQUARE_F1,
        types::Square::G1 => SQUARE_G1,
        types::Square::H1 => SQUARE_H1,
        types::Square::A2 => SQUARE_A2,
        types::Square::B2 => SQUARE_B2,
        types::Square::C2 => SQUARE_C2,
        types::Square::D2 => SQUARE_D2,
        types::Square::E2 => SQUARE_E2,
        types::Square::F2 => SQUARE_F2,
        types::Square::G2 => SQUARE_G2,
        types::Square::H2 => SQUARE_H2,
        types::Square::A3 => SQUARE_A3,
        types::Square::B3 => SQUARE_B3,
        types::Square::C3 => SQUARE_C3,
        types::Square::D3 => SQUARE_D3,
        types::Square::E3 => SQUARE_E3,
        types::Square::F3 => SQUARE_F3,
        types::Square::G3 => SQUARE_G3,
        types::Square::H3 => SQUARE_H3,
        types::Square::A4 => SQUARE_A4,
        types::Square::B4 => SQUARE_B4,
        types::Square::C4 => SQUARE_C4,
        types::Square::D4 => SQUARE_D4,
        types::Square::E4 => SQUARE_E4,
        types::Square::F4 => SQUARE_F4,
        types::Square::G4 => SQUARE_G4,
        types::Square::H4 => SQUARE_H4,
        types::Square::A5 => SQUARE_A5,
        types::Square::B5 => SQUARE_B5,
        types::Square::C5 => SQUARE_C5,
        types::Square::D5 => SQUARE_D5,
        types::Square::E5 => SQUARE_E5,
        types::Square::F5 => SQUARE_F5,
        types::Square::G5 => SQUARE_G5,
        types::Square::H5 => SQUARE_H5,
        types::Square::A6 => SQUARE_A6,
        types::Square::B6 => SQUARE_B6,
        types::Square::C6 => SQUARE_C6,
        types::Square::D6 => SQUARE_D6,
        types::Square::E6 => SQUARE_E6,
        types::Square::F6 => SQUARE_F6,
        types::Square::G6 => SQUARE_G6,
        types::Square::H6 => SQUARE_H6,
        types::Square::A7 => SQUARE_A7,
        types::Square::B7 => SQUARE_B7,
        types::Square::C7 => SQUARE_C7,
        types::Square::D7 => SQUARE_D7,
        types::Square::E7 => SQUARE_E7,
        types::Square::F7 => SQUARE_F7,
        types::Square::G7 => SQUARE_G7,
        types::Square::H7 => SQUARE_H7,
        types::Square::A8 => SQUARE_A8,
        types::Square::B8 => SQUARE_B8,
        types::Square::C8 => SQUARE_C8,
        types::Square::D8 => SQUARE_D8,
        types::Square::E8 => SQUARE_E8,
        types::Square::F8 => SQUARE_F8,
        types::Square::G8 => SQUARE_G8,
        types::Square::H8 => SQUARE_H8
    }
}

pub fn bitboard_from_index(i: u32) -> Bitboard {
    return (0x1 << i);
}

pub fn flip_bitboard(b: Bitboard) -> Bitboard {
    let mut res : Bitboard = 0u64;
    for i in 0..8 {
        let shift1 = i*8;
        let shift2 = 56 - shift1;
        let mask = 0xFFu64 << shift1;
        res |= (((b & mask) >> shift1) << shift2);
    }
    return res;
}

pub fn get_bitboard_rel(b: Bitboard, c: Color) -> Bitboard {
    return if c == Color::White { b } else { flip_bitboard(b) };
}

pub fn flip_bitboard_pieces(p: Pieces) -> Pieces {
    return Pieces {
        king: flip_bitboard(p.king),
        queens: flip_bitboard(p.queens),
        rooks: flip_bitboard(p.rooks),
        bishops: flip_bitboard(p.bishops),
        knights: flip_bitboard(p.knights),
        pawns: flip_bitboard(p.pawns)
    }
}

pub fn get_bitboard_pieces_rel(p: Pieces, c: Color) -> Pieces {
    return if c == Color::White { p } else { flip_bitboard_pieces(p) };
}

pub fn slide1(b: Bitboard, dir: Direction) -> Bitboard {
    return slide(b, dir, 1);
}

pub fn slide(b: Bitboard, dir: Direction, dist: i32) -> Bitboard {
    match dir {
        Direction::N => {
            let mask = ALL_SQUARES >> (8*dist);
            return (b & mask) << 8*dist;
        },
        Direction::S => {
            let mask = ALL_SQUARES << (8*dist);
            return (b & mask) >> 8*dist;
        },
        Direction::E => {
            let row_mask = 0xFFu8 >> dist;
            let mask = u64::from_ne_bytes([row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask]);
            return (b & mask) << dist;
        },
        Direction::W => {
            let row_mask = 0xFFu8 << dist;
            let mask = u64::from_ne_bytes([row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask]);
            return (b & mask) >> dist;
        },
        Direction::NW => {
            let row_mask = 0xFFu8 << dist;
            let mask = (ALL_SQUARES >> (8*dist)) & u64::from_ne_bytes([row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask]);
            return (b & mask) << 7*dist;
        },
        Direction::NE => {
            let row_mask = 0xFFu8 >> dist;
            let mask = (ALL_SQUARES >> (8*dist)) & u64::from_ne_bytes([row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask]);
            return (b & mask) << 9*dist;
        },
        Direction::SW => {
            let row_mask = 0xFFu8 << dist;
            let mask = (ALL_SQUARES << (8*dist)) & u64::from_ne_bytes([row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask]);
            return (b & mask) >> 9*dist;
        },
        Direction::SE => {
            let row_mask = 0xFFu8 >> dist;
            let mask = (ALL_SQUARES << (8*dist)) & u64::from_ne_bytes([row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask, row_mask]);
            return (b & mask) >> 7*dist;
        }
    }
}

pub fn knight_hop(b: Bitboard, kh: KnightHop) -> Bitboard {
    return match kh {
        KnightHop::NNW => (b & NNW_MASK) << 15,
        KnightHop::NNE => (b & NNE_MASK) << 17,
        KnightHop::NWW => (b & NWW_MASK) << 6,
        KnightHop::NEE => (b & NEE_MASK) << 10,
        KnightHop::SSW => (b & SSW_MASK) >> 17,
        KnightHop::SSE => (b & SSE_MASK) >> 15,
        KnightHop::SWW => (b & SWW_MASK) >> 10,
        KnightHop::SEE => (b & SEE_MASK) >> 6
    }
}

pub fn get_pieces_material_value(p: Pieces, g: GamePhase) -> i32 {
    let mut eval = 0;

    // king
    let mut bb = p.king;
    let idx = bb.trailing_zeros();
    if bb != 0 {
        if g == GamePhase::Endgame {
            eval += eval::KING_ENDGAME_EVAL[(idx as usize)];
        } else {
            eval += eval::KING_EVAL[(idx as usize)];
        }
    }

    // queens
    bb = p.queens;
    while bb != 0 {
        let idx = bb.trailing_zeros();
        let idx_bb = bitboard_from_index(idx);
        eval += eval::QUEEN_EVAL[(idx as usize)];
        bb &= !idx_bb;
    }

    // rooks
    bb = p.rooks;
    while bb != 0 {
        let idx = bb.trailing_zeros();
        let idx_bb = bitboard_from_index(idx);
        eval += eval::ROOK_EVAL[(idx as usize)];
        bb &= !idx_bb;
    }

    // bishops
    bb = p.bishops;
    while bb != 0 {
        let idx = bb.trailing_zeros();
        let idx_bb = bitboard_from_index(idx);
        eval += eval::BISHOP_EVAL[(idx as usize)];
        bb &= !idx_bb;
    }

    // knights
    bb = p.knights;
    while bb != 0 {
        let idx = bb.trailing_zeros();
        let idx_bb = bitboard_from_index(idx);
        eval += eval::KNIGHT_EVAL[(idx as usize)];
        bb &= !idx_bb;
    }

    // pawns
    bb = p.pawns;
    while bb != 0 {
        let idx = bb.trailing_zeros();
        let idx_bb = bitboard_from_index(idx);
        eval += eval::PAWN_EVAL[(idx as usize)];
        bb &= !idx_bb;
    }

    return eval;
}