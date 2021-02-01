use crate::scene::Scene;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::engine::{CameraHandle};
use crate::gl::GL;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct WebGl {
    canvas: HtmlCanvasElement,
    gl: GL,

    scene: Scene,
}

unsafe impl Send for WebGl {}

unsafe impl Sync for WebGl {}

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

        let gl: GL = {
            let webgl: web_sys::WebGlRenderingContext = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            GL { gl: webgl }
        };

        Ok(Self {
            canvas,
            gl,

            scene: Scene::new(),
        })
    }

    pub fn camera_handle(&self) -> CameraHandle {
        self.scene.camera_handle()
    }

    pub fn resize(&mut self) {
        let width = self.canvas.parent_element().unwrap().client_width();
        let height = self.canvas.parent_element().unwrap().client_height();

        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.gl.viewport(0, 0, width, height);

        self.scene.camera_handle().set_aspect(width as f32 / height as f32);
    }

    pub async fn init_renderer(self) -> Result<WebGl, JsValue> {
        let WebGl {scene, gl, canvas} = self;

        // Clear the canvas AND the depth buffer.
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // Turn on culling. By default backfacing triangles
        // will be culled.
        gl.enable(GL::CULL_FACE);

        // Enable the depth buffer
        gl.enable(GL::DEPTH_TEST);

        let scene = scene.init_renderer(&gl).await?;
        let mut webgl = Self { gl, scene, canvas};
        webgl.resize();
        Ok(webgl)
    }

    pub fn handle_client_update(&mut self, _val: &JsValue) {
        // TODO
    }

    pub fn update(&mut self, dt: f64) -> Result<(), JsValue> {
        self.scene.update(&self.gl, dt)?;
        Ok(())
    }

    pub fn render_gl(&mut self) -> Result<(), JsValue> {
        self.scene.render_gl(&self.gl)?;
        Ok(())
    }

    pub fn handle_click(&mut self, _x: f32, _y: f32) {
        // let (origin, direction) = self.camera.handle_click(x, y);
        // self.universe.handle_click(origin, direction);
    }
}
