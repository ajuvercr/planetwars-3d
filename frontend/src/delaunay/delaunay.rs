use super::{AlmostEqual, Edge, EdgeType, Triangle, TriangleType, VertexType};

pub struct Delaunay {
    triangles: Vec<TriangleType>,
    edges: Vec<EdgeType>,
    vertices: Vec<VertexType>,
}

impl Delaunay {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            edges: Vec::new(),
            vertices: Vec::new(),
        }
    }

    #[inline]
    pub fn triangles(&self) -> &Vec<TriangleType> {
        &self.triangles
    }

    #[inline]
    pub fn edges(&self) -> &Vec<EdgeType> {
        &self.edges
    }

    #[inline]
    pub fn vertices(&self) -> &Vec<VertexType> {
        &self.vertices
    }

    pub fn triangulate(vertices: Vec<VertexType>) -> Self {
        let mut triangles = Vec::new();
        let mut edges = Vec::new();

        let (min_x, min_y, max_x, max_y) = {
            let mut min_x = vertices[0].x;
            let mut min_y = vertices[0].y;
            let mut max_x = min_x;
            let mut max_y = min_y;

            for v in &vertices {
                if v.x < min_x {
                    min_x = v.x;
                }
                if v.y < min_y {
                    min_y = v.y;
                }
                if v.x > max_x {
                    max_x = v.x;
                }
                if v.y > max_y {
                    max_y = v.y;
                }
            }

            (min_x, min_y, max_x, max_y)
        };

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let delta_max = dx.max(dy);
        let mid_x = (min_x + max_x) / 2.0;
        let mid_y = (min_y + max_y) / 2.0;

        let p1 = VertexType::new(mid_x - 20.0 * delta_max, mid_y - delta_max);
        let p2 = VertexType::new(mid_x, mid_y + 20.0 * delta_max);
        let p3 = VertexType::new(mid_x + 20.0 * delta_max, mid_y - delta_max);

        triangles.push(Triangle::new(p1, p2, p3));

        for p in &vertices {
            let mut polygon = Vec::new();

            for t in triangles.iter_mut() {
                if t.circum_circle_contains(p) {
                    t.is_bad = true;
                    polygon.push(Edge::new(t.a, t.b));
                    polygon.push(Edge::new(t.b, t.c));
                    polygon.push(Edge::new(t.c, t.a));
                }
            }

            triangles = triangles.into_iter().filter(|t| !t.is_bad).collect();

            for i in 0..polygon.len() {
                for j in i + 1..polygon.len() {
                    if polygon[i].almost_equal(&polygon[j]) {
                        polygon[i].is_bad = true;
                        polygon[j].is_bad = true;
                    }
                }
            }

            polygon = polygon.into_iter().filter(|t| !t.is_bad).collect();

            for Edge {
                v,
                w,
                is_bad: _is_bad,
            } in polygon
            {
                triangles.push(Triangle::new(v, w, p.clone()));
            }
        }

        triangles = triangles
            .into_iter()
            .filter(|t| {
                !(t.contains_vertex(&p1) || t.contains_vertex(&p2) || t.contains_vertex(&p3))
            })
            .collect();

        for t in &triangles {
            edges.push(Edge::new(t.a, t.b));
            edges.push(Edge::new(t.b, t.c));
            edges.push(Edge::new(t.c, t.a));
        }

        Self {
            vertices,
            edges,
            triangles,
        }
    }
}
