#![feature(prelude_import)]
#![feature(pattern)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use wasm_bindgen::prelude::*;
mod webgl {
    mod webgl {
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
        pub struct WebGl {
            canvas: HtmlCanvasElement,
            gl: GL,
            aspect: f32,
            renderer: Renderer,
            sphere_index: usize,
        }
        #[allow(clippy::all)]
        impl wasm_bindgen::describe::WasmDescribe for WebGl {
            fn describe() {
                use wasm_bindgen::__wbindgen_if_not_std;
                use wasm_bindgen::describe::*;
                inform(RUST_STRUCT);
                inform(5u32);
                inform(87u32);
                inform(101u32);
                inform(98u32);
                inform(71u32);
                inform(108u32);
            }
        }
        #[allow(clippy::all)]
        impl wasm_bindgen::convert::IntoWasmAbi for WebGl {
            type Abi = u32;
            fn into_abi(self) -> u32 {
                use wasm_bindgen::__rt::std::boxed::Box;
                use wasm_bindgen::__rt::WasmRefCell;
                Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
            }
        }
        #[allow(clippy::all)]
        impl wasm_bindgen::convert::FromWasmAbi for WebGl {
            type Abi = u32;
            unsafe fn from_abi(js: u32) -> Self {
                use wasm_bindgen::__rt::std::boxed::Box;
                use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
                let ptr = js as *mut WasmRefCell<WebGl>;
                assert_not_null(ptr);
                let js = Box::from_raw(ptr);
                (*js).borrow_mut();
                js.into_inner()
            }
        }
        #[allow(clippy::all)]
        impl wasm_bindgen::__rt::core::convert::From<WebGl> for wasm_bindgen::JsValue {
            fn from(value: WebGl) -> Self {
                let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
                #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
                unsafe fn __wbg_webgl_new(_: u32) -> u32 {
                    {
                        ::std::rt::begin_panic(
                            "cannot convert to JsValue outside of the wasm target",
                        )
                    }
                }
                unsafe {
                    <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                        __wbg_webgl_new(ptr),
                    )
                }
            }
        }
        #[allow(clippy::all)]
        impl wasm_bindgen::convert::RefFromWasmAbi for WebGl {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::Ref<'static, WebGl>;
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<WebGl>;
                wasm_bindgen::__rt::assert_not_null(js);
                (*js).borrow()
            }
        }
        #[allow(clippy::all)]
        impl wasm_bindgen::convert::RefMutFromWasmAbi for WebGl {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RefMut<'static, WebGl>;
            unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<WebGl>;
                wasm_bindgen::__rt::assert_not_null(js);
                (*js).borrow_mut()
            }
        }
        impl wasm_bindgen::convert::OptionIntoWasmAbi for WebGl {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl wasm_bindgen::convert::OptionFromWasmAbi for WebGl {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        unsafe impl Send for WebGl {}
        unsafe impl Sync for WebGl {}
        impl WebGl {
            pub fn new(canvas_id: &str) -> Result<WebGl, JsValue> {
                #[allow(non_snake_case)]
                #[allow(clippy::all)]
                pub extern "C" fn __wasm_bindgen_generated_WebGl_new(
                    arg0: <str as wasm_bindgen::convert::RefFromWasmAbi>::Abi,
                ) -> <Result<WebGl, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi
                {
                    let _ret = {
                        let arg0 = unsafe {
                            <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(arg0)
                        };
                        let arg0 = &*arg0;
                        WebGl::new(arg0)
                    };
                    <Result<WebGl, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                        _ret,
                    )
                }
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
            pub fn init_renderer(&mut self) -> Option<String> {
                #[allow(non_snake_case)]
                #[allow(clippy::all)]
                pub extern "C" fn __wasm_bindgen_generated_WebGl_init_renderer(
                    me: u32,
                ) -> <Option<String> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                    let _ret = {
                        let mut me = unsafe {
                            <WebGl as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                                me,
                            )
                        };
                        let me = &mut *me;
                        me.init_renderer()
                    };
                    <Option<String> as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                }
                let canvas = &self.canvas;
                let gl = &self.gl;
                let width = canvas.parent_element().unwrap().client_width();
                let height = canvas.parent_element().unwrap().client_height();
                canvas.set_width(width as u32);
                canvas.set_height(height as u32);
                gl.viewport(0, 0, width, height);
                self.aspect = width as f32 / height as f32;
                let vert_source = "precision mediump float;\r\n\r\nuniform float u_time;\r\nuniform float u_aspect;\r\nuniform vec2 u_viewport;\r\n\r\nattribute vec3 a_position;\r\n\r\nvoid main() {\r\n\r\n    float time = u_time * 0.001;\r\n    mat3 rot2 = mat3(\r\n        cos(time), 0.0, sin(time),\r\n        0.0, 1.0, 0.0,\r\n        -sin(time), 0.0, cos(time));\r\n\r\n    time = time * 0.0;\r\n    mat3 rot = mat3(\r\n        1.0, 0.0, 0.0,\r\n        0.0, cos(time), -sin(time),\r\n        0.0, sin(time), cos(time)\r\n    );\r\n\r\n    vec4 position = vec4(a_position * rot * rot2, 1.0);\r\n    vec2 scale = vec2(u_aspect, 1.0);\r\n    gl_Position = vec4(position.xy / u_viewport / scale, 1.0, 1.0);\r\n}\r\n" ;
                let frag_source = "precision mediump float;\r\n\r\nuniform float u_time;\r\n\r\nvoid main() {\r\n    float r = sin(u_time * 0.0003);\r\n    float g = sin(u_time * 0.0005);\r\n    float b = sin(u_time * 0.0007);\r\n\r\n    gl_FragColor = vec4(r, g, b, 1.0);\r\n    // gl_FragColor = vec4(1.0);\r\n}\r\n" ;
                let shader = Shader::single(gl, frag_source, vert_source, HashMap::new())?;
                let (vertices, indices) = gen_triangle_square(30);
                let vertex_buffer = VertexBuffer::vertex_buffer(gl, vertices)?;
                let index_buffer = IndexBuffer::index_buffer(gl, indices)?;
                let mut layout = VertexBufferLayout::new();
                layout.push(GL::FLOAT, 3, 4, "a_position", false);
                let mut vao = VertexArray::new();
                vao.add_buffer(vertex_buffer, layout);
                self.sphere_index = self
                    .renderer
                    .add_to_draw(index_buffer, vao, shader, None, 0);
                Some("nice".to_string())
            }
            pub fn render_gl(&mut self, timestamp: f64) {
                #[allow(non_snake_case)]
                #[allow(clippy::all)]
                pub extern "C" fn __wasm_bindgen_generated_WebGl_render_gl(
                    me: u32,
                    arg1: <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi,
                ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                    let _ret = {
                        let mut me = unsafe {
                            <WebGl as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                                me,
                            )
                        };
                        let me = &mut *me;
                        let arg1 =
                            unsafe { <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1) };
                        me.render_gl(arg1)
                    };
                    <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                }
                let canvas = &self.canvas;
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
        pub fn gen_sphere_icosahedral(_n: i32) -> Vec<f32> {
            let rho = 0.5 * (1.0 + 5.0_f32.sqrt());
            <[_]>::into_vec(box [
                0.0, 1.0, rho, 0.0, -1.0, rho, rho, 0.0, 1.0, rho, 0.0, 1.0, 0.0, -1.0, rho, rho,
                -1.0, 0.0, rho, -1.0, 0.0, 0.0, -1.0, rho, rho, 1.0, 0.0,
            ])
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
                .chain(<[_]>::into_vec(box [
                    (0.0, 0.0),
                    (5.0, 5.0),
                    (5.0, -5.0),
                    (-5.0, 5.0),
                    (-5.0, -5.0),
                ]))
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
    }
    pub use webgl::*;
    mod shader {
        use web_sys::WebGlRenderingContext as GL;
        use web_sys::*;
        use std::collections::HashMap;
        fn load_shader(
            gl: &GL,
            shader_source: &str,
            shader_type: u32,
        ) -> Option<web_sys::WebGlShader> {
            let shader = gl.create_shader(shader_type)?;
            gl.shader_source(&shader, shader_source);
            gl.compile_shader(&shader);
            let compiled = gl
                .get_shader_parameter(&shader, GL::COMPILE_STATUS)
                .as_bool()
                .unwrap();
            if !compiled {
                crate::log(
                    &::core::fmt::Arguments::new_v1(
                        &["*** Error compiling shader \'", "\': "],
                        &match (&shader_source, &gl.get_shader_info_log(&shader).unwrap()) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    )
                    .to_string(),
                );
                gl.delete_shader(Some(&shader));
                return None;
            }
            Some(shader)
        }
        fn create_program(
            gl: &GL,
            shaders: Vec<web_sys::WebGlShader>,
        ) -> Option<web_sys::WebGlProgram> {
            let program = gl.create_program()?;
            shaders
                .iter()
                .for_each(|shader| gl.attach_shader(&program, shader));
            gl.link_program(&program);
            let linked = gl
                .get_program_parameter(&program, GL::LINK_STATUS)
                .as_bool()
                .unwrap();
            if !linked {
                crate::log(
                    &::core::fmt::Arguments::new_v1(
                        &["*** Error linking program \'", "\': "],
                        &match (&program, &gl.get_program_info_log(&program).unwrap()) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    )
                    .to_string(),
                );
                gl.delete_program(Some(&program));
                return None;
            }
            Some(program)
        }
        pub struct ShaderFactory {
            frag_source: String,
            vert_source: String,
        }
        impl ShaderFactory {
            pub fn new(frag_source: String, vert_source: String) -> Self {
                Self {
                    frag_source,
                    vert_source,
                }
            }
            pub fn create_shader(&self, gl: &GL, context: HashMap<String, &str>) -> Option<Shader> {
                Shader::single(gl, &self.frag_source, &self.vert_source, context)
            }
        }
        pub struct Shader {
            shader: web_sys::WebGlProgram,
            uniform_cache: HashMap<String, WebGlUniformLocation>,
            attrib_cache: HashMap<String, i32>,
        }
        impl Shader {
            fn new(shader: web_sys::WebGlProgram) -> Self {
                Self {
                    shader,
                    uniform_cache: HashMap::new(),
                    attrib_cache: HashMap::new(),
                }
            }
            pub fn single(
                gl: &GL,
                frag_source: &str,
                vert_source: &str,
                context: HashMap<String, &str>,
            ) -> Option<Self> {
                let mut frag = frag_source.to_string();
                let mut vert = vert_source.to_string();
                for (key, value) in context {
                    frag = frag.replace(
                        &{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["$"],
                                &match (&key,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ));
                            res
                        },
                        &value,
                    );
                    vert = vert.replace(
                        &{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["$"],
                                &match (&key,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ));
                            res
                        },
                        &value,
                    );
                }
                let shaders = <[_]>::into_vec(box [
                    load_shader(gl, &vert, GL::VERTEX_SHADER)?,
                    load_shader(gl, &frag, GL::FRAGMENT_SHADER)?,
                ]);
                let program = create_program(gl, shaders)?;
                Some(Shader::new(program))
            }
            pub fn factory(frag_source: String, vert_source: String) -> ShaderFactory {
                ShaderFactory::new(frag_source, vert_source)
            }
            pub fn bind(&self, gl: &GL) {
                gl.use_program(Some(&self.shader));
            }
            pub fn get_uniform_location(
                &mut self,
                gl: &GL,
                name: &str,
            ) -> Option<WebGlUniformLocation> {
                if let Some(location) = self.uniform_cache.get(name) {
                    Some(location.clone())
                } else {
                    let location = gl.get_uniform_location(&self.shader, name)?;
                    self.uniform_cache
                        .insert(name.to_string(), location.clone());
                    Some(location)
                }
            }
            pub fn get_attrib_location(&mut self, gl: &GL, name: &str) -> Option<i32> {
                if let Some(location) = self.attrib_cache.get(name) {
                    Some(*location)
                } else {
                    let location = gl.get_attrib_location(&self.shader, name);
                    self.attrib_cache.insert(name.to_string(), location);
                    Some(location)
                }
            }
            pub fn uniform(
                &mut self,
                gl: &GL,
                name: &str,
                uniform: &Box<dyn Uniform>,
            ) -> Option<()> {
                self.bind(gl);
                let loc = self.get_uniform_location(gl, name)?;
                uniform.set_uniform(gl, &loc);
                Some(())
            }
            pub unsafe fn drop_ref(&self, gl: &GL) {
                gl.delete_program(Some(&self.shader));
            }
            pub fn drop(self, gl: &GL) {
                gl.delete_program(Some(&self.shader));
            }
        }
        use std::{fmt::Debug, ops::Deref};
        pub trait Uniform: Debug {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation);
        }
        pub struct Uniform2fv<A: Deref<Target = [f32]>> {
            data: A,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<A: ::core::fmt::Debug + Deref<Target = [f32]>> ::core::fmt::Debug for Uniform2fv<A> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Uniform2fv {
                        data: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Uniform2fv");
                        let _ = debug_trait_builder.field("data", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl<A: Deref<Target = [f32]>> Uniform2fv<A> {
            pub fn new(data: A) -> Self {
                Self { data }
            }
        }
        impl<A: Deref<Target = [f32]> + Debug> Uniform for Uniform2fv<A> {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform2fv_with_f32_array(Some(location), self.data.deref());
            }
        }
        pub struct Uniform3fv<A: Deref<Target = [f32]>> {
            data: A,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<A: ::core::fmt::Debug + Deref<Target = [f32]>> ::core::fmt::Debug for Uniform3fv<A> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Uniform3fv {
                        data: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Uniform3fv");
                        let _ = debug_trait_builder.field("data", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl<A: Deref<Target = [f32]>> Uniform3fv<A> {
            pub fn new(data: A) -> Self {
                Self { data }
            }
        }
        impl<A: Deref<Target = [f32]> + Debug> Uniform for Uniform3fv<A> {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform3fv_with_f32_array(Some(location), self.data.deref());
            }
        }
        pub struct Uniformifv<A: Deref<Target = [i32]>> {
            data: A,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<A: ::core::fmt::Debug + Deref<Target = [i32]>> ::core::fmt::Debug for Uniformifv<A> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Uniformifv {
                        data: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Uniformifv");
                        let _ = debug_trait_builder.field("data", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl<A: Deref<Target = [i32]>> Uniformifv<A> {
            pub fn new(data: A) -> Self {
                Self { data }
            }
        }
        impl<A: Deref<Target = [i32]> + Debug> Uniform for Uniformifv<A> {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform1iv_with_i32_array(Some(location), self.data.deref());
            }
        }
        pub struct Uniform4f {
            x: f32,
            y: f32,
            z: f32,
            w: f32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Uniform4f {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Uniform4f {
                        x: ref __self_0_0,
                        y: ref __self_0_1,
                        z: ref __self_0_2,
                        w: ref __self_0_3,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Uniform4f");
                        let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("y", &&(*__self_0_1));
                        let _ = debug_trait_builder.field("z", &&(*__self_0_2));
                        let _ = debug_trait_builder.field("w", &&(*__self_0_3));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Uniform4f {
            pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
                Self { x, y, z, w }
            }
        }
        impl Uniform for Uniform4f {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform4f(Some(location), self.x, self.y, self.z, self.w);
            }
        }
        pub struct Uniform3f {
            x: f32,
            y: f32,
            z: f32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Uniform3f {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Uniform3f {
                        x: ref __self_0_0,
                        y: ref __self_0_1,
                        z: ref __self_0_2,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Uniform3f");
                        let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("y", &&(*__self_0_1));
                        let _ = debug_trait_builder.field("z", &&(*__self_0_2));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Uniform3f {
            pub fn new(x: f32, y: f32, z: f32) -> Self {
                Self { x, y, z }
            }
        }
        impl Uniform for Uniform3f {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform3f(Some(location), self.x, self.y, self.z);
            }
        }
        pub struct Uniform2f {
            x: f32,
            y: f32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Uniform2f {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Uniform2f {
                        x: ref __self_0_0,
                        y: ref __self_0_1,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Uniform2f");
                        let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("y", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Uniform2f {
            pub fn new(x: f32, y: f32) -> Self {
                Self { x, y }
            }
        }
        impl Uniform for Uniform2f {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform2f(Some(location), self.x, self.y);
            }
        }
        pub struct Uniform1f {
            x: f32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Uniform1f {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Uniform1f { x: ref __self_0_0 } => {
                        let mut debug_trait_builder = f.debug_struct("Uniform1f");
                        let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Uniform1f {
            pub fn new(x: f32) -> Self {
                Self { x }
            }
        }
        impl Uniform for Uniform1f {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform1f(Some(location), self.x);
            }
        }
        pub struct Uniform1i {
            x: i32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Uniform1i {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Uniform1i { x: ref __self_0_0 } => {
                        let mut debug_trait_builder = f.debug_struct("Uniform1i");
                        let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Uniform1i {
            pub fn new(x: i32) -> Self {
                Self { x }
            }
            pub fn new_bool(x: bool) -> Self {
                if x {
                    Self { x: 1 }
                } else {
                    Self { x: 0 }
                }
            }
        }
        impl Uniform for Uniform1i {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform1i(Some(location), self.x);
            }
        }
        pub struct UniformMat3fv<A: Deref<Target = [f32]>> {
            data: A,
            transpose: bool,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<A: ::core::fmt::Debug + Deref<Target = [f32]>> ::core::fmt::Debug for UniformMat3fv<A> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    UniformMat3fv {
                        data: ref __self_0_0,
                        transpose: ref __self_0_1,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("UniformMat3fv");
                        let _ = debug_trait_builder.field("data", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("transpose", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl<A: Deref<Target = [f32]>> UniformMat3fv<A> {
            pub fn new(data: A) -> Self {
                Self {
                    data,
                    transpose: false,
                }
            }
            pub fn new_transpose(data: A, transpose: bool) -> Self {
                Self { data, transpose }
            }
        }
        impl<A: Deref<Target = [f32]> + Debug> Uniform for UniformMat3fv<A> {
            fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
                gl.uniform_matrix3fv_with_f32_array(
                    Some(location),
                    self.transpose,
                    self.data.deref(),
                );
            }
        }
    }
    use shader::Shader;
    mod buffer {
        pub use buffer::{Buffer, BufferTrait, IndexBuffer, VertexBuffer};
        mod buffer {
            use std::ops::Deref;
            use web_sys::WebGlRenderingContext as GL;
            use web_sys::*;
            pub type VertexBuffer = Buffer<f32, Vec<f32>>;
            pub type IndexBuffer = Buffer<u16, Vec<u16>>;
            pub trait BufferTrait {
                fn bind(&self, gl: &GL);
                fn get_count(&self) -> usize;
            }
            pub struct Buffer<T, A: Deref<Target = [T]>> {
                buffer: WebGlBuffer,
                data: Option<A>,
                count: usize,
                target: u32,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<T: ::core::fmt::Debug, A: ::core::fmt::Debug + Deref<Target = [T]>>
                ::core::fmt::Debug for Buffer<T, A>
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Buffer {
                            buffer: ref __self_0_0,
                            data: ref __self_0_1,
                            count: ref __self_0_2,
                            target: ref __self_0_3,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("Buffer");
                            let _ = debug_trait_builder.field("buffer", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("data", &&(*__self_0_1));
                            let _ = debug_trait_builder.field("count", &&(*__self_0_2));
                            let _ = debug_trait_builder.field("target", &&(*__self_0_3));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl VertexBuffer {
                #[inline]
                pub fn vertex_buffer<D: Into<Option<Vec<f32>>>>(gl: &GL, data: D) -> Option<Self> {
                    Buffer::<f32, Vec<f32>>::new(gl, data.into(), GL::ARRAY_BUFFER)
                }
            }
            impl IndexBuffer {
                #[inline]
                pub fn index_buffer<D: Into<Option<Vec<u16>>>>(gl: &GL, data: D) -> Option<Self> {
                    Buffer::<u16, Vec<u16>>::new(gl, data.into(), GL::ELEMENT_ARRAY_BUFFER)
                }
            }
            impl<A: Deref<Target = [f32]>> Buffer<f32, A> {
                pub fn new(gl: &GL, data: Option<A>, target: u32) -> Option<Self> {
                    let buffer = gl.create_buffer()?;
                    let mut this = Self {
                        count: 0,
                        data: None,
                        target,
                        buffer,
                    };
                    if let Some(data) = data {
                        this.update_data(gl, data);
                    }
                    Some(this)
                }
                pub fn update_data(&mut self, gl: &GL, data: A) {
                    self.count = data.len();
                    self.data = Some(data);
                    let verts = unsafe { js_sys::Float32Array::view(self.data.as_ref().unwrap()) };
                    gl.bind_buffer(self.target, Some(&self.buffer));
                    gl.buffer_data_with_array_buffer_view(self.target, &verts, GL::STATIC_DRAW);
                }
            }
            impl<A: Deref<Target = [i32]>> Buffer<i32, A> {
                pub fn new(gl: &GL, data: Option<A>, target: u32) -> Option<Self> {
                    let buffer = gl.create_buffer()?;
                    let mut this = Self {
                        count: 0,
                        data: None,
                        target,
                        buffer,
                    };
                    if let Some(data) = data {
                        this.update_data(gl, data);
                    }
                    Some(this)
                }
                pub fn update_data(&mut self, gl: &GL, data: A) {
                    self.count = data.len();
                    self.data = Some(data);
                    let verts = unsafe { js_sys::Int32Array::view(self.data.as_ref().unwrap()) };
                    gl.bind_buffer(self.target, Some(&self.buffer));
                    gl.buffer_data_with_array_buffer_view(self.target, &verts, GL::STATIC_DRAW);
                }
            }
            impl<A: Deref<Target = [u16]>> Buffer<u16, A> {
                pub fn new(gl: &GL, data: Option<A>, target: u32) -> Option<Self> {
                    let buffer = gl.create_buffer()?;
                    let mut this = Self {
                        count: 0,
                        data: None,
                        target,
                        buffer,
                    };
                    if let Some(data) = data {
                        this.update_data(gl, data);
                    }
                    Some(this)
                }
                pub fn update_data(&mut self, gl: &GL, data: A) {
                    self.count = data.len();
                    self.data = Some(data);
                    let verts = unsafe { js_sys::Uint16Array::view(self.data.as_ref().unwrap()) };
                    gl.bind_buffer(self.target, Some(&self.buffer));
                    gl.buffer_data_with_array_buffer_view(self.target, &verts, GL::STATIC_DRAW);
                }
            }
            impl<T, A: Deref<Target = [T]>> BufferTrait for Buffer<T, A> {
                fn bind(&self, gl: &GL) {
                    gl.bind_buffer(self.target, Some(&self.buffer));
                }
                fn get_count(&self) -> usize {
                    self.count
                }
            }
        }
        pub use vertex::{VertexArray, VertexBufferLayout};
        mod vertex {
            use super::super::Shader;
            use web_sys::WebGlRenderingContext as GL;
            use super::{BufferTrait, VertexBuffer};
            struct VertexBufferElement {
                type_: u32,
                amount: i32,
                type_size: i32,
                index: String,
                normalized: bool,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for VertexBufferElement {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        VertexBufferElement {
                            type_: ref __self_0_0,
                            amount: ref __self_0_1,
                            type_size: ref __self_0_2,
                            index: ref __self_0_3,
                            normalized: ref __self_0_4,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("VertexBufferElement");
                            let _ = debug_trait_builder.field("type_", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("amount", &&(*__self_0_1));
                            let _ = debug_trait_builder.field("type_size", &&(*__self_0_2));
                            let _ = debug_trait_builder.field("index", &&(*__self_0_3));
                            let _ = debug_trait_builder.field("normalized", &&(*__self_0_4));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl VertexBufferElement {
                fn new(
                    type_: u32,
                    amount: i32,
                    type_size: i32,
                    index: String,
                    normalized: bool,
                ) -> Self {
                    Self {
                        type_,
                        amount,
                        type_size,
                        index,
                        normalized,
                    }
                }
            }
            pub struct VertexBufferLayout {
                elements: Vec<VertexBufferElement>,
                stride: i32,
                offset: i32,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for VertexBufferLayout {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        VertexBufferLayout {
                            elements: ref __self_0_0,
                            stride: ref __self_0_1,
                            offset: ref __self_0_2,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("VertexBufferLayout");
                            let _ = debug_trait_builder.field("elements", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("stride", &&(*__self_0_1));
                            let _ = debug_trait_builder.field("offset", &&(*__self_0_2));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl VertexBufferLayout {
                pub fn new() -> Self {
                    Self {
                        elements: Vec::new(),
                        stride: 0,
                        offset: 0,
                    }
                }
                pub fn offset(&mut self, offset: i32) {
                    self.offset = offset;
                }
                pub fn push<S: Into<String>>(
                    &mut self,
                    type_: u32,
                    amount: i32,
                    type_size: i32,
                    index: S,
                    normalized: bool,
                ) {
                    self.elements.push(VertexBufferElement::new(
                        type_,
                        amount,
                        type_size,
                        index.into(),
                        normalized,
                    ));
                    self.stride += amount * type_size;
                }
            }
            pub struct VertexArray {
                buffers: Vec<VertexBuffer>,
                layouts: Vec<VertexBufferLayout>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for VertexArray {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        VertexArray {
                            buffers: ref __self_0_0,
                            layouts: ref __self_0_1,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("VertexArray");
                            let _ = debug_trait_builder.field("buffers", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("layouts", &&(*__self_0_1));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl VertexArray {
                #[inline]
                pub fn new() -> Self {
                    Self {
                        buffers: Vec::new(),
                        layouts: Vec::new(),
                    }
                }
                pub fn add_buffer(&mut self, vb: VertexBuffer, layout: VertexBufferLayout) {
                    self.buffers.push(vb);
                    self.layouts.push(layout);
                }
                pub fn update_buffer(&mut self, gl: &GL, index: usize, data: Vec<f32>) {
                    self.buffers.get_mut(index).map(|b| b.update_data(gl, data));
                }
                pub fn bind(&self, gl: &GL, shader: &mut Shader) {
                    shader.bind(gl);
                    for (buffer, layout) in self.buffers.iter().zip(self.layouts.iter()) {
                        buffer.bind(gl);
                        let mut offset = layout.offset;
                        for element in &layout.elements {
                            let location = shader
                                .get_attrib_location(gl, &element.index)
                                .expect("Location error");
                            if location >= 0 {
                                let idx = location as u32;
                                gl.vertex_attrib_pointer_with_i32(
                                    idx,
                                    element.amount,
                                    element.type_,
                                    element.normalized,
                                    layout.stride,
                                    offset,
                                );
                                gl.enable_vertex_attrib_array(idx);
                            } else {
                            }
                            offset += element.amount * element.type_size;
                        }
                    }
                }
                pub fn unbind(&self, gl: &GL, shader: &mut Shader) {
                    for layout in &self.layouts {
                        for element in &layout.elements {
                            let location = shader.get_attrib_location(gl, &element.index);
                            if let Some(location) = location {
                                if location >= 0 {
                                    gl.disable_vertex_attrib_array(location as u32);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mod renderer {
        use super::{
            buffer::{BufferTrait, IndexBuffer, VertexArray},
            shader::Uniform,
            Shader,
        };
        use std::collections::{BTreeSet, HashMap};
        use web_sys::WebGlRenderingContext as GL;
        static SHOW_UNIFORMS: bool = false;
        pub trait Renderable {
            fn get_uniforms<'a>(&'a mut self) -> &'a mut Option<HashMap<String, Box<dyn Uniform>>>;
            fn render(&mut self, gl: &GL);
            fn update_vao(&mut self, gl: &GL, index: usize, new_data: Vec<f32>);
            fn update_ib(&mut self, gl: &GL, new_indices: Vec<u16>);
        }
        pub struct DefaultRenderable {
            ibo: IndexBuffer,
            vao: VertexArray,
            shader: Shader,
            uniforms: Option<HashMap<String, Box<dyn Uniform>>>,
        }
        impl DefaultRenderable {
            pub fn new(
                ibo: IndexBuffer,
                vao: VertexArray,
                shader: Shader,
                uniforms: Option<HashMap<String, Box<dyn Uniform>>>,
            ) -> Self {
                Self {
                    ibo,
                    vao,
                    shader,
                    uniforms,
                }
            }
        }
        impl Renderable for DefaultRenderable {
            #[inline]
            fn get_uniforms<'a>(&'a mut self) -> &'a mut Option<HashMap<String, Box<dyn Uniform>>> {
                &mut self.uniforms
            }
            fn update_vao(&mut self, gl: &GL, index: usize, new_data: Vec<f32>) {
                self.vao.update_buffer(gl, index, new_data);
            }
            fn update_ib(&mut self, gl: &GL, new_indices: Vec<u16>) {
                self.ibo.update_data(gl, new_indices);
            }
            fn render(&mut self, gl: &GL) {
                if let Some(uniforms) = &self.uniforms {
                    for (name, uniform) in uniforms.iter() {
                        if SHOW_UNIFORMS {}
                        if self.shader.uniform(gl, &name, &uniform).is_none() {}
                    }
                }
                self.vao.bind(gl, &mut self.shader);
                self.ibo.bind(gl);
                gl.draw_elements_with_i32(
                    GL::TRIANGLES,
                    self.ibo.get_count() as i32,
                    GL::UNSIGNED_SHORT,
                    0,
                );
            }
        }
        pub struct Renderer {
            layers: HashMap<usize, Vec<(Box<dyn Renderable>, bool)>>,
            sorted_layers: BTreeSet<usize>,
        }
        impl Renderer {
            #[inline]
            pub fn new() -> Self {
                Self {
                    layers: HashMap::new(),
                    sorted_layers: BTreeSet::new(),
                }
            }
            pub fn update_uniforms<F>(&mut self, index: usize, layer: usize, apply: F)
            where
                F: FnOnce(&mut Option<HashMap<String, Box<dyn Uniform>>>),
            {
                if let Some(layer) = self.layers.get_mut(&layer) {
                    if let Some(renderable) = layer.get_mut(index) {
                        apply(renderable.0.get_uniforms());
                    }
                }
            }
            pub fn disable_renderable(&mut self, index: usize, layer: usize) {
                if let Some(layer) = self.layers.get_mut(&layer) {
                    if let Some(renderable) = layer.get_mut(index) {
                        renderable.1 = false;
                    }
                }
            }
            pub fn enable_renderable(&mut self, index: usize, layer: usize) {
                if let Some(layer) = self.layers.get_mut(&layer) {
                    if let Some(renderable) = layer.get_mut(index) {
                        renderable.1 = true;
                    }
                }
            }
            pub fn add_renderable(&mut self, item: Box<dyn Renderable>, layer: usize) -> usize {
                if self.sorted_layers.insert(layer) {
                    self.layers.insert(layer, Vec::new());
                }
                let layer = self.layers.get_mut(&layer).unwrap();
                layer.push((item, true));
                layer.len() - 1
            }
            #[inline]
            pub fn add_to_draw<U: Into<Option<HashMap<String, Box<dyn Uniform>>>>>(
                &mut self,
                ibo: IndexBuffer,
                vao: VertexArray,
                shader: Shader,
                uniforms: U,
                layer: usize,
            ) -> usize {
                self.add_renderable(
                    Box::new(DefaultRenderable::new(ibo, vao, shader, uniforms.into())),
                    layer,
                )
            }
            pub fn render(&mut self, gl: &GL) {
                for layer_idx in self.sorted_layers.iter() {
                    if let Some(layer) = self.layers.get_mut(layer_idx) {
                        for (renderable, enabled) in layer.iter_mut() {
                            if *enabled {
                                renderable.render(gl);
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use webgl::*;
mod delaunay {
    mod delaunay {
        use super::{Edge, EdgeType, Triangle, TriangleType, VertexType};
        pub struct Delaunay {
            triangles: Vec<TriangleType>,
            edges: Vec<EdgeType>,
            vertices: Vec<VertexType>,
        }
        impl Delaunay {
            pub fn new() -> Self {
                Self {
                    triangles: Vec::new(),
                    edges: Vec::new(),
                    vertices: Vec::new(),
                }
            }
            #[inline]
            pub fn triangles(&self) -> &Vec<TriangleType> {
                &self.triangles
            }
            #[inline]
            pub fn edges(&self) -> &Vec<EdgeType> {
                &self.edges
            }
            #[inline]
            pub fn vertices(&self) -> &Vec<VertexType> {
                &self.vertices
            }
            pub fn triangulate(vertices: &Vec<VertexType>) -> Self {
                let mut vertices = vertices.clone();
                let mut triangles = Vec::new();
                let mut edges = Vec::new();
                let (min_x, min_y, max_x, max_y) = {
                    let (mut min_x, mut min_y) = vertices[0];
                    let (mut max_x, mut max_y) = (min_x, min_y);
                    for &(vx, vy) in &vertices {
                        if vx < min_x {
                            min_x = vx;
                        }
                        if vy < min_y {
                            min_y = vy;
                        }
                        if vx > max_x {
                            max_x = vx;
                        }
                        if vy > max_y {
                            max_y = vy;
                        }
                    }
                    (min_x, min_y, max_x, max_y)
                };
                let dx = max_x - min_x;
                let dy = max_y - min_y;
                let delta_max = dx.max(dy);
                let mid_x = (min_x + max_x) / 2.0;
                let mid_y = (min_y + max_y) / 2.0;
                let p1 = (mid_x - 20.0 * delta_max, mid_y - 20.0 * delta_max);
                let p2 = (mid_x, mid_y + delta_max);
                let p3 = (mid_x + 20.0 * delta_max, mid_y - 20.0 * delta_max);
                vertices.push(p1);
                vertices.push(p2);
                vertices.push(p3);
                triangles.push(Triangle::new(
                    vertices.len() - 3,
                    vertices.len() - 2,
                    vertices.len() - 1,
                    &vertices,
                ));
                for p in 0..vertices.len() - 3 {
                    let mut polygon = Vec::new();
                    for t in triangles.iter_mut() {
                        if t.circum_circle_contains(p, &vertices) {
                            t.is_bad = true;
                            polygon.push(Edge::new(t.a, t.b));
                            polygon.push(Edge::new(t.b, t.c));
                            polygon.push(Edge::new(t.c, t.a));
                        }
                    }
                    triangles = triangles.into_iter().filter(|t| !t.is_bad).collect();
                    for i in 0..polygon.len() {
                        for j in i + 1..polygon.len() {
                            if polygon[i] == polygon[j] {
                                polygon[i].is_bad = true;
                                polygon[j].is_bad = true;
                            }
                        }
                    }
                    for Edge { v, w, is_bad } in polygon {
                        if !is_bad {
                            triangles.push(Triangle::new(v, w, p.clone(), &vertices));
                        }
                    }
                }
                triangles = triangles
                    .into_iter()
                    .filter(|t| {
                        !(t.contains_vertex(vertices.len() - 3)
                            || t.contains_vertex(vertices.len() - 2)
                            || t.contains_vertex(vertices.len() - 1))
                    })
                    .collect();
                for t in &triangles {
                    edges.push(Edge::new(t.a, t.b));
                    edges.push(Edge::new(t.b, t.c));
                    edges.push(Edge::new(t.c, t.a));
                }
                vertices.pop();
                vertices.pop();
                vertices.pop();
                Self {
                    vertices,
                    edges,
                    triangles,
                }
            }
        }
    }
    pub use delaunay::*;
    pub type Type = f32;
    pub type VertexType = (Type, Type);
    pub type EdgeType = Edge;
    pub type TriangleType = Triangle;
    pub fn dist2((tx, ty): &VertexType, (ox, oy): &VertexType) -> Type {
        let dx = tx - ox;
        let dy = ty - oy;
        dx * dx + dy * dy
    }
    pub fn norm2((x, y): &VertexType) -> Type {
        x * x + y * y
    }
    pub fn dist((tx, ty): &VertexType, (ox, oy): &VertexType) -> f32 {
        (tx - ox).hypot(ty - oy)
    }
    pub struct Edge {
        pub v: usize,
        pub w: usize,
        pub is_bad: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Edge {
        #[inline]
        fn clone(&self) -> Edge {
            match *self {
                Edge {
                    v: ref __self_0_0,
                    w: ref __self_0_1,
                    is_bad: ref __self_0_2,
                } => Edge {
                    v: ::core::clone::Clone::clone(&(*__self_0_0)),
                    w: ::core::clone::Clone::clone(&(*__self_0_1)),
                    is_bad: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Edge {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Edge {
                    v: ref __self_0_0,
                    w: ref __self_0_1,
                    is_bad: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Edge");
                    let _ = debug_trait_builder.field("v", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("w", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("is_bad", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl Edge {
        pub fn new(v: usize, w: usize) -> Self {
            Self {
                v,
                w,
                is_bad: false,
            }
        }
    }
    impl PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            (self.v == other.v || self.v == other.w) && (self.w == other.v || self.w == other.v)
        }
    }
    pub struct Triangle {
        pub a: usize,
        pub b: usize,
        pub c: usize,
        pub is_bad: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Triangle {
        #[inline]
        fn clone(&self) -> Triangle {
            match *self {
                Triangle {
                    a: ref __self_0_0,
                    b: ref __self_0_1,
                    c: ref __self_0_2,
                    is_bad: ref __self_0_3,
                } => Triangle {
                    a: ::core::clone::Clone::clone(&(*__self_0_0)),
                    b: ::core::clone::Clone::clone(&(*__self_0_1)),
                    c: ::core::clone::Clone::clone(&(*__self_0_2)),
                    is_bad: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Triangle {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Triangle {
                    a: ref __self_0_0,
                    b: ref __self_0_1,
                    c: ref __self_0_2,
                    is_bad: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Triangle");
                    let _ = debug_trait_builder.field("a", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("b", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("c", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("is_bad", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl Triangle {
        pub fn new(a: usize, b: usize, c: usize, vs: &Vec<VertexType>) -> Self {
            let (ax, ay) = vs[a];
            let (bx, by) = vs[b];
            let (cx, cy) = vs[c];
            if (bx - ax) * (cy - ay) - (by - ay) * (cx - ax) < 0.0 {
                Self {
                    a: b,
                    b: a,
                    c,
                    is_bad: false,
                }
            } else {
                Self {
                    a,
                    b,
                    c,
                    is_bad: false,
                }
            }
        }
        pub fn contains_vertex(&self, v: usize) -> bool {
            self.a == v || self.b == v || self.c == v
        }
        pub fn circum_circle_contains(&self, v: usize, vs: &Vec<VertexType>) -> bool {
            let a = vs[self.a];
            let b = vs[self.b];
            let c = vs[self.c];
            let ab = norm2(&a);
            let cd = norm2(&b);
            let ef = norm2(&c);
            let (ax, ay) = a;
            let (bx, by) = b;
            let (cx, cy) = c;
            let circum_x = (ab * (cy - by) + cd * (ay - cy) + ef * (by - ay))
                / (ax * (cy - by) + bx * (ay - cy) + cx * (by - ay));
            let circum_y = (ab * (cx - bx) + cd * (ax - cx) + ef * (bx - ax))
                / (ay * (cx - bx) + by * (ax - cx) + cy * (bx - ax));
            let circum = (circum_x / 2.0, circum_y / 2.0);
            let circum_radius = dist2(&a, &circum);
            let dist = dist2(&vs[v], &circum);
            dist <= circum_radius
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn det(
        a: Type,
        b: Type,
        c: Type,
        d: Type,
        e: Type,
        f: Type,
        g: Type,
        h: Type,
        i: Type,
    ) -> Type {
        a * e * i + b * f * g + c * d * h - c * e * g - b * d * i - a * f * h
    }
    impl PartialEq for Triangle {
        fn eq(&self, other: &Self) -> bool {
            (self.a == other.a || self.a == other.b || self.a == other.c)
                && (self.b == other.a || self.b == other.b || self.b == other.c)
                && (self.c == other.a || self.c == other.b || self.c == other.c)
        }
    }
}
#[allow(bad_style)]
///
#[allow(clippy::all)]
pub fn log(s: &str) {
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    unsafe fn __wbg_log_340625fac3035954(
        s: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    ) -> () {
        drop(s);
        {
            ::std::rt::begin_panic(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets",
            )
        };
    }
    unsafe {
        let _ret = {
            let s = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(s);
            __wbg_log_340625fac3035954(s)
        };
        ()
    }
}
