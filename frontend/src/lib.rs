#![feature(pattern, destructuring_assignment)]
#![allow(unused_unsafe)]

#[cfg(feature = "web")]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (#[allow(unused_unsafe)] unsafe { crate::log(&format_args!($($t)*).to_string()) })
}

#[cfg(feature = "standalone")]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (#[allow(unused_unsafe)] println!($($t)*))
}

#[macro_use]
extern crate add_getters_setters;

pub mod engine;

pub mod models;

pub mod delaunay;

pub mod universe;

pub mod scene;

#[cfg(feature = "web")]
pub mod web;
#[cfg(feature = "web")]
pub use web::*;

#[cfg(feature = "standalone")]
pub mod standalone;
#[cfg(feature = "standalone")]
pub use standalone::*;

pub struct FpsCounter {
    fps: u32,
    time: f64,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self { fps: 0, time: 0.0 }
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        self.fps += 1;

        if self.time > 1.0 {
            self.time = 0.0;
            console_log!("Fps {}", self.fps);
            self.fps = 0;
        }
    }
}
