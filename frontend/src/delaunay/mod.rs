mod delaunay;
pub use delaunay::*;

pub type Type = f32;
pub type VertexType = [Type; 2];
pub type EdgeType = Edge;
pub type TriangleType = Triangle;

pub fn dist2(this: &VertexType, other: &VertexType) -> Type {
    let dx = this[0] - other[0];
    let dy = this[1] - other[1];
    dx * dx + dy * dy
}

pub fn norm2(this: &VertexType) -> Type {
    this[0] * this[0] + this[1] * this[1]
}

pub fn dist(this: &VertexType, other: &VertexType) -> f32 {
    (this[0] - other[0]).hypot(this[1] - other[1])
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub v: usize,
    pub w: usize,
    pub is_bad: bool,
}

impl Edge {
    pub fn new(v: usize, w: usize) -> Self {
        Self {
            v,
            w,
            is_bad: false,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.v == other.v || self.v == other.w) && (self.w == other.v || self.w == other.v)
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub is_bad: bool,
}

impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a,
            b,
            c,
            is_bad: false,
        }
    }

    pub fn contains_vertex(&self, v: usize) -> bool {
        self.a == v || self.b == v || self.c == v
    }

    pub fn circum_circle_contains(&self, v: usize, vs: &Vec<VertexType>) -> bool {
        let a = vs[self.a];
        let b = vs[self.b];
        let c = vs[self.c];

        let ab = norm2(&a);
        let cd = norm2(&b);
        let ef = norm2(&c);

        let ax = a[0];
        let ay = a[1];
        let bx = b[0];
        let by = b[1];
        let cx = c[0];
        let cy = c[1];

        let circum_x = (ab * (cy - by) + cd * (ay - cy) + ef * (by - ay))
            / (ax * (cy - by) + bx * (ay - cy) + cx * (by - ay));
        let circum_y = (ab * (cx - bx) + cd * (ax - cx) + ef * (bx - ax))
            / (ay * (cx - bx) + by * (ax - cx) + cy * (bx - ax));

        let circum = [circum_x / 2.0, circum_y / 2.0];
        let circum_radius = dist2(&a, &circum);
        let dist = dist2(&vs[v], &circum);

        dist <= circum_radius
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a || self.a == other.b || self.a == other.c)
            && (self.b == other.a || self.b == other.b || self.b == other.c)
            && (self.c == other.a || self.c == other.b || self.c == other.c)
    }
}
