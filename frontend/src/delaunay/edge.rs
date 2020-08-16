use super::{AlmostEqual, VertexType};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
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
