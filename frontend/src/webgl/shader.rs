use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

use std::collections::HashMap;
use yew::services::ConsoleService as Console;

fn load_shader(gl: &GL, shader_source: &str, shader_type: u32) -> Option<web_sys::WebGlShader> {
    let shader = gl.create_shader(shader_type)?;
    gl.shader_source(&shader, shader_source);
    gl.compile_shader(&shader);

    let compiled = gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap();
    if !compiled {
        Console::error(&format!(
            "*** Error compiling shader '{:?}': {}",
            shader_source, gl.get_shader_info_log(&shader).unwrap()
        ));
        gl.delete_shader(Some(&shader)); // Wtf why this option?
        return None;
    }

    Some(shader)
}

fn create_program(gl: &GL, shaders: Vec<web_sys::WebGlShader>) -> Option<web_sys::WebGlProgram> {
    let program = gl.create_program()?;
    shaders
        .iter()
        .for_each(|shader| gl.attach_shader(&program, shader));

    gl.link_program(&program);

    let linked = gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap();
    if !linked {
        Console::error(&format!(
            "*** Error linking program '{:?}': {}",
            program, gl.get_program_info_log(&program).unwrap()
        ));
        gl.delete_program(Some(&program)); // Wtf why this option?
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
            frag = frag.replace(&format!("${}", key), &value);
            vert = vert.replace(&format!("${}", key), &value);
        }

        let shaders = vec![
            load_shader(gl, &vert, GL::VERTEX_SHADER)?,
            load_shader(gl, &frag, GL::FRAGMENT_SHADER)?,
        ];

        let program = create_program(gl, shaders)?;

        Some(Shader::new(program))
    }

    pub fn factory(frag_source: String, vert_source: String) -> ShaderFactory {
        ShaderFactory::new(frag_source, vert_source)
    }

    pub fn bind(&self, gl: &GL) {
        gl.use_program(Some(&self.shader));
    }

    pub fn get_uniform_location(&mut self, gl: &GL, name: &str) -> Option<WebGlUniformLocation> {
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

    pub fn uniform(&mut self, gl: &GL, name: &str, uniform: &Box<dyn Uniform>) -> Option<()> {
        self.bind(gl);
        let loc = self.get_uniform_location(gl, name)?;
        uniform.set_uniform(gl, &loc);
        Some(())
    }

    pub unsafe fn drop_ref(&self, gl: &GL) {
        gl.delete_program(Some(&self.shader));
    }

    // This can be just a ref too
    pub fn drop(self, gl: &GL) {
        gl.delete_program(Some(&self.shader));
    }
}

/************************************************************************/
/*             Start super ugly generic code, just uniforms             */
/************************************************************************/

use std::{fmt::Debug, ops::Deref};
pub trait Uniform: Debug {
    fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation);
}

#[derive(Debug)]
pub struct Uniform2fv<A: Deref<Target = [f32]>> {
    data: A,
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

#[derive(Debug)]
pub struct Uniform3fv<A: Deref<Target = [f32]>> {
    data: A,
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
#[derive(Debug)]

pub struct Uniformifv<A: Deref<Target = [i32]>> {
    data: A,
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

#[derive(Debug)]
pub struct Uniform4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
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

#[derive(Debug)]
pub struct Uniform3f {
    x: f32,
    y: f32,
    z: f32,
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

#[derive(Debug)]
pub struct Uniform2f {
    x: f32,
    y: f32,
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
#[derive(Debug)]
pub struct Uniform1f {
    x: f32,
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

#[derive(Debug)]
pub struct Uniform1i {
    x: i32,
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

#[derive(Debug)]
pub struct UniformMat3fv<A: Deref<Target = [f32]>> {
    data: A,
    transpose: bool,
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
        gl.uniform_matrix3fv_with_f32_array(Some(location), self.transpose, self.data.deref());
    }
}
