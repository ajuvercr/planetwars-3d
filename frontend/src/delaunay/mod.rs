mod delaunay;
pub use delaunay::*;

pub type Type = f32;
pub type VertexType = (Type, Type);
pub type EdgeType = Edge;
pub type TriangleType = Triangle;

pub fn dist2((tx, ty): &VertexType, (ox, oy): &VertexType) -> Type {
    let dx = tx - ox;
    let dy = ty - oy;
    dx * dx + dy * dy
}

pub fn norm2((x, y): &VertexType) -> Type {
    x * x + y * y
}

pub fn dist((tx, ty): &VertexType, (ox, oy): &VertexType) -> f32 {
    (tx - ox).hypot(ty - oy)
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
    pub fn new(a: usize, b: usize, c: usize, vs: &Vec<VertexType>) -> Self {
        let (ax, ay) = vs[a];
        let (bx, by) = vs[b];
        let (cx, cy) = vs[c];

        if (bx - ax) * (cy - ay) - (by - ay) * (cx - ax) < 0.0 {
            Self {
                a: b,
                b: a,
                c,
                is_bad: false,
            }
        } else {
            Self {
                a,
                b,
                c,
                is_bad: false,
            }
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

        let (ax, ay) = a;
        let (bx, by) = b;
        let (cx, cy) = c;

        let circum_x = (ab * (cy - by) + cd * (ay - cy) + ef * (by - ay))
            / (ax * (cy - by) + bx * (ay - cy) + cx * (by - ay));
        let circum_y = (ab * (cx - bx) + cd * (ax - cx) + ef * (bx - ax))
            / (ay * (cx - bx) + by * (ax - cx) + cy * (bx - ax));

        let circum = (circum_x / 2.0, circum_y / 2.0);
        let circum_radius = dist2(&a, &circum);
        let dist = dist2(&vs[v], &circum);

        dist <= circum_radius
    }
}

#[allow(dead_code)]
#[inline]
fn det(a: Type, b: Type, c: Type, d: Type, e: Type, f: Type, g: Type, h: Type, i: Type) -> Type {
    a * e * i + b * f * g + c * d * h - c * e * g - b * d * i - a * f * h
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a || self.a == other.b || self.a == other.c)
            && (self.b == other.a || self.b == other.b || self.b == other.c)
            && (self.c == other.a || self.c == other.b || self.c == other.c)
    }
}
