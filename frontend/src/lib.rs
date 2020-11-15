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

pub mod settings;

pub mod engine;

pub mod models;

pub mod delaunay;

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

use pw_derive::Settings;

pub struct NoDefault {}

#[derive(Settings)]
pub struct InnerSettings {
    #[settings(name = "Slidy", value = 0.4)]
    pub x: f32,
    #[settings(name = "Slidy")]
    pub y: Vec<f32>,
}

#[derive(Settings)]
pub struct SettingsInst {
    #[settings(name = "settings_name", value = 0.2)]
    pub size: f32,
    pub cool_string: String,
    #[settings(id = "cool_id", name = "settings_name")]
    pub foo: String,

    pub location: Vec<f32>,
}
