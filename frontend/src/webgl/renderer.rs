use super::{
    buffer::{BufferTrait, IndexBuffer, VertexArray},
    uniform::Uniform,
    Shader,
};
use crate::uniform::{UniformUpdate, UniformsHandle};
use std::{
    collections::{BTreeSet, HashMap},
    sync::mpsc,
};
use web_sys::WebGlRenderingContext as GL;

static SHOW_UNIFORMS: bool = false;

pub trait Renderable {
    fn render(&mut self, gl: &GL);
    fn update(&mut self, gl: &GL) -> Option<()>;
    fn is_disabled(&self) -> bool {
        false
    }
}

pub trait BatchRenderableTrait: Renderable {
    fn draw(&mut self, gl: &GL);
    fn bind(&mut self, gl: &GL) -> Option<()>;
    fn shader(&mut self) -> &mut Shader;
}

pub struct DefaultRenderable {
    ibo: Option<IndexBuffer>,
    vao: VertexArray,
    shader: Shader,
    uniforms: HashMap<String, Box<dyn Uniform>>,

    disabled: bool,

    tx: mpsc::Sender<UniformUpdate>,
    rx: mpsc::Receiver<UniformUpdate>,
}

impl DefaultRenderable {
    pub fn new<I: Into<Option<IndexBuffer>>, U: Into<Option<HashMap<String, Box<dyn Uniform>>>>>(
        ibo: I,
        vao: VertexArray,
        shader: Shader,
        uniforms: U,
    ) -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            ibo: ibo.into(),
            vao,
            disabled: false,
            shader,
            uniforms: uniforms.into().unwrap_or(HashMap::new()),
            tx,
            rx,
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
                }
                Ok(UniformUpdate::Single(name, uniform)) => {
                    self.uniforms.insert(name, uniform);
                }
                Ok(UniformUpdate::Disable) => {
                    self.disabled = true;
                }
                Ok(UniformUpdate::Enable) => {
                    self.disabled = false;
                }
                Err(mpsc::TryRecvError::Disconnected) => return None,
                Err(mpsc::TryRecvError::Empty) => break,
            }
        }
        if let Some(ibo) = &mut self.ibo {
            ibo.flush(gl)?;
        }

        self.vao.update(gl)?;
        Some(())
    }
    fn render(&mut self, gl: &GL) {
        self.vao.bind(gl, &mut self.shader);

        for (name, uniform) in self.uniforms.iter() {
            if SHOW_UNIFORMS {
                console_log!("Setting uniform {} {:?}", name, uniform);
            }

            if self.shader.uniform(gl, &name, &uniform).is_none() {
                console_log!("Failed etting uniform {} {:?}", name, uniform);
            }
        }

        if let Some(ibo) = &self.ibo {
            ibo.bind(gl);

            gl.draw_elements_with_i32(GL::TRIANGLES, ibo.get_count() as i32, GL::UNSIGNED_SHORT, 0);
        } else {
            gl.draw_arrays(GL::TRIANGLES, 0, self.vao.get_count())
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }
}

impl BatchRenderableTrait for DefaultRenderable {
    fn draw(&mut self, gl: &GL) {
        if let Some(ibo) = &self.ibo {
            gl.draw_elements_with_i32(GL::TRIANGLES, ibo.get_count() as i32, GL::UNSIGNED_SHORT, 0);
        } else {
            gl.draw_arrays(GL::TRIANGLES, 0, self.vao.get_count())
        }
    }
    fn bind(&mut self, gl: &GL) -> std::option::Option<()> {
        self.vao.bind(gl, &mut self.shader);
        if let Some(ibo) = &self.ibo {
            ibo.bind(gl);
        }
        Some(())
    }
    fn shader(&mut self) -> &mut Shader {
        &mut self.shader
    }
}

enum BatchRenderableHandleUpdate {
    Create(mpsc::Sender<UniformUpdate>, mpsc::Receiver<UniformUpdate>),
}

#[derive(Clone)]
pub struct BatchRenderableHandle {
    inner: mpsc::Sender<BatchRenderableHandleUpdate>,
}

