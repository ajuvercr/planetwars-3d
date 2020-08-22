use super::{
    buffer::{BufferTrait, IndexBuffer, VertexArray},
    uniform::Uniform,
    Shader,
};
use std::{sync::mpsc, collections::{BTreeSet, HashMap}};
use web_sys::WebGlRenderingContext as GL;
use crate::uniform::{UniformsHandle, UniformUpdate};

static SHOW_UNIFORMS: bool = false;

pub trait Renderable {
    fn render(&mut self, gl: &GL);
    fn update(&mut self, gl: &GL) -> Option<()>;
}


pub struct DefaultRenderable {
    ibo: IndexBuffer,
    vao: VertexArray,
    shader: Shader,
    uniforms: HashMap<String, Box<dyn Uniform>>,

    tx: mpsc::Sender<UniformUpdate>,
    rx: mpsc::Receiver<UniformUpdate>,
}

impl DefaultRenderable {
    pub fn new<U: Into<Option<HashMap<String, Box<dyn Uniform>>>>>(
        ibo: IndexBuffer,
        vao: VertexArray,
        shader: Shader,
        uniforms: U,
    ) -> Self {

        let (tx, rx) = mpsc::channel();

        Self {
            ibo,
            vao,
            shader,
            uniforms: uniforms.into().unwrap_or(HashMap::new()),
            tx, rx,
        }
    }

    pub fn handle(&self) -> UniformsHandle {
        UniformsHandle::new(self.tx.clone())
    }
}

impl Renderable for DefaultRenderable {
    fn update(&mut self, gl: &GL) -> Option<()> {
        loop {
            match self.rx.try_recv() {
                Ok(UniformUpdate::Batch(context)) => {
                    self.uniforms.extend(context.into_iter());
                },
                Ok(UniformUpdate::Single(name, uniform)) => {
                    self.uniforms.insert(name, uniform);
                },
                Err(mpsc::TryRecvError::Disconnected) => return None,
                Err(mpsc::TryRecvError::Empty) => break,
            }
        }
        self.ibo.flush(gl)?;
        self.vao.update(gl)?;
        Some(())
    }
    fn render(&mut self, gl: &GL) {
        for (name, uniform) in self.uniforms.iter() {
            if SHOW_UNIFORMS {
                console_log!("Setting uniform {} {:?}", name, uniform);
            }

            if self.shader.uniform(gl, &name, &uniform).is_none() {
                console_log!("Failed etting uniform {} {:?}", name, uniform);
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

    pub fn add_renderable<R: Renderable + 'static>(&mut self, item: R, layer: usize) -> usize {
        if self.sorted_layers.insert(layer) {
            self.layers.insert(layer, Vec::new());
        }

        let layer = self.layers.get_mut(&layer).unwrap();
        layer.push((Box::new(item), true));
        layer.len() - 1
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
