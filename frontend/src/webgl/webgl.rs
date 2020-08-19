use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::{
    buffer::{IndexBuffer, VertexArray, VertexBuffer, VertexBufferLayout},
    renderer::Renderer,
    shader::{Uniform1f, Uniform2f},
    Shader,
};
use crate::delaunay::Delaunay;
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;

#[wasm_bindgen]
pub struct WebGl {
    canvas: HtmlCanvasElement,
    gl: GL,
    aspect: f32,

    renderer: Renderer,
    sphere_index: usize,
}

unsafe impl Send for WebGl {}

unsafe impl Sync for WebGl {}

#[wasm_bindgen]
impl WebGl {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WebGl, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas: HtmlCanvasElement = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into()
            .unwrap();

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        Ok(Self {
            canvas,
            gl,
            aspect: 1.0,

            renderer: Renderer::new(),
            sphere_index: 0,
        })
    }

    pub fn init_renderer(&mut self) -> Result<JsValue, JsValue> {
        // TODO add resize
        let canvas = &self.canvas;
        let gl = &self.gl;

        let width = canvas.parent_element().unwrap().client_width();
        let height = canvas.parent_element().unwrap().client_height();

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        gl.viewport(0, 0, width, height);

        self.aspect = width as f32 / height as f32;

        let vert_source = include_str!("./basic.vert");
        let frag_source = include_str!("./basic.frag");

        let shader = Shader::single(gl, frag_source, vert_source, HashMap::new())
            .ok_or("Failed create shader")?;

        let (vertices, indices) = gen_triangle_square(10);
        let vertex_buffer =
            VertexBuffer::vertex_buffer(gl, vertices).ok_or("Failed to get vertices")?;
        let index_buffer =
            IndexBuffer::index_buffer(gl, indices).ok_or("Failed to get indicies")?;

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);

        self.sphere_index = self
            .renderer
            .add_to_draw(index_buffer, vao, shader, None, 0);

        Ok("nice".into())
    }

    pub fn render_gl(&mut self, timestamp: f64) {
        let gl = &self.gl;

        let aspect = self.aspect;
        self.renderer.update_uniforms(self.sphere_index, 0, |c| {
            if c.is_none() {
                *c = Some(HashMap::new());
            }

            let context = c.as_mut().unwrap();
            context.insert(
                "u_time".to_string(),
                Box::new(Uniform1f::new(timestamp as f32)),
            );

            context.insert("u_aspect".to_string(), Box::new(Uniform1f::new(aspect)));

            context.insert(
                "u_viewport".to_string(),
                Box::new(Uniform2f::new(100.0, 100.0)),
            );
        });

        self.renderer.render(gl);
    }
}

// struct Rect([f32; 3], [f32; 3], [f32; 3], [f32; 3]);

pub fn gen_sphere_icosahedral(_n: i32) -> Vec<f32> {
    let rho = 0.5 * (1.0 + 5.0_f32.sqrt());

    // let (ptr, ptl, pbr, pbl) = ();

    vec![
        0.0, 1.0, rho, 0.0, -1.0, rho, rho, 0.0, 1.0, rho, 0.0, 1.0, 0.0, -1.0, rho, rho, -1.0,
        0.0, rho, -1.0, 0.0, 0.0, -1.0, rho, rho, 1.0,
        0.0,
        // rho, 1.0, 0.0,
        // 0.0, -1.0, rho,
        // rho, -1.0, 0.0,

        // rho, 0.0, -1.0,

        // 0.0, -1.0, -rho,
        // 0.0, 1.0, -rho,

        // -rho, 0.0, 1.0,
        // -rho, 0.0, -1.0,

        // 1.0, rho, 0.0,
        // -1.0, rho, 0.0,
        // -1.0, -rho, 0.0,
        // 1.0, -rho, 0.0,
    ]
}

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
