use gl;
use std;
use std::ffi::CString;
pub struct GLStruct {}

#[derive(Debug)]
pub struct GlBuffer {
    id: gl::types::GLuint,
}

#[derive(Debug)]
pub struct GlShader {
    id: gl::types::GLuint,
}

#[derive(Debug)]
pub struct GlProgram {
    id: gl::types::GLuint,
}

#[derive(Debug)]
pub struct GlVao {
    id: gl::types::GLuint,
}

#[derive(Debug, Clone)]
pub struct GlUniformLocation {
    id: gl::types::GLint,
}

pub use GLStruct as GL;

impl GLStruct {
    pub const ARRAY_BUFFER: u32 = gl::ARRAY_BUFFER;
    pub const ELEMENT_ARRAY_BUFFER: u32 = gl::ELEMENT_ARRAY_BUFFER;
    pub const STATIC_DRAW: u32 = gl::STATIC_DRAW;
    pub const TRIANGLES: u32 = gl::TRIANGLES;
    pub const FLOAT: u32 = gl::FLOAT;
    pub const UNSIGNED_SHORT: u32 = gl::UNSIGNED_SHORT;

    pub const COMPILE_STATUS: u32 = gl::COMPILE_STATUS;
    pub const LINK_STATUS: u32 = gl::LINK_STATUS;
    pub const VERTEX_SHADER: u32 = gl::VERTEX_SHADER;
    pub const FRAGMENT_SHADER: u32 = gl::FRAGMENT_SHADER;

    pub fn new() -> Self {
        GLStruct {}
    }

    pub fn create_buffer(&self) -> Option<GlBuffer> {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Some(GlBuffer { id })
    }

    pub fn bind_buffer(&self, target: u32, buffer: Option<&GlBuffer>) {
        let id = buffer.map(|x| x.id).unwrap_or(0);
        unsafe {
            gl::BindBuffer(target, id);
        }
    }

    pub fn set_f32_buffer_data(&self, target: u32, data: &[f32], type_: u32) {
        unsafe {
            gl::BufferData(
                target,                                                             // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                type_,                                     // usage
            );
        }
    }

