use wasm_bindgen::JsCast;

use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;
use yew::services::resize::{ResizeService, WindowDimensions, ResizeTask};
use yew::services::{RenderService, Task};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};
use yew::services::ConsoleService;

use std::collections::HashMap;

pub type Point = Vec<f32>;

pub struct Triangulator {
    idx: Vec<usize>,
    verts: Vec<f32>,

    cache: HashMap<Point, usize>,
}

impl Triangulator {
    pub fn new() -> Self {
        Triangulator {
            idx: Vec::new(),
            verts: Vec::new(),

            cache: HashMap::new(),
        }
    }

    pub fn idx(&self) -> &Vec<usize> {
        &self.idx
    }

    pub fn verts(&self) -> &Vec<f32> {
        &self.verts()
    }

    pub fn add_triangle(&mut self, p1: &Point, p2: &Point, p3: &Point) {

    }
}

pub struct WebGl {
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,
    aspect: f32,

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
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(timestamp) => {
                self.render_gl(timestamp);
            },
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
    fn render_gl(&mut self, timestamp: f64) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");


        let vertices = gen_generalized_spiral(700.0, 3.6);
        // let vertices = gen_sphere_icosahedral(0);

        let vertex_buffer = gl.create_buffer().unwrap();
        // let verts = js_sys::Float32Array::from(vertices.as_slice());
        let verts = unsafe { js_sys::Float32Array::view(vertices.as_slice() )};

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, &vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, &frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        let u_aspect = gl.get_uniform_location(&shader_program, "u_aspect");
        gl.uniform1f(u_aspect.as_ref(), self.aspect);

        gl.draw_arrays(GL::TRIANGLES, 0, vertices.len() as i32 / 3);

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);

        // A reference to the new handle must be retained for the next render to run.
        self.render_loop = Some(Box::new(handle));
    }
}

struct Rect([f32;3], [f32;3], [f32;3], [f32;3]);

pub fn gen_sphere_icosahedral(n: i32) -> Vec<f32> {
    let rho = 0.5 * ( 1.0 + 5.0_f32.sqrt());

    // let (ptr, ptl, pbr, pbl) = ();

    vec! [
        0.0, 1.0, rho,
        0.0, -1.0, rho,
        rho, 0.0, 1.0,

        rho, 0.0, 1.0,
        0.0, -1.0, rho,
        rho, -1.0, 0.0,

        rho, -1.0, 0.0,
        0.0, -1.0, rho,
        rho, 1.0, 0.0,

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
    let n_sqrt = c / (n + 1  as f32).sqrt();

    for k in 2..(n as u32) {
        let k = k as f32;

        let hk = 2.0*(k-1.0) / n -1.0;

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
