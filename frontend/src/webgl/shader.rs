use crate::gl::{GL, self};

use crate::uniform::Uniform;
use std::collections::HashMap;

fn load_shader(gl: &GL, shader_source: &str, shader_type: u32) -> Option<gl::GlShader> {
    let shader = gl.create_shader(shader_type)?;
    gl.shader_source(&shader, shader_source);
    gl.compile_shader(&shader);

    let compiled = gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .unwrap();
    if !compiled {
        console_log!(
            "*** Error compiling shader '{:?}': {}",
            shader_source,
            gl.get_shader_info_log(&shader).unwrap()
        );
        gl.delete_shader(Some(&shader)); // Wtf why this option?
        return None;
    }

    Some(shader)
}

fn create_program(gl: &GL, shaders: Vec<gl::GlShader>) -> Option<gl::GlProgram> {
    let program = gl.create_program()?;
    shaders
        .iter()
        .for_each(|shader| gl.attach_shader(&program, shader));

    gl.link_program(&program);

    let linked = gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .unwrap();
    if !linked {
        console_log!(
            "*** Error linking program '{:?}': {}",
            program,
            gl.get_program_info_log(&program).unwrap(),
        );
        gl.delete_program(Some(&program)); // Wtf why this option?
        return None;
    }

    Some(program)
}

#[derive(Clone)]
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
    shader: gl::GlProgram,
    uniform_cache: HashMap<String, gl::GlUniformLocation>,
    attrib_cache: HashMap<String, i32>,
}

impl Shader {
    fn new(shader: gl::GlProgram) -> Self {
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

    pub fn get_uniform_location(&mut self, gl: &GL, name: &str) -> Option<gl::GlUniformLocation> {
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
