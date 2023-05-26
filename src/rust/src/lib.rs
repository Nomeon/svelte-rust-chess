use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
// use serde::{Serialize, Deserialize};
use shakmaty::{fen::Fen, Chess, Position, CastlingMode, Square, Move};


// Rust functions that can be called from JS
#[wasm_bindgen]
pub fn get_possible_moves(fen_str: &str, square: &str) -> JsValue {
    let board = get_pos_from_fen(fen_str);
    let mut moves = board.legal_moves();

    let sq = Square::from_ascii(square.as_bytes()).unwrap();
    moves.retain(|m| m.from() == Some(sq));
    let legal_moves: Vec<String> = moves.into_iter().map(|m| m.to().to_string()).collect();
    let legal_moves_js: JsValue = JsValue::from_serde(&legal_moves).unwrap();
    return legal_moves_js;
}

#[wasm_bindgen]
pub fn play_move(fen_str: &str, square: &str, target: &str) -> JsValue {
    let mut board = get_pos_from_fen(fen_str);
    let sq = Square::from_ascii(square.as_bytes()).unwrap();
    let target_sq = Square::from_ascii(target.as_bytes()).unwrap();

    let mut moves = board.legal_moves();
    moves.retain(|m| m.from() == Some(sq) && m.to() == target_sq);
    let playable_move: &Move = &moves[0];
    board = board.play(playable_move).unwrap();
    let new_fen = Fen::from_position(board, shakmaty::EnPassantMode::Legal);
    let new_fen_js: JsValue = JsValue::from_serde(&new_fen.to_string()).unwrap();
    return new_fen_js
}

// Local functions
fn get_pos_from_fen(fen_str: &str) -> Chess {
    let fen_bytes = fen_str.as_bytes();
    let fen = Fen::from_ascii(fen_bytes).unwrap();
    let board: Chess = fen.into_position(CastlingMode::Standard).unwrap();
    return board;
}