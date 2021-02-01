mod camera;
pub use camera::*;

/// The player entity
mod entity;
pub use entity::*;

pub mod physics;

pub mod objects;

pub use objects::{Object, ObjectConfig, ObjectFactory};

pub mod rendering;
pub use rendering::renderer::Renderer;

pub type Index = usize;
pub type Float = f32;

pub type Vector<A> = [A; 3];

pub enum Mesh {
    Indexed {
        vertices: Vec<Float>,
        normals: Vec<Float>,
        indices: Vec<u16>,
    },
    NotIndexed {
        vertices: Vec<Float>,
        normals: Vec<Float>,
    },
}
