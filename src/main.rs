mod types;
mod bitboard;
mod utils;
mod eval;
mod move_search;

use std::io;
use rand;
use rand::seq::SliceRandom;

fn main() -> io::Result<()> {
    let mut pos = utils::START_POSITION;

    // UCI parsing
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        line = line.trim().to_string();

        if line == "quit" {
            break;
        } else if line == "uci" {
            println!("id name Goldychess v0.1");
            println!("id author Michael Goldstein");
            println!("uciok");
        } else if line == "isready" {
            println!("readyok");
        } else if line.starts_with("position fen") {
            println!("ERROR: parsing fen not implemented");
        } else if line.starts_with("position startpos") {
            pos = utils::START_POSITION; // reset to startpos
            let mut tokens = line.split_whitespace().skip(2);
            match tokens.next() {
                Some(x) => {
                    if x == "moves" {
                        for m_str in tokens {
                            match (types::Move::from_uci(m_str)) {
                                Some(m) => {
                                    pos = utils::apply_move(&pos, m);
                                },
                                _ => {
                                    println!("ERROR: failed to parse move: {}", m_str);
                                    continue;
                                }
                            }
                        }
                    }
                },
                _ => {
                    continue;
                }
            }
        } else if line.starts_with("go") {
            let n = "go".len();
            let tokens = line[n..].split_whitespace();
            let mut moves = move_search::calc_moves(&pos);

            moves.shuffle(&mut rand::thread_rng()); // shuffle to make our move choices a little more interesting

            // let mut moves_with_eval = Vec::<(i32, Vec::<types::Move>)>::new();
            let mut moves_with_eval = Vec::<(i32, types::Move)>::new();
            let depth = 6;
            // for m in moves.iter() {
            //     let eval = eval::eval_move(&pos, *m, depth);
            //     moves_with_eval.push(eval);
            // }
            // moves_with_eval.sort_by_key(|k| k.0);
    
            for m in moves.iter() {
                let eval = eval::eval_move(&pos, *m, depth);
                moves_with_eval.push((eval, *m));
            }
            moves_with_eval.sort_by_key(|k| k.0);

            // for (e, m) in moves_with_eval.iter() {
            //     println!("info depth {} score cp {} pv {}", depth, e, m.to_uci());
            // }

            let best_pv = moves_with_eval.into_iter().last();

            // let best_move = moves.choose(&mut rand::thread_rng());
            // match best_pv {
            //     Some((e, pv)) => {
            //         let best_move = pv.into_iter().nth(0);
            //         match best_move {
            //             Some(m) => println!("bestmove {}", m.to_uci()),
            //             _ => println!("ERROR: no move found")
            //         }
            //     },
            //     _ => println!("ERROR: no move found")
            // }

            match best_pv {
                Some((e, m)) => {
                    // TODO this is a hack to make Lichess happy with how we promote
                    let p = pos.get_piece_at(m.from_square);
                    let bb = bitboard::square_to_bitboard(m.to_square);
                    let bb_rel = if pos.turn == types::Color::White { bb } else { bitboard::flip_bitboard(bb) };
                    if (!p.is_none() && p.unwrap().piece_type == types::PieceType::Pawn) && ((bb_rel & bitboard::RANK_8) != 0) {
                        println!("bestmove {}q", m.to_uci());
                    }
                    else {
                        println!("bestmove {}", m.to_uci());
                    }
                },
                _ => println!("ERROR: no move found")
            }
            
            // for m in moves.iter() {
            //     println!("{:?}", m)
            // }

        } else if line.starts_with("time") {

        } else if line.starts_with("otim") {

        } else if line.starts_with("DEBUG") {
            let mut tokens = line.split_whitespace().skip(1);
            match tokens.next() {
                Some(x) => {
                    match x {
                        "showmoves" => {
                            let moves = move_search::calc_moves(&pos);
                            for m in moves.iter() {
                                println!("{:?}", m);
                            }
                        },
                        "showpmoves" => {
                            let moves = move_search::calc_pmoves(&pos, false);
                            for m in moves.iter() {
                                println!("{:?}", m);
                            }
                        },
                        "showpieces" => {
                            println!("White pieces:");
                            for p in pos.white_pieces.iter() {
                                if (p.piece_type != types::PieceType::Null) {
                                    println!("{:?}", p)
                                }
                            }
                            println!("Black pieces:");
                            for p in pos.black_pieces.iter() {
                                if (p.piece_type != types::PieceType::Null) {
                                    println!("{:?}", p)
                                }
                            }
                        },
                        "showboard" => {
                            pos.pretty_print();
                        },
                        "color" => {
                            println!("{:?}", pos.turn);
                        },
                        "showbitboards" => {
                            println!("White K/Q/R/B/N/P:");
                            println!("{:016X}", pos.white_bitboard_pieces.king);
                            println!("{:016X}", pos.white_bitboard_pieces.queens);
                            println!("{:016X}", pos.white_bitboard_pieces.rooks);
                            println!("{:016X}", pos.white_bitboard_pieces.bishops);
                            println!("{:016X}", pos.white_bitboard_pieces.knights);
                            println!("{:016X}", pos.white_bitboard_pieces.pawns);
                            println!("Black K/Q/R/B/N/P:");
                            println!("{:016X}", pos.black_bitboard_pieces.king);
                            println!("{:016X}", pos.black_bitboard_pieces.queens);
                            println!("{:016X}", pos.black_bitboard_pieces.rooks);
                            println!("{:016X}", pos.black_bitboard_pieces.bishops);
                            println!("{:016X}", pos.black_bitboard_pieces.knights);
                            println!("{:016X}", pos.black_bitboard_pieces.pawns);
                        },
                        _ => {
                            continue;
                        }
                    }
                },
                _ => {
                    continue;
                }
            }
        }
    }

    return Ok(());
}
