use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::super::sphere;
use super::{
    buffer::{IndexBuffer, VertexArray, VertexBuffer, VertexBufferLayout},
    renderer::Renderer,
    shader::{Uniform1f, Uniform2f},
    Shader,
};
use crate::shader::UniformMat4;
use cgmath::{perspective, Deg, Matrix4, SquareMatrix, Vector3};
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen_futures::JsFuture;

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

async fn fetch(url: &str) -> Result<String, JsValue> {
    use web_sys::{Request, RequestInit, RequestMode, Response};

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    if !resp.ok() {
        return Err(resp.into());
    }

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap();

    Ok(text)
}

#[wasm_bindgen]
impl WebGl {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: String) -> Result<WebGl, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas: HtmlCanvasElement = document
            .get_element_by_id(&canvas_id)
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

    pub async fn init_renderer(mut self) -> Result<WebGl, JsValue> {
        // TODO add resize
        let canvas = &self.canvas;
        let gl = &self.gl;

        let width = canvas.parent_element().unwrap().client_width();
        let height = canvas.parent_element().unwrap().client_height();

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        gl.viewport(0, 0, width, height);

        // Clear the canvas AND the depth buffer.
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // Turn on culling. By default backfacing triangles
        // will be culled.
        gl.enable(GL::CULL_FACE);

        // Enable the depth buffer
        gl.enable(GL::DEPTH_TEST);

        self.aspect = width as f32 / height as f32;

        let vert_source =  fetch("shaders/basic.vert").await?;
        let frag_source =  fetch("shaders/basic.frag").await?;

        let shader = Shader::single(gl, &frag_source, &vert_source, HashMap::new())
            .ok_or("Failed create shader")?;

        let (vertices, indices, layers) = sphere::gen_sphere_icosahedral(5.0);
        console_log!("{} verts, {} indices", vertices.len(), indices.len());

        let vertex_buffer =
            VertexBuffer::vertex_buffer(gl, vertices).ok_or("Failed to get vertices")?;
        let layer_buffer = VertexBuffer::vertex_buffer(gl, layers).ok_or("Failed to get layers")?;
        let index_buffer =
            IndexBuffer::index_buffer(gl, indices).ok_or("Failed to get indicies")?;

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);

        let mut layer_layout = VertexBufferLayout::new();
        layer_layout.push(GL::FLOAT, 1, 4, "a_layer", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);
        vao.add_buffer(layer_buffer, layer_layout);

        self.sphere_index = self
            .renderer
            .add_to_draw(index_buffer, vao, shader, None, 0);

        Ok(self)
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        let gl = &self.gl;
        self.renderer
            .update(gl)
            .ok_or("Renderer didn't update well")?;
        Ok(())
    }

    pub fn render_gl(&mut self, timestamp: f64) -> Result<(), JsValue> {
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

            context.insert("u_viewport".to_string(), Box::new(Uniform2f::new(5.0, 5.0)));

            let projection_matrix = perspective(Deg(90.0), aspect, 0.2, 2000.0);

            // let camera_matrix = Matrix4::from_angle_y(Rad(std::f32::consts::PI));
            let camera_matrix = Matrix4::identity();
            let camera_matrix =
                camera_matrix + Matrix4::from_translation(Vector3::new(0.0, 0.0, 5.0));
            let view_matrix = camera_matrix.invert().unwrap();

            let view_projection_matrix = projection_matrix * view_matrix;

            context.insert(
                "u_matrix".to_string(),
                Box::new(UniformMat4::new_mat4(view_projection_matrix)),
            );
        });

        self.renderer.render(gl);

        Ok(())
    }
}
