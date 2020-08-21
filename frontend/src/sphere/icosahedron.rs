
type Triangle = (usize, usize, usize);
type Vertex = (f32, f32, f32);

fn normalize((x, y, z): Vertex) -> Vertex {
    let lrcp = 1.0 / (x * x + y * y+ z * z).sqrt();

    (x*lrcp, y*lrcp, z*lrcp)
}

pub fn gen_sphere_icosahedral(n: f32) -> (Vec<f32>, Vec<u16>, Vec<f32>) {
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0;

    let mut verts = vec![
        (-1.0,  t, 0.0),
        ( 1.0,  t, 0.0),
        (-1.0, -t, 0.0),
        ( 1.0, -t, 0.0),
        (0.0, -1.0,  t),
        (0.0,  1.0,  t),
        (0.0, -1.0, -t),
        (0.0,  1.0, -t),
        ( t, 0.0, -1.0),
        ( t, 0.0,  1.0),
        (-t, 0.0, -1.0),
        (-t, 0.0,  1.0),
    ];

    let mut layers = vec![
        n,n,
        n,n,
        n,n,
        n,n,
        n,n,
        n,n,
    ];

    let mut triangles = vec![
	    (0, 11, 5),
	    (0, 5, 1),
	    (0, 1, 7),
	    (0, 7, 10),
	    (0, 10, 11),
	    (1, 5, 9),
	    (5, 11, 4),
	    (11, 10, 2),
	    (10, 7, 6),
	    (7, 1, 8),
	    (3, 9, 4),
	    (3, 4, 2),
	    (3, 2, 6),
	    (3, 6, 8),
	    (3, 8, 9),
	    (4, 9, 5),
	    (2, 4, 11),
	    (6, 2, 10),
	    (8, 6, 7),
	    (9, 8, 1),
    ];

    let mut i = n;
    for _ in 0..(n as i32) {
        i -= 1.0;
        triangles = gen_more(&mut verts, &mut layers, triangles, i);
    }

    let mut v_outs = Vec::new();
    let mut idx_out = Vec::new();

    for vert in verts {
        let (x, y, z) = normalize(vert);   // normalize
        v_outs.push(x);
        v_outs.push(y);
        v_outs.push(z);
    }

    for (a, b, c) in triangles {
        idx_out.push(a as u16);
        idx_out.push(b as u16);
        idx_out.push(c as u16);
    }

    console_log!("{:?}", v_outs);

    (v_outs, idx_out, layers)
}

/**
  3
  /\
6/__\5
/_\/_\
1  4  2
*/

fn gen_more(verts: &mut Vec<Vertex>, layers: &mut Vec<f32>, triangles: Vec<Triangle>, layer: f32,) -> Vec<Triangle> {
    let mut new_triangles = Vec::new();

    for t in triangles {
        let (x1, y1, z1) = verts[t.0];
        let (x2, y2, z2) = verts[t.1];
        let (x3, y3, z3) = verts[t.2];

        let v4 = ((x1 + x2) / 0.5, (y1 + y2) / 0.5,(z1 + z2) / 0.5);
        let v5 = ((x3 + x2) / 0.5, (y3 + y2) / 0.5,(z3 + z2) / 0.5);
        let v6 = ((x1 + x3) / 0.5, (y1 + y3) / 0.5,(z1 + z3) / 0.5);

        layers.push(layer);
        verts.push(v4);
        let i4 = verts.len() - 1;

        layers.push(layer);
        verts.push(v5);
        let i5 = verts.len() - 1;

        layers.push(layer);
        verts.push(v6);
        let i6 = verts.len() - 1;

        new_triangles.push((t.0, i4, i6));
        new_triangles.push((t.1, i4, i5));
        new_triangles.push((t.2, i5, i6));

        new_triangles.push((i4, i5, i6));
    }

    new_triangles
}