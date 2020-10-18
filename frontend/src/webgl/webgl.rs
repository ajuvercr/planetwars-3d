use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::settings::TheseSettings;
use pw_settings::SettingsTrait;

use super::super::models;
use super::{
    buffer::{BufferHandle, IndexBuffer, VertexArray, VertexBuffer, VertexBufferLayout},
    renderer::Renderer,
    Shader,
};
use crate::{
    entity::{Camera, CameraHandle, Entity},
    renderer::DefaultRenderable,
    set_settings,
    uniform::UniformsHandle,
    uniform::{Uniform3f, UniformMat4},
};
use cgmath::Vector3;
use std::collections::HashMap;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;

#[wasm_bindgen]
pub struct WebGl {
    canvas: HtmlCanvasElement,
    gl: GL,

    circle_handle: Option<BufferHandle<Vec<f32>>>,

    uniform_handles: Vec<UniformsHandle>,

    entities: Vec<Entity>,

    camera: Camera,
    camera_handle: CameraHandle,

    renderer: Renderer,
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

        let camera = Camera::new();
        let camera_handle = camera.handle();
        // camera_handle.reset_position(0.0, 0.0, 5.0);

        Ok(Self {
            canvas,
            gl,

            uniform_handles: Vec::new(),
            entities: Vec::new(),

            camera,
            camera_handle,

            circle_handle: None,

            renderer: Renderer::new(),
        })
    }

    pub fn camera_handle(&self) -> CameraHandle {
        self.camera_handle.clone()
    }

    pub fn resize(&mut self) {
        let width = self.canvas.parent_element().unwrap().client_width();
        let height = self.canvas.parent_element().unwrap().client_height();

        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.gl.viewport(0, 0, width, height);

        self.camera_handle.set_aspect(width as f32 / height as f32);
    }

    pub async fn init_renderer(mut self) -> Result<WebGl, JsValue> {
        self.resize();

        let gl = &self.gl;

        // Clear the canvas AND the depth buffer.
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // Turn on culling. By default backfacing triangles
        // will be culled.
        gl.enable(GL::CULL_FACE);

        // Enable the depth buffer
        gl.enable(GL::DEPTH_TEST);

        let vert_source = fetch("shaders/basic.vert").await?;
        let frag_source = fetch("shaders/basic.frag").await?;

        let shader_factory = Shader::factory(frag_source, vert_source);

        // Setup sphere
        let sphere_entity = Entity::default()
            .with_position(Vector3::new(0.0, 0.0, -500.0))
            .with_hom_scale(50.0)
            .with_ang_speed(Vector3::new(30.0, 60.0, 0.0)); //.with_speed(Vector3::new(5.0, 0.0, 10.0));
        self.entities.push(sphere_entity);

        let vertices = models::gen_sphere_icosahedral(5.0);
        console_log!("Verts count {}", vertices.len() / 6);

        let vertex_buffer =
            VertexBuffer::vertex_buffer(gl, vertices).ok_or("Failed to get vertices")?;

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);
        layout.push(GL::FLOAT, 3, 4, "a_normal", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);

        let shader = shader_factory
            .create_shader(gl, HashMap::new())
            .ok_or("failed to create new shader")?;
        let sphere_renderable = DefaultRenderable::new(None, vao, shader, None);
        self.uniform_handles.push(sphere_renderable.handle());
        self.renderer.add_renderable(sphere_renderable, 0);

        // Setup sphere2
        let sphere_entity = Entity::default()
            .with_position(Vector3::new(-500.0, 0.0, -500.0))
            .with_hom_scale(50.0)
            .with_ang_speed(Vector3::new(30.0, 60.0, 0.0)); //.with_speed(Vector3::new(5.0, 0.0, 10.0));
        self.entities.push(sphere_entity);

        let vertices = models::gen_sphere_icosahedral(1.0);
        console_log!("Verts count {}", vertices.len() / 6);

        let vertex_buffer =
            VertexBuffer::vertex_buffer(gl, vertices).ok_or("Failed to get vertices")?;

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);
        layout.push(GL::FLOAT, 3, 4, "a_normal", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);

        let shader = shader_factory
            .create_shader(gl, HashMap::new())
            .ok_or("failed to create new shader")?;
        let sphere_renderable = DefaultRenderable::new(None, vao, shader, None);
        self.uniform_handles.push(sphere_renderable.handle());
        self.renderer.add_renderable(sphere_renderable, 0);

        // Setup cube
        let cube_entity = Entity::default()
            .with_position(Vector3::new(500.0, 0.0, -500.0))
            .with_hom_scale(50.0)
            .with_ang_speed(Vector3::new(10.0, 30.0, 0.0)); //.with_speed(Vector3::new(5.0, 0.0, 10.0));
        self.entities.push(cube_entity);

        let (vertices, normals, indx) = models::gen_cube();
        console_log!("Verts count {}, index count {}", vertices.len(), indx.len());
        let index_buffer = IndexBuffer::index_buffer(gl, indx).ok_or("Failed to get indicies")?;
        let vertex_buffer =
            VertexBuffer::vertex_buffer(gl, vertices).ok_or("Failed to get vertices")?;
        let normal_buffer =
            VertexBuffer::vertex_buffer(gl, normals).ok_or("Failed to get vertices")?;

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);

        let mut normal_layout = VertexBufferLayout::new();
        normal_layout.push(GL::FLOAT, 3, 4, "a_normal", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);
        vao.add_buffer(normal_buffer, normal_layout);

        let shader = shader_factory
            .create_shader(gl, HashMap::new())
            .ok_or("failed to create new shader")?;
        let cube_renderable = DefaultRenderable::new(index_buffer, vao, shader, None);
        self.uniform_handles.push(cube_renderable.handle());
        self.renderer.add_renderable(cube_renderable, 0);

        // Setup floor
        let cube_entity = Entity::default()
            .with_position(Vector3::new(0.0, -100.0, 0.0))
            .with_scale(5000.0, 5.0, 5000.0);
        self.entities.push(cube_entity);

        let (vertices, normals, indx) = models::gen_cube();
        console_log!("Verts count {}, index count {}", vertices.len(), indx.len());
        let index_buffer = IndexBuffer::index_buffer(gl, indx).ok_or("Failed to get indicies")?;
        let vertex_buffer =
            VertexBuffer::vertex_buffer(gl, vertices).ok_or("Failed to get vertices")?;
        let normal_buffer =
            VertexBuffer::vertex_buffer(gl, normals).ok_or("Failed to get vertices")?;

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);

        let mut normal_layout = VertexBufferLayout::new();
        normal_layout.push(GL::FLOAT, 3, 4, "a_normal", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);
        vao.add_buffer(normal_buffer, normal_layout);

        let shader = shader_factory
            .create_shader(gl, HashMap::new())
            .ok_or("failed to create new shader")?;
        let cube_renderable = DefaultRenderable::new(index_buffer, vao, shader, None);
        self.uniform_handles.push(cube_renderable.handle());
        self.renderer.add_renderable(cube_renderable, 0);

        // Setup fancy circle
        let circle_thing = Entity::default()
            .with_position(Vector3::new(0.0, 0.0, -200.0))
            .with_hom_scale(50.0);
        self.entities.push(circle_thing);

        let vertices = models::gen_circle(0.5, 8 * 4);
        let vertex_buffer =
            VertexBuffer::vertex_buffer(gl, vertices).ok_or("Failed to get vertices")?;

        self.circle_handle = Some(vertex_buffer.handle());

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);
        layout.push(GL::FLOAT, 3, 4, "a_color", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);

        // TODO: This needs to change too
        let shader = Shader::single(
            gl,
            &fetch("shaders/circle.frag").await?,
            &fetch("shaders/circle.vert").await?,
            HashMap::new(),
        )
        .ok_or("failed to create new shader")?;
        let circle_thing = DefaultRenderable::new(None, vao, shader, None);
        self.uniform_handles.push(circle_thing.handle());
        self.renderer.add_renderable(circle_thing, 0);

        for uniform_handle in &mut self.uniform_handles {
            uniform_handle.single(
                "u_reverseLightDirection",
                Uniform3f::new(0.28735632183908044, 0.4022988505747126, 0.5747126436781609),
            );
        }

        unsafe {
            console_log!(
                "Sending settings {:?}",
                JsValue::from_serde(&TheseSettings::new_settings())
            );
            set_settings(
                JsValue::from_serde(&TheseSettings::new().to_settings())
                    .map_err(|_| "Serde Failed")?,
            )
        };

        Ok(self)
    }

    pub fn handle_client_update(&mut self, val: &JsValue) {
        // if let Some(mut settings) = val.into_serde::<TheseSettings>().ok() {
        //     console_log!("Settings update {:?}", settings);
        //     settings.count += 1.0;
        //     let js_value = JsValue::from_serde(&settings.to_settings())
        //         .map_err(|_| "Serde Failed")
        //         .unwrap();
        //     println!("js value {:?}", js_value);
        //     unsafe { set_settings(js_value) };

        //     if let Some(handle) = &self.circle_handle {
        //         handle.reset(models::gen_circle(
        //             settings.inner_diameter,
        //             settings.count as usize,
        //         ));
        //     }
        // }
    }

    pub fn update(&mut self, dt: f64) -> Result<(), JsValue> {
        self.camera.update().ok_or("Couldn't update camera")?;
        let gl = &self.gl;

        for (uniform_handle, entity) in self.uniform_handles.iter().zip(self.entities.iter_mut()) {
            entity.update(dt as f32);

            uniform_handle.single(
                "u_worldViewProjection",
                UniformMat4::new_mat4(self.camera.world_view_projection_matrix()),
            );
            uniform_handle.single("u_world", UniformMat4::new_mat4(entity.world_matrix()));
        }

        self.renderer
            .update(gl)
            .ok_or("Renderer didn't update well")?;
        Ok(())
    }

    pub fn render_gl(&mut self) -> Result<(), JsValue> {
        self.renderer.render(&self.gl);
        Ok(())
    }
}
