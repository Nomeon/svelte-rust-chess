use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// Javascript functions
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// Rust functions
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn valid_moves(board_js: JsValue, square_js: JsValue, x: i32, y: i32) -> bool {
    let board: Vec<Square> = board_js.into_serde().unwrap();
    let square: Square = square_js.into_serde().unwrap();

    for target in board.iter() {
        if target.col == x {
            if target.row == y {
                if target.color != square.color {
                    return true;
                }
            }
        }
    }
    return false;
}

#[wasm_bindgen]
pub fn get_index(column: i8, row: i8) -> i8 {
    let index: i8 = column + (row * 8);
    return index;
}

#[wasm_bindgen]
pub fn possible_moves(board_js: JsValue, square_js: JsValue) -> Vec<i8> {
    let mut pos_moves: Vec<i8> = Vec::new();
    // let board: Vec<Square> = board_js.into_serde().unwrap();
    let square: Square = square_js.into_serde().unwrap();
    
    pos_moves.append(&mut pawn_moves(square));
    
    return pos_moves;
}


pub fn pawn_moves(square: Square) -> Vec<i8> {
    let mut pawn_moves = Vec::new();
    let col: i8 = square.col as i8;
    let row: i8 = square.row as i8;
    if square.color == "white" {
        if row == 2 {
            let index = get_index(col, row + 2);
            pawn_moves.push(index)
        }
    }
    return pawn_moves
}

// Rust Structs

#[derive(Serialize, Deserialize, Debug)]
pub struct Square {
    pub id: i32,
    pub col: i32,
    pub row: i32,
    pub color: String,
    pub piece: String,
    pub draggable: bool,
}