pub mod objects;

pub use objects::{Object, ObjectConfig, ObjectFactory};

pub type Index = usize;
pub type Float = f32;

pub type Vector<A> = [A; 3];
