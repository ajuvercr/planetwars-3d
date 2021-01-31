pub struct GLStruct {}

#[derive(Debug)]
pub struct GlBuffer {}

#[derive(Debug)]
pub struct GlShader {}

#[derive(Debug)]
pub struct GlProgram {}

#[derive(Debug, Clone)]
pub struct GlUniformLocation {}

pub use GLStruct as GL;

impl GLStruct {
    pub const ARRAY_BUFFER: u32 = 0;
    pub const ELEMENT_ARRAY_BUFFER: u32 = 0;
    pub const STATIC_DRAW: u32 = 0;
    pub const TRIANGLES: u32 = 0;
    pub const FLOAT: u32 = 0;
    pub const UNSIGNED_SHORT: u32 = 0;

    pub const COMPILE_STATUS: u32 = 0;
    pub const LINK_STATUS: u32 = 0;
    pub const VERTEX_SHADER: u32 = 0;
    pub const FRAGMENT_SHADER: u32 = 0;


    pub fn create_buffer(&self) -> Option<GlBuffer> {
        todo!()
    }

    pub fn bind_buffer(&self, target: u32, buffer: Option<&GlBuffer>) {
        todo!()
    }

    pub fn set_f32_buffer_data(&self, target: u32, data: &[f32], type_: u32) {
        todo!();
    }

    pub fn set_u16_buffer_data(&self, target: u32, data: &[u16], type_: u32) {
        todo!();
    }

    pub fn set_i32_buffer_data(&self, target: u32, data: &[i32], type_: u32) {
        todo!();
    }

    pub fn vertex_attrib_pointer_with_i32(&self, id: u32, amount: i32, type_: u32, normalized: bool, stride: i32, offset: i32) {
        todo!()
    }

    pub fn enable_vertex_attrib_array(&self, id: u32) {
        todo!();
    }

    pub fn disable_vertex_attrib_array(&self, id: u32) {
        todo!();
    }

    pub fn draw_elements_with_i32(&self, method: u32, count: i32, type_: u32, offset: i32){
        todo!();
    }

    pub fn draw_arrays(&self, method: u32, offset: i32, count: i32) {
        todo!();
    }

    pub fn create_shader(&self, shader_type: u32) -> Option<GlShader> {
        todo!();
    }

    pub fn shader_source(&self, shader: &GlShader, source: &str) {
        todo!();
    }

    pub fn compile_shader(&self, shader: &GlShader) {
        todo!();
    }

    pub fn get_shader_parameter(&self, shader: &GlShader, type_: u32) -> Option<bool> {
        todo!();
    }

    pub fn get_shader_info_log(&self, shader: &GlShader) -> Option<String> {
        todo!();
    }

    pub fn delete_shader(&self, shader: Option<&GlShader>) {
        todo!();
    }

    pub fn create_program(&self) -> Option<GlProgram> {
        todo!();
    }

    pub fn attach_shader(&self, program: &GlProgram, shader: &GlShader) {
        todo!();
    }

    pub fn link_program(&self, program: &GlProgram) {
        todo!();
    }

    pub fn get_program_parameter(&self, program: &GlProgram, type_: u32) -> Option<bool> {
        todo!();
    }

    pub fn get_program_info_log(&self, program: &GlProgram) -> Option<String> {
        todo!();
    }

    pub fn delete_program(&self, program: Option<&GlProgram>) {
        todo!();
    }

    pub fn use_program(&self, program: Option<&GlProgram>) {
        todo!();
    }

    pub fn get_uniform_location(&self, shader: &GlProgram, name: &str) -> Option<GlUniformLocation> {
        todo!();
    }

    pub fn get_attrib_location(&self, shader: &GlProgram, name: &str) -> i32 {
        todo!();
    }

    pub fn uniform2fv_with_f32_array(&self, location: Option<&GlUniformLocation>, data: &[f32]) {
        todo!();
    }

    pub fn uniform3fv_with_f32_array(&self, location: Option<&GlUniformLocation>, data: &[f32]) {
        todo!();
    }

    pub fn uniform1iv_with_i32_array(&self, location: Option<&GlUniformLocation>, data: &[i32]) {
        todo!();
    }

    pub fn uniform4f(&self, location: Option<&GlUniformLocation>, x: f32, y: f32, z:f32, w: f32) {
        todo!()
    }

    pub fn uniform3f(&self, location: Option<&GlUniformLocation>, x: f32, y: f32, z:f32) {
        todo!()
    }

    pub fn uniform2f(&self, location: Option<&GlUniformLocation>, x: f32, y: f32) {
        todo!()
    }

    pub fn uniform1f(&self, location: Option<&GlUniformLocation>, x: f32) {
        todo!()
    }

    pub fn uniform1i(&self, location: Option<&GlUniformLocation>, x: i32) {
        todo!()
    }

    pub fn uniform_matrix3fv_with_f32_array(&self, location: Option<&GlUniformLocation>, transpose: bool, data: &[f32]) {
        todo!()
    }

    pub fn uniform_matrix4fv_with_f32_array(&self, location: Option<&GlUniformLocation>, transpose: bool, data: &[f32]) {
        todo!()
    }
}