impl BatchRenderableHandle {
    pub fn push(&self) -> Option<UniformsHandle> {
        let (tx, rx) = mpsc::channel();
        self.inner
            .send(BatchRenderableHandleUpdate::Create(tx.clone(), rx))
            .ok()?;
        Some(UniformsHandle::new(tx))
    }
}

pub struct BatchRenderable<R: BatchRenderableTrait> {
    inner: R,
    uniforms: Vec<(
        mpsc::Receiver<UniformUpdate>,
        HashMap<String, Box<dyn Uniform>>,
    )>,
    senders: Vec<mpsc::Sender<UniformUpdate>>,
    handle: (
        mpsc::Sender<BatchRenderableHandleUpdate>,
        mpsc::Receiver<BatchRenderableHandleUpdate>,
    ),
}

impl<R: BatchRenderableTrait> BatchRenderable<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            uniforms: Vec::new(),
            senders: Vec::new(),
            handle: mpsc::channel(),
        }
    }

    pub fn handle(&self) -> BatchRenderableHandle {
        BatchRenderableHandle {
            inner: self.handle.0.clone(),
        }
    }

    pub fn push(&mut self) -> UniformsHandle {
        let (tx, rx) = mpsc::channel();
        self.uniforms.push((rx, HashMap::new()));
        self.senders.push(tx.clone());
        UniformsHandle::new(tx)
    }
}

impl<R: BatchRenderableTrait> Renderable for BatchRenderable<R> {
    fn render(&mut self, gl: &GL) {
        self.inner.bind(gl);
        for (_, uniforms) in self.uniforms.iter_mut() {
            let shader = self.inner.shader();
            for (name, uniform) in uniforms.iter() {
                if shader.uniform(gl, &name, &uniform).is_none() {
                    console_log!("Failed etting uniform {} {:?}", name, uniform);
                }
            }

            self.inner.draw(gl);
        }
    }
    fn update(&mut self, gl: &GL) -> Option<()> {
        loop {
            match self.handle.1.try_recv() {
                Ok(BatchRenderableHandleUpdate::Create(tx, rx)) => {
                    self.uniforms.push((rx, HashMap::new()));
                    self.senders.push(tx.clone());
                }
                Err(mpsc::TryRecvError::Disconnected) => return None,
                Err(mpsc::TryRecvError::Empty) => break,
            }
        }
        self.inner.update(gl)?;

        for (rx, uniforms) in self.uniforms.iter_mut() {
            loop {
                match rx.try_recv() {
                    Ok(UniformUpdate::Batch(context)) => {
                        uniforms.extend(context.into_iter());
                    }
                    Ok(UniformUpdate::Single(name, uniform)) => {
                        uniforms.insert(name, uniform);
                    }
                    Ok(UniformUpdate::Disable) => {
                        unreachable!();
                    }
                    Ok(UniformUpdate::Enable) => {
                        unreachable!();
                    }
                    Err(mpsc::TryRecvError::Disconnected) => return None,
                    Err(mpsc::TryRecvError::Empty) => break,
                }
            }
        }

        Some(())
    }
    fn is_disabled(&self) -> bool {
        self.inner.is_disabled()
    }
}

pub struct Renderer {
    layers: HashMap<usize, Vec<Box<dyn Renderable>>>,
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

    pub fn add_renderable<R: Renderable + 'static>(&mut self, item: R, layer: usize) -> usize {
        if self.sorted_layers.insert(layer) {
            self.layers.insert(layer, Vec::new());
        }

        let layer = self.layers.get_mut(&layer).unwrap();
        let out = layer.len();
        layer.push(Box::new(item));

        out
    }

    pub fn update(&mut self, gl: &GL) -> Option<()> {
        for layer_idx in self.sorted_layers.iter() {
            if let Some(layer) = self.layers.get_mut(layer_idx) {
                for renderable in layer.iter_mut() {
                    // FIXME maybe only update if renderable is enabled?
                    renderable.update(gl)?;
                }
            }
        }
        Some(())
    }

    pub fn render(&mut self, gl: &GL) {
        for layer_idx in self.sorted_layers.iter() {
            if let Some(layer) = self.layers.get_mut(layer_idx) {
                for renderable in layer.iter_mut() {
                    if !renderable.is_disabled() {
                        renderable.render(gl);
                    }
                }
            }
        }
    }
}
