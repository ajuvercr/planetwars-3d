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
    fn update(&mut self, gl: &GL) -> Option<()>;
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
    fn update(&mut self, gl: &GL) -> Option<()> {
        self.ibo.flush(gl)?;
        self.vao.update(gl)?;
        Some(())
    }
    fn render(&mut self, gl: &GL) {
        if let Some(uniforms) = &self.uniforms {
            for (name, uniform) in uniforms.iter() {
                if SHOW_UNIFORMS {
                    // console_log!("Setting uniform {} {:?}", name, uniform);
                }

                if self.shader.uniform(gl, &name, &uniform).is_none() {
                    // console_log!("Failed etting uniform {} {:?}", name, uniform);
                }
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

    pub fn update(&mut self, gl: &GL) -> Option<()> {
        for layer_idx in self.sorted_layers.iter() {
            if let Some(layer) = self.layers.get_mut(layer_idx) {
                for (renderable, _enabled) in layer.iter_mut() {
                    renderable.update(gl)?;
                }
            }
        }
        Some(())
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
