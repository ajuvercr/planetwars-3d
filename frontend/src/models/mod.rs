// struct Rect([f32; 3], [f32; 3], [f32; 3], [f32; 3]);

use crate::delaunay::Delaunay;
use cgmath::Vector3;
mod icosahedron;
pub use icosahedron::*;

pub fn gen_generalized_spiral(n: f32, c: f32) -> Vec<f32> {
    let mut out = Vec::new();

    let mut phi = 0.0;
    let n_sqrt = c / (n + 1 as f32).sqrt();

    for k in 2..(n as u32) {
        let k = k as f32;

        let hk = 2.0 * (k - 1.0) / n - 1.0;

        let eta = hk.acos();
        phi = phi + n_sqrt / (1.0 - hk * hk).sqrt();

        let (eta_sin, eta_cos) = eta.sin_cos();
        let (phi_sin, phi_cos) = phi.sin_cos();
        out.push(eta_sin * phi_sin);
        out.push(eta_cos * phi_sin);
        out.push(phi_cos);
    }

    out
}

pub fn gen_triangle_square(n: i32) -> (Vec<f32>, Vec<u16>) {
    let mut out = Vec::new();
    let points: Vec<(f32, f32)> = (0..n)
        .map(|x| 2.0 * std::f32::consts::PI * (x as f32) / n as f32)
        .map(|i| (i.cos() * 100.0, i.sin() * 100.0))
        .chain(vec![
            (0.0, 0.0),
            (5.0, 5.0),
            (5.0, -5.0),
            (-5.0, 5.0),
            (-5.0, -5.0),
        ])
        .collect();

    for &(x, y) in &points {
        out.push(x);
        out.push(y);
        out.push(0.0);
    }

    let denauy = Delaunay::triangulate(&points);

    let mut idxs = Vec::new();

    for p in denauy.triangles() {
        idxs.push(p.a as u16);
        idxs.push(p.b as u16);
        idxs.push(p.c as u16);
    }

    (out, idxs)
}

pub fn gen_cube() -> (Vec<f32>, Vec<f32>, Vec<u16>) {
    #[rustfmt::skip]
    let verts = vec![
        1.0, 1.0, 1.0, 
        1.0, -1.0, 1.0, 
        1.0, 1.0, -1.0,
        1.0, -1.0, -1.0, 
        
        -1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0, 
        -1.0, -1.0, 1.0, 
        -1.0, -1.0, -1.0, 
        
        1.0, 1.0, 1.0, 
        1.0, 1.0, -1.0,
        -1.0, 1.0, 1.0, 
        -1.0, 1.0, -1.0, 
        
        1.0, -1.0, 1.0, 
        -1.0, -1.0, 1.0, 
        1.0, -1.0, -1.0, 
        -1.0, -1.0, -1.0, 
        
        1.0, 1.0, 1.0, 
        -1.0, 1.0, 1.0, 
        1.0, -1.0, 1.0, 
        -1.0, -1.0, 1.0, 
        
        1.0, 1.0, -1.0, 
        1.0, -1.0, -1.0, 
        -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,
    ];

    let mut normals = Vec::new();

    let mut ids = Vec::new();

    for i in 0..6 {
        let (v1, v2, v3) = ((i * 4 + 0) * 3, (i * 4 + 1) * 3, (i * 4 + 2) * 3);
        let normal = normalize(calc_normal(
            Vector3::new(verts[v1 + 0], verts[v1 + 1], verts[v1 + 2]),
            Vector3::new(verts[v2 + 0], verts[v2 + 1], verts[v2 + 2]),
            Vector3::new(verts[v3 + 0], verts[v3 + 1], verts[v3 + 2]),
        ));
        let normal_ref = AsRef::<[f32; 3]>::as_ref(&normal);

        normals.extend_from_slice(normal_ref);
        normals.extend_from_slice(normal_ref);
        normals.extend_from_slice(normal_ref);
        normals.extend_from_slice(normal_ref);

        for v in &[0, 1, 2, 1, 3, 2] {
            ids.push(v + 4 * i as u16);
        }
    }

    (verts, normals, ids)
}

pub fn gen_circle(inner_diameter: f32, diamond_count: usize) -> Vec<f32> {
    let mut verts = Vec::new();

    let angle_per_diamond = std::f32::consts::PI * 2.0 / diamond_count as f32;
    let half_angle_per_diamond = angle_per_diamond * 0.5;

    let middle_diameter = (inner_diameter + 1.0) * 0.5;

    {
        let verts = &mut verts;
        let mut current_addition = 0;

        let mut add_point_on_circle = move |angle: f32, radius: f32| {
            let (y, x) = angle.sin_cos();
            verts.push(x * radius);
            verts.push(y * radius);
            verts.push(0.0);

            if current_addition / 3 % 2 == 0 {
                verts.extend_from_slice(&[0.0, 1.0, 0.0]);
            } else {
                verts.extend_from_slice(&[1.0, 0.0, 0.0]);
            }

            current_addition += 1;
        };

        for i in 0..diamond_count {
            let angle = angle_per_diamond * i as f32;

            let prev_angle = angle - half_angle_per_diamond;
            let next_angle = angle + half_angle_per_diamond;

            // Top diamond part
            add_point_on_circle(angle, 1.0);
            add_point_on_circle(next_angle, middle_diameter);
            add_point_on_circle(prev_angle, middle_diameter);

            // Add top triangle
            add_point_on_circle(angle, 1.0);
            add_point_on_circle(angle + angle_per_diamond, 1.0);
            add_point_on_circle(next_angle, middle_diameter);

            // Lower diamond part
            add_point_on_circle(angle, inner_diameter);
            add_point_on_circle(prev_angle, middle_diameter);
            add_point_on_circle(next_angle, middle_diameter);

            // Add lower triangle
            add_point_on_circle(next_angle, middle_diameter);
            add_point_on_circle(angle + angle_per_diamond, inner_diameter);
            add_point_on_circle(angle, inner_diameter);
        }
    }

    verts
}
