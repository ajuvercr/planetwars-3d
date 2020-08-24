use cgmath::{MetricSpace, Vector3, Zero};

type Triangle = (usize, usize, usize);
type Vertex = Vector3<f32>;

#[inline]
pub fn normalize(point: Vertex) -> Vertex {
    point / point.distance(Vector3::zero())
}

// Vertices have normals too now, so 6 floats per vertex
pub fn gen_sphere_icosahedral(n: f32) -> Vec<f32> {
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let n = n + 1.0;

    let mut verts = vec![
        normalize(Vector3::new(-1.0, t, 0.0)),
        normalize(Vector3::new(1.0, t, 0.0)),
        normalize(Vector3::new(-1.0, -t, 0.0)),
        normalize(Vector3::new(1.0, -t, 0.0)),
        normalize(Vector3::new(0.0, -1.0, t)),
        normalize(Vector3::new(0.0, 1.0, t)),
        normalize(Vector3::new(0.0, -1.0, -t)),
        normalize(Vector3::new(0.0, 1.0, -t)),
        normalize(Vector3::new(t, 0.0, -1.0)),
        normalize(Vector3::new(t, 0.0, 1.0)),
        normalize(Vector3::new(-t, 0.0, -1.0)),
        normalize(Vector3::new(-t, 0.0, 1.0)),
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

    for _ in 0..(n as i32 - 1) {
        triangles = gen_more(&mut verts, triangles);
    }

    let mut v_outs = Vec::new();

    for (a, b, c) in triangles {
        let normal = normalize(calc_normal(verts[a], verts[b], verts[c]));
        let normal_ref = AsRef::<[f32; 3]>::as_ref(&normal);

        v_outs.extend_from_slice(AsRef::<[f32; 3]>::as_ref(&verts[a]));
        v_outs.extend_from_slice(normal_ref);

        v_outs.extend_from_slice(AsRef::<[f32; 3]>::as_ref(&verts[b]));
        v_outs.extend_from_slice(normal_ref);

        v_outs.extend_from_slice(AsRef::<[f32; 3]>::as_ref(&verts[c]));
        v_outs.extend_from_slice(normal_ref);
    }

    v_outs
}

// https://www.khronos.org/opengl/wiki/Calculating_a_Surface_Normal
pub fn calc_normal(p1: Vertex, p2: Vertex, p3: Vertex) -> Vertex {
    let Vertex {
        x: ux,
        y: uy,
        z: uz,
    } = p2 - p1;
    let Vertex {
        x: vx,
        y: vy,
        z: vz,
    } = p3 - p1;
    return Vertex::new(uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx);
}

/**
  3
  /\
6/__\5
/_\/_\
1  4  2
*/

fn get_point(i1: usize, i2: usize, verts: &mut Vec<Vertex>) -> usize {
    let new_vertex = normalize((verts[i1] + verts[i2]) * 0.5);
    let new_index = verts.len();
    verts.push(new_vertex);
    new_index
}

fn gen_more(verts: &mut Vec<Vertex>, triangles: Vec<Triangle>) -> Vec<Triangle> {
    let mut new_triangles = Vec::new();

    for t in triangles {
        let i4 = get_point(t.0, t.1, verts);
        let i5 = get_point(t.1, t.2, verts);
        let i6 = get_point(t.0, t.2, verts);

        new_triangles.push((t.0, i4, i6));
        new_triangles.push((t.1, i5, i4));
        new_triangles.push((t.2, i6, i5));

        new_triangles.push((i4, i5, i6));
    }

    new_triangles
}
