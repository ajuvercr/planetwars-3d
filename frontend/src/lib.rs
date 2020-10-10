#![feature(pattern)]
#![allow(unused_unsafe)]

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (#[allow(unused_unsafe)] unsafe { crate::log(&format_args!($($t)*).to_string()) })
}



use wasm_bindgen::prelude::*;

pub mod entity;
mod webgl;
pub use webgl::*;

pub mod models;

pub mod delaunay;

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

use pw_derive::Settings;

pub struct NoDefault {

}

#[derive(Settings)]
pub struct SettingsInst {
    pub size: NoDefault,
    pub cool_string: String,
    #[name("bar")] #[id("cools_string")] #[value("FOO STRING")]
    pub foo: String,
}
