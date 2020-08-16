mod triangle;
pub use triangle::*;

mod vector2;
pub use vector2::*;

mod edge;
pub use edge::*;

mod delaunay;
pub use delaunay::*;

pub type Type = f32;
pub type VertexType = Vector2<Type>;
pub type EdgeType = Edge;
pub type TriangleType = Triangle;

// Big todo:see if you can make edge and triangle point to vertices instead of copying

pub trait AlmostEqual {
    fn almost_equal(&self, b: &Self) -> bool;
}

impl AlmostEqual for f32 {
    fn almost_equal(&self, b: &f32) -> bool {
        (self - b).abs() <= self.abs().max(b.abs()) * std::f32::EPSILON
    }
}

impl AlmostEqual for f64 {
    fn almost_equal(&self, b: &f64) -> bool {
        (self - b).abs() <= self.abs().max(b.abs()) * std::f64::EPSILON
    }
}