    pub fn set_u16_buffer_data(&self, target: u32, data: &[u16], type_: u32) {
        unsafe {
            gl::BufferData(
                target,                                                             // target
                (data.len() * std::mem::size_of::<u16>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                type_,                                     // usage
            );
        }
    }

    pub fn set_i32_buffer_data(&self, target: u32, data: &[i32], type_: u32) {
        unsafe {
            gl::BufferData(
                target,                                                             // target
                (data.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                type_,                                     // usage
            );
        }
    }

    pub fn create_vertex_array(&self) -> Option<GlVao> {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Some(GlVao { id })
    }

    pub fn bind_vertex_array(&self, vao: Option<&GlVao>) {
        let id = vao.map(|x| x.id).unwrap_or(0);
        unsafe {
            gl::BindVertexArray(id);
        }
    }

    pub fn vertex_attrib_pointer_with_i32(
        &self,
        id: u32,
        amount: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        let normalized = if normalized { gl::TRUE } else { gl::FALSE };
        let offset: *const std::ffi::c_void = unsafe {std::mem::transmute(offset as i64) };
        unsafe {
            gl::VertexAttribPointer(
                id,
                amount,
                type_,
                normalized,
                stride, // maybe
                offset,
            );
        }
    }

    pub fn enable_vertex_attrib_array(&self, id: u32) {
        unsafe {
            gl::EnableVertexAttribArray(id);
        }
    }

    pub fn disable_vertex_attrib_array(&self, id: u32) {
        unsafe {
            gl::DisableVertexAttribArray(id);
        }
    }

    pub fn draw_elements_with_i32(&self, method: u32, count: i32, type_: u32, offset: i32) {
        let offset: *const std::ffi::c_void = unsafe {std::mem::transmute(offset as i64) };
        unsafe {
            gl::DrawElements(method, count, type_, offset);
        }
    }

    pub fn draw_arrays(&self, method: u32, offset: i32, count: i32) {
        unsafe {
            gl::DrawArrays(method, offset, count);
        }
    }

    pub fn create_shader(&self, shader_type: u32) -> Option<GlShader> {
        let id = unsafe { gl::CreateShader(shader_type) };
        Some(GlShader { id })
    }

    pub fn shader_source(&self, shader: &GlShader, source: &str) {
        // TODO set version
        let source = CString::new(source).unwrap();
        unsafe {
            gl::ShaderSource(shader.id, 1, &(&source).as_ptr(), std::ptr::null());
        }
    }

    pub fn compile_shader(&self, shader: &GlShader) {
        unsafe {
            gl::CompileShader(shader.id);
        }
    }

    pub fn get_shader_parameter(&self, shader: &GlShader, type_: u32) -> Option<bool> {
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(shader.id, type_, &mut success);
        }
        Some(success == 1)
    }

    pub fn get_shader_info_log(&self, shader: &GlShader) -> Option<String> {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                shader.id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        Some(error.to_string_lossy().into_owned())
    }

    pub fn delete_shader(&self, shader: Option<&GlShader>) {
        if let Some(shader) = shader {
            unsafe {
                gl::DeleteShader(shader.id);
            }
        }
    }

    pub fn create_program(&self) -> Option<GlProgram> {
        let id = unsafe { gl::CreateProgram() };
        Some(GlProgram { id })
    }

    pub fn attach_shader(&self, program: &GlProgram, shader: &GlShader) {
        unsafe {
            gl::AttachShader(program.id, shader.id);
        }
    }

    pub fn link_program(&self, program: &GlProgram) {
        unsafe {
            gl::LinkProgram(program.id);
        }
    }

    pub fn get_program_parameter(&self, program: &GlProgram, type_: u32) -> Option<bool> {
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program.id, type_, &mut success);
        }
        Some(success == 1)
    }

    pub fn get_program_info_log(&self, program: &GlProgram) -> Option<String> {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetProgramInfoLog(
                program.id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        Some(error.to_string_lossy().into_owned())
    }

    pub fn delete_program(&self, program: Option<&GlProgram>) {
        if let Some(program) = program {
            unsafe {
                gl::DeleteProgram(program.id);
            }
        }
    }

    pub fn use_program(&self, program: Option<&GlProgram>) {
        if let Some(program) = program {
            unsafe {
                gl::UseProgram(program.id);
            }
        }
    }

    pub fn get_uniform_location(
        &self,
        shader: &GlProgram,
        name: &str,
    ) -> Option<GlUniformLocation> {
        let source = CString::new(name).unwrap();
        let id = unsafe { gl::GetUniformLocation(shader.id, (&source).as_ptr()) };
        Some(GlUniformLocation { id })
    }

    pub fn get_attrib_location(&self, shader: &GlProgram, name: &str) -> i32 {
        let source = CString::new(name).unwrap();
        let id = unsafe { gl::GetAttribLocation(shader.id, (&source).as_ptr()) };
        id
    }

    pub fn uniform2fv_with_f32_array(&self, location: Option<&GlUniformLocation>, data: &[f32]) {
        if let Some(location) = location {
            unsafe {
                gl::Uniform2fv(
                    location.id,
                    data.len() as i32 / 2,
                    data.as_ptr() as *const gl::types::GLfloat,
                )
            }
        }
    }

    pub fn uniform3fv_with_f32_array(&self, location: Option<&GlUniformLocation>, data: &[f32]) {
        if let Some(location) = location {
            unsafe {
                gl::Uniform3fv(
                    location.id,
                    data.len() as i32 / 3,
                    data.as_ptr() as *const gl::types::GLfloat,
                )
            }
        }
    }

    pub fn uniform1iv_with_i32_array(&self, location: Option<&GlUniformLocation>, data: &[i32]) {
        if let Some(location) = location {
            unsafe {
                gl::Uniform1iv(
                    location.id,
                    data.len() as i32 / 1,
                    data.as_ptr() as *const gl::types::GLint,
                )
            }
        }
    }

    pub fn uniform4f(&self, location: Option<&GlUniformLocation>, x: f32, y: f32, z: f32, w: f32) {
        if let Some(location) = location {
            unsafe { gl::Uniform4f(location.id, x, y, z, w) }
        }
    }

    pub fn uniform3f(&self, location: Option<&GlUniformLocation>, x: f32, y: f32, z: f32) {
        if let Some(location) = location {
            unsafe { gl::Uniform3f(location.id, x, y, z) }
        }
    }

    pub fn uniform2f(&self, location: Option<&GlUniformLocation>, x: f32, y: f32) {
        if let Some(location) = location {
            unsafe { gl::Uniform2f(location.id, x, y) }
        }
    }

    pub fn uniform1f(&self, location: Option<&GlUniformLocation>, x: f32) {
        if let Some(location) = location {
            unsafe { gl::Uniform1f(location.id, x) }
        }
    }

    pub fn uniform1i(&self, location: Option<&GlUniformLocation>, x: i32) {
        if let Some(location) = location {
            unsafe { gl::Uniform1i(location.id, x) }
        }
    }

    pub fn uniform_matrix3fv_with_f32_array(
        &self,
        location: Option<&GlUniformLocation>,
        transpose: bool,
        data: &[f32],
    ) {
        if let Some(location) = location {
            unsafe {
                gl::UniformMatrix3fv(
                    location.id,
                    data.len() as i32 / 9,
                    if transpose { gl::TRUE } else { gl::FALSE },
                    data.as_ptr() as *const gl::types::GLfloat,
                )
            }
        }
    }

    pub fn uniform_matrix4fv_with_f32_array(
        &self,
        location: Option<&GlUniformLocation>,
        transpose: bool,
        data: &[f32],
    ) {
        if let Some(location) = location {
            unsafe {
                gl::UniformMatrix4fv(
                    location.id,
                    data.len() as i32 / 16,
                    if transpose { gl::TRUE } else { gl::FALSE },
                    data.as_ptr() as *const gl::types::GLfloat,
                )
            }
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
