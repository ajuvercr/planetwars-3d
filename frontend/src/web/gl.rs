use std::ops::Deref;

pub use web_sys::WebGlBuffer as GlBuffer;
pub use web_sys::WebGlProgram as GlProgram;
pub use web_sys::WebGlShader as GlShader;
pub use web_sys::WebGlUniformLocation as GlUniformLocation;
pub use GLStruct as GL;

use web_sys::WebGlRenderingContext as WebGL;

pub struct GLStruct {
    pub gl: WebGL,
}

#[derive(Debug)]
pub struct GlVao {}

impl GLStruct {
    pub const ARRAY_BUFFER: u32 = WebGL::ARRAY_BUFFER;
    pub const ELEMENT_ARRAY_BUFFER: u32 = WebGL::ELEMENT_ARRAY_BUFFER;
    pub const STATIC_DRAW: u32 = WebGL::STATIC_DRAW;
    pub const TRIANGLES: u32 = WebGL::TRIANGLES;
    pub const FLOAT: u32 = WebGL::FLOAT;
    pub const UNSIGNED_SHORT: u32 = WebGL::UNSIGNED_SHORT;

    pub const COMPILE_STATUS: u32 = WebGL::COMPILE_STATUS;
    pub const LINK_STATUS: u32 = WebGL::LINK_STATUS;
    pub const VERTEX_SHADER: u32 = WebGL::VERTEX_SHADER;
    pub const FRAGMENT_SHADER: u32 = WebGL::FRAGMENT_SHADER;

    pub const COLOR_BUFFER_BIT: u32 = WebGL::COLOR_BUFFER_BIT;
    pub const DEPTH_BUFFER_BIT: u32 = WebGL::DEPTH_BUFFER_BIT;
    pub const CULL_FACE: u32 = WebGL::CULL_FACE;
    pub const DEPTH_TEST: u32 = WebGL::DEPTH_TEST;

    pub fn set_f32_buffer_data(&self, target: u32, data: &[f32], type_: u32) {
        let verts = unsafe { js_sys::Float32Array::view(data) };
        self.gl
            .buffer_data_with_array_buffer_view(target, &verts, type_);
    }

    pub fn set_u16_buffer_data(&self, target: u32, data: &[u16], type_: u32) {
        let verts = unsafe { js_sys::Uint16Array::view(data) };
        self.gl
            .buffer_data_with_array_buffer_view(target, &verts, type_);
    }

    pub fn set_i32_buffer_data(&self, target: u32, data: &[i32], type_: u32) {
        let verts = unsafe { js_sys::Int32Array::view(data) };
        self.gl
            .buffer_data_with_array_buffer_view(target, &verts, type_);
    }

    pub fn get_shader_parameter(&self, shader: &GlShader, type_: u32) -> Option<bool> {
        self.gl.get_shader_parameter(shader, type_).as_bool()
    }

    pub fn get_program_parameter(&self, program: &GlProgram, type_: u32) -> Option<bool> {
        self.gl.get_program_parameter(program, type_).as_bool()
    }

    pub fn create_vertex_array(&self) -> Option<GlVao> {
        Some(GlVao {})
    }

    pub fn bind_vertex_array(&self, _vao: Option<&GlVao>) {
        // Empty
    }
}

impl Deref for GLStruct {
    type Target = web_sys::WebGlRenderingContext;
    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}
