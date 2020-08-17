use super::{Edge, EdgeType, Triangle, TriangleType, VertexType};

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

    pub fn triangulate(vertices: &Vec<VertexType>) -> Self {
        let mut vertices = vertices.clone();
        let mut triangles = Vec::new();
        let mut edges = Vec::new();

        let (min_x, min_y, max_x, max_y) = {
            let mut min_x = vertices[0][0];
            let mut min_y = vertices[0][1];
            let mut max_x = min_x;
            let mut max_y = min_y;

            for v in &vertices {
                if v[0] < min_x {
                    min_x = v[0];
                }
                if v[1] < min_y {
                    min_y = v[1];
                }
                if v[0] > max_x {
                    max_x = v[0];
                }
                if v[1] > max_y {
                    max_y = v[1];
                }
            }

            (min_x, min_y, max_x, max_y)
        };

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let delta_max = dx.max(dy);
        let mid_x = (min_x + max_x) / 2.0;
        let mid_y = (min_y + max_y) / 2.0;

        let p1 = [mid_x - 20.0 * delta_max, mid_y - 20.0 * delta_max];
        let p2 = [mid_x, mid_y + 20.0 * delta_max];
        let p3 = [mid_x + 20.0 * delta_max, mid_y - 20.0 * delta_max];

        vertices.push(p1);
        vertices.push(p2);
        vertices.push(p3);

        triangles.push(Triangle::new(
            vertices.len() - 3,
            vertices.len() - 2,
            vertices.len() - 1,
        ));

        for p in 0..vertices.len() - 3 {
            // -3 for the newly added points
            let mut polygon = Vec::new();

            for t in triangles.iter_mut() {
                if t.circum_circle_contains(p, &vertices) {
                    t.is_bad = true;
                    polygon.push(Edge::new(t.a, t.b));
                    polygon.push(Edge::new(t.b, t.c));
                    polygon.push(Edge::new(t.c, t.a));
                }
            }

            triangles = triangles.into_iter().filter(|t| !t.is_bad).collect();

            for i in 0..polygon.len() {
                for j in i + 1..polygon.len() {
                    if polygon[i] == polygon[j] {
                        polygon[i].is_bad = true;
                        polygon[j].is_bad = true;
                    }
                }
            }

            for Edge { v, w, is_bad } in polygon {
                if !is_bad {
                    triangles.push(Triangle::new(v, w, p.clone()));
                }
            }
        }

        triangles = triangles
            .into_iter()
            .filter(|t| {
                !(t.contains_vertex(vertices.len() - 3)
                    || t.contains_vertex(vertices.len() - 2)
                    || t.contains_vertex(vertices.len() - 1))
            })
            .collect();

        for t in &triangles {
            edges.push(Edge::new(t.a, t.b));
            edges.push(Edge::new(t.b, t.c));
            edges.push(Edge::new(t.c, t.a));
        }

        vertices.pop();
        vertices.pop();
        vertices.pop();

        Self {
            vertices,
            edges,
            triangles,
        }
    }
}
