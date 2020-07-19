
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sub(a: i32, b: i32) -> i32 {
    a + b
}
