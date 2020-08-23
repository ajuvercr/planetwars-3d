use cgmath::{MetricSpace, Vector3, Zero};
use std::collections::HashMap;

type Triangle = (usize, usize, usize);
type Vertex = Vector3<f32>;

#[inline]
fn normalize(point: Vertex) -> Vertex {
    point / point.distance(Vector3::zero())
}

pub fn gen_sphere_icosahedral(n: f32) -> (Vec<f32>, Vec<u16>, Vec<f32>) {
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let n = n + 1.0;

    let mut verts = vec![
        Vector3::new(-1.0, t, 0.0),
        Vector3::new(1.0, t, 0.0),
        Vector3::new(-1.0, -t, 0.0),
        Vector3::new(1.0, -t, 0.0),
        Vector3::new(0.0, -1.0, t),
        Vector3::new(0.0, 1.0, t),
        Vector3::new(0.0, -1.0, -t),
        Vector3::new(0.0, 1.0, -t),
        Vector3::new(t, 0.0, -1.0),
        Vector3::new(t, 0.0, 1.0),
        Vector3::new(-t, 0.0, -1.0),
        Vector3::new(-t, 0.0, 1.0),
    ];

    let mut layers = vec![
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
        n - 0.1,
    ];

    let mut triangles = vec![
        (0, 5, 11),
        (0, 1, 5),
        (0, 7, 1),
        (0, 10, 7),
        (0, 11, 10),
        (1, 9, 5),
        (5, 4, 11),
        (11, 2, 10),
        (10, 6, 7),
        (7, 8, 1),
        (3, 4, 9),
        (3, 2, 4),
        (3, 6, 2),
        (3, 8, 6),
        (3, 9, 8),
        (4, 5, 9),
        (2, 11, 4),
        (6, 10, 2),
        (8, 7, 6),
        (9, 1, 8),
    ];

    let mut i = n;
    for _ in 0..(n as i32 - 1) {
        i -= 1.0;
        triangles = gen_more(&mut verts, &mut layers, triangles, i);
    }

    triangles = pinch_triangles(&mut verts, &mut layers, triangles);

    let mut v_outs = Vec::new();
    let mut idx_out = Vec::new();

    for vert in verts {
        let Vertex { x, y, z } = normalize(vert); // normalize
        v_outs.push(x);
        v_outs.push(y);
        v_outs.push(z);
    }

    for (a, b, c) in triangles {
        idx_out.push(a as u16);
        idx_out.push(b as u16);
        idx_out.push(c as u16);
    }

    (v_outs, idx_out, layers)
}

/**
  3
  /\
6/__\5
/_\/_\
1  4  2
*/

fn get_point(
    i1: usize,
    i2: usize,
    verts: &mut Vec<Vertex>,
    cache: &mut HashMap<(usize, usize), usize>,
    layer: f32,
    layers: &mut Vec<f32>,
) -> usize {
    let (i1, i2) = if i1 < i2 { (i1, i2) } else { (i2, i1) };
    if let Some(out) = cache.get(&(i1, i2)) {
        *out
    } else {
        let new_vertex = (verts[i1] + verts[i2]) * 0.5;
        let new_index = verts.len();
        verts.push(new_vertex);
        cache.insert((i1, i2), new_index);

        layers.push(layer - 0.1);

        new_index
    }
}

fn gen_more(
    verts: &mut Vec<Vertex>,
    layers: &mut Vec<f32>,
    triangles: Vec<Triangle>,
    layer: f32,
) -> Vec<Triangle> {
    let mut new_triangles = Vec::new();

    let mut cache = HashMap::new();

    for t in triangles {
        let i4 = get_point(t.0, t.1, verts, &mut cache, layer, layers);
        let i5 = get_point(t.1, t.2, verts, &mut cache, layer, layers);
        let i6 = get_point(t.0, t.2, verts, &mut cache, layer, layers);

        new_triangles.push((t.0, i6, i4));
        new_triangles.push((t.1, i4, i5));
        new_triangles.push((t.2, i5, i6));

        new_triangles.push((i4, i6, i5));
    }

    new_triangles
}

fn pinch_triangles(
    verts: &mut Vec<Vertex>,
    layers: &mut Vec<f32>,
    triangles: Vec<Triangle>,
) -> Vec<Triangle> {
    let mut out = Vec::new();

    for triag in triangles {
        let new_index = verts.len();
        verts.push((verts[triag.0] + verts[triag.1] + verts[triag.2]) / 3.0);

        // let l = layers[triag.0].min(layers[triag.1]).min(layers[triag.2]);
        // layers[triag.0] = 0.9;
        // layers[triag.1] = 0.9;
        // layers[triag.2] = 0.9;

        let l = (layers[triag.0] + layers[triag.1] + layers[triag.2]) / 3.0;
        layers.push(l - 1.0);

        out.push((triag.0, triag.1, new_index));
        out.push((triag.1, triag.2, new_index));
        out.push((triag.2, triag.0, new_index));
    }

    out
}
