#![feature(pattern, destructuring_assignment)]
#![allow(unused_unsafe)]

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (#[allow(unused_unsafe)] unsafe { crate::log(&format_args!($($t)*).to_string()) })
}

#[macro_use]
extern crate add_getters_setters;

use wasm_bindgen::prelude::*;

mod webgl;
pub use webgl::*;

pub mod engine;

pub mod models;

pub mod delaunay;

pub mod universe;

pub mod util;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn set_info(x: f32, y: f32, z: f32, angl_x: f32, angl_y: f32, angl_z: f32);

    #[wasm_bindgen]
    fn set_settings(settings: JsValue);
}
