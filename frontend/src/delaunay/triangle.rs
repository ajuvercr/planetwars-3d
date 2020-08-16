use super::{AlmostEqual, VertexType};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Triangle {
    pub a: VertexType,
    pub b: VertexType,
    pub c: VertexType,
    pub is_bad: bool,
}

impl Triangle {
    pub fn new(a: VertexType, b: VertexType, c: VertexType) -> Self {
        Self {
            a,
            b,
            c,
            is_bad: false,
        }
    }

    pub fn contains_vertex(&self, v: &VertexType) -> bool {
        self.a.almost_equal(v) || self.b.almost_equal(v) || self.c.almost_equal(v)
    }

    pub fn circum_circle_contains(&self, v: &VertexType) -> bool {
        let ab = self.a.norm();
        let cd = self.b.norm();
        let ef = self.c.norm();

        let ax = self.a.x;
        let ay = self.a.y;
        let bx = self.b.x;
        let by = self.b.y;
        let cx = self.c.x;
        let cy = self.c.y;

        let circum_x = (ab * (cy - by) + cd * (ay - cy) + ef * (by - ay))
            / (ax * (cy - by) + bx * (ay - cy) + cx * (by - ay));
        let circum_y = (ab * (cx - bx) + cd * (ax - cx) + ef * (bx - ax))
            / (ay * (cx - bx) + by * (ax - cx) + cy * (bx - ax));

        let circum = VertexType::new(circum_x, circum_y);
        let circum_radius = self.a.dist(&circum);
        let dist = v.dist2(&circum);

        dist <= circum_radius
    }
}

impl AlmostEqual for Triangle {
    fn almost_equal(&self, b: &Self) -> bool {
        self.a.almost_equal(&b.a) && self.b.almost_equal(&b.b) && self.c.almost_equal(&b.c)
    }
}
