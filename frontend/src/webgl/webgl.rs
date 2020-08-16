use wasm_bindgen::JsCast;

use super::{
    buffer::{IndexBuffer, VertexArray, VertexBuffer, VertexBufferLayout},
    renderer::Renderer,
    shader::Uniform1f,
    Shader,
};
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;
use yew::services::resize::{ResizeService, ResizeTask, WindowDimensions};
use yew::services::ConsoleService;
use yew::services::{RenderService, Task};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

pub type Point = Vec<f32>;

// pub struct Triangulator {
//     idx: Vec<usize>,
//     verts: Vec<f32>,

//     cache: HashMap<Point, usize>,
// }

// impl Triangulator {
//     pub fn new() -> Self {
//         Triangulator {
//             idx: Vec::new(),
//             verts: Vec::new(),

//             cache: HashMap::new(),
//         }
//     }

//     pub fn idx(&self) -> &Vec<usize> {
//         &self.idx
//     }

//     pub fn verts(&self) -> &Vec<f32> {
//         &self.verts()
//     }

//     pub fn add_triangle(&mut self, p1: &Point, p2: &Point, p3: &Point) {}
// }

pub struct WebGl {
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,
    aspect: f32,

    renderer: Renderer,
    sphere_index: usize,

    _resize_task: ResizeTask,
}

pub enum Msg {
    Render(f64),
    Resize(WindowDimensions),
}

impl Component for WebGl {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _resize_task = ResizeService::new().register(link.callback(|dim| Msg::Resize(dim)));

        Self {
            canvas: None,
            gl: None,
            link,
            node_ref: NodeRef::default(),
            render_loop: None,
            aspect: 1.0,

            renderer: Renderer::new(),
            sphere_index: 0,
            _resize_task,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.canvas = Some(canvas);
        self.gl = Some(gl);

        if first_render {
            let render_frame = self.link.callback(Msg::Render);
            let handle = RenderService::request_animation_frame(render_frame);
            self.render_loop = Some(Box::new(handle));

            {
                // Setup size correctly
                let canvas = self.canvas.as_ref().unwrap();
                let gl = self.gl.as_ref().expect("GL Context not initialized!");

                let width = canvas.parent_element().unwrap().client_width();
                let height = canvas.parent_element().unwrap().client_height();

                canvas.set_width(width as u32);
                canvas.set_height(height as u32);
                gl.viewport(0, 0, width, height);

                self.aspect = width as f32 / height as f32;
            }

            if self.init_renderer().is_none() {
                ConsoleService::error("init_renderer returned None");
            };
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(timestamp) => {
                self.render_gl(timestamp);
            }
            Msg::Resize(WindowDimensions { width, height }) => {
                if let Some(ref mut canvas) = &mut self.canvas {
                    let gl = self.gl.as_ref().expect("GL Context not initialized!");

                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);
                    gl.viewport(0, 0, width, height);

                    self.aspect = width as f32 / height as f32;
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
           <canvas class="nav-body" ref={self.node_ref.clone()} />
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        ConsoleService::log("CHANGE FUNCTION CALL");
        true
    }
}

impl WebGl {
    fn init_renderer(&mut self) -> Option<()> {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let vert_source = include_str!("./basic.vert");
        let frag_source = include_str!("./basic.frag");

        let shader = Shader::single(gl, frag_source, vert_source, HashMap::new())?;

        let vertices = gen_generalized_spiral(700.0, 3.6);
        // let vertices = vec![
        //     0.5, -0.5, 0.0,
        //     0.5, 0.5, 0.0,
        //     -0.5, -0.5, 0.0,

        //     0.5, 0.5, 0.0,
        //     -0.5, -0.5, 0.0,
        //     -0.5, 0.5, 0.0,
        // ];

        let indices: Vec<u16> = (0..vertices.len() / 3).map(|x| x as u16).collect();
        let vertex_buffer = VertexBuffer::vertex_buffer(gl, vertices)?;
        let index_buffer = IndexBuffer::index_buffer(gl, indices)?;

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);

        self.sphere_index = self
            .renderer
            .add_to_draw(index_buffer, vao, shader, None, 0);

        Some(())
    }

    fn render_gl(&mut self, timestamp: f64) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

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
        });

        self.renderer.render(gl);

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);

        // A reference to the new handle must be retained for the next render to run.
        self.render_loop = Some(Box::new(handle));
    }
}

// struct Rect([f32; 3], [f32; 3], [f32; 3], [f32; 3]);

pub fn gen_sphere_icosahedral(n: i32) -> Vec<f32> {
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
