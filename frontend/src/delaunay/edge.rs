use super::{AlmostEqual, VertexType};

#[derive(Clone, Debug)]
pub struct Edge {
    pub v: VertexType,
    pub w: VertexType,
    pub is_bad: bool,
}

impl Edge {
    pub fn new(v: VertexType, w: VertexType) -> Self {
        Self {
            v,
            w,
            is_bad: false,
        }
    }
}

impl AlmostEqual for Edge {
    fn almost_equal(&self, b: &Self) -> bool {
        self.v.almost_equal(&b.v) && self.w.almost_equal(&b.w)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.v == other.v || self.v == other.w) && (self.w == other.v || self.w == other.v)
    }
}
