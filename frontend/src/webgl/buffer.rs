pub use buffer::{Buffer, BufferHandle, BufferTrait, IndexBuffer, VertexBuffer};
mod buffer {
    use std::ops::Deref;

    use std::sync::mpsc;
    use web_sys::WebGlRenderingContext as GL;
    use web_sys::*;

    pub type VertexBuffer = Buffer<f32, Vec<f32>>;
    pub type IndexBuffer = Buffer<u16, Vec<u16>>;

    enum BufferChange<A> {
        Reset(Box<A>),
        Update(Box<A>, usize), // Data start index
    }

    #[derive(Clone, Debug)]
    pub struct BufferHandle<A> {
        sender: mpsc::Sender<BufferChange<A>>,
    }

    impl<A> BufferHandle<A> {
        pub fn reset<B: Into<Box<A>>>(&self, data: B) -> Option<()> {
            self.sender.send(BufferChange::Reset(data.into())).ok()
        }

        pub fn update<B: Into<Box<A>>>(&self, data: B, start: usize) -> Option<()> {
            self.sender
                .send(BufferChange::Update(data.into(), start))
                .ok()
        }
    }

    pub trait PrivBufferTrait<A> {
        fn update(&mut self, gl: &GL, data: Box<A>, start: usize);
        fn reset(&mut self, gl: &GL, data: Box<A>);
    }

    pub trait BufferTrait {
        fn bind(&self, gl: &GL);
        fn get_count(&self) -> usize;
        fn flush(&mut self, gl: &GL) -> Option<()>;
    }

    #[derive(Debug)]
    pub struct Buffer<T, A: Deref<Target = [T]>> {
        buffer: WebGlBuffer,
        data: Option<Box<A>>,
        count: usize,
        target: u32,

        tx: mpsc::Sender<BufferChange<A>>,
        rx: mpsc::Receiver<BufferChange<A>>,
    }

    impl<T, A: Deref<Target = [T]>> Buffer<T, A> {
        pub fn new<B: Into<Option<A>>>(gl: &GL, data: B, target: u32) -> Option<Self> {
            let buffer = gl.create_buffer()?;

            let (tx, rx) = mpsc::channel();

            if let Some(data) = data.into() {
                tx.send(BufferChange::Reset(Box::new(data))).ok()?;
            }

            let this = Self {
                count: 0,
                data: None,
                target,
                buffer,
                tx,
                rx,
            };

            Some(this)
        }

        pub fn vertex_buffer<B: Into<Option<A>>>(gl: &GL, data: B) -> Option<Self> {
            Buffer::new(gl, data, GL::ARRAY_BUFFER)
        }

        pub fn index_buffer<B: Into<Option<A>>>(gl: &GL, data: B) -> Option<Self> {
            Buffer::new(gl, data, GL::ELEMENT_ARRAY_BUFFER)
        }

        pub fn handle(&self) -> BufferHandle<A> {
            BufferHandle {
                sender: self.tx.clone(),
            }
        }
    }

    impl<A: Deref<Target = [f32]>> PrivBufferTrait<A> for Buffer<f32, A> {
        fn update(&mut self, _gl: &GL, _data: Box<A>, _start: usize) {
            unimplemented!();
        }
        fn reset(&mut self, gl: &GL, data: Box<A>) {
            self.count = data.len();
            self.data = Some(data);

            let verts = unsafe { js_sys::Float32Array::view(self.data.as_ref().unwrap()) };
            gl.bind_buffer(self.target, Some(&self.buffer));
            gl.buffer_data_with_array_buffer_view(self.target, &verts, GL::STATIC_DRAW);
        }
    }

    impl<A: Deref<Target = [i32]>> PrivBufferTrait<A> for Buffer<i32, A> {
        fn update(&mut self, _gl: &GL, _data: Box<A>, _start: usize) {
            unimplemented!();
        }
        fn reset(&mut self, gl: &GL, data: Box<A>) {
            self.count = data.len();
            self.data = Some(data);

            let verts = unsafe { js_sys::Int32Array::view(self.data.as_ref().unwrap()) };
            gl.bind_buffer(self.target, Some(&self.buffer));
            gl.buffer_data_with_array_buffer_view(self.target, &verts, GL::STATIC_DRAW);
        }
    }

    impl<A: Deref<Target = [u16]>> PrivBufferTrait<A> for Buffer<u16, A> {
        fn update(&mut self, _gl: &GL, _data: Box<A>, _start: usize) {
            unimplemented!();
        }
        fn reset(&mut self, gl: &GL, data: Box<A>) {
            self.count = data.len();
            self.data = Some(data);

            let verts = unsafe { js_sys::Uint16Array::view(self.data.as_ref().unwrap()) };
            gl.bind_buffer(self.target, Some(&self.buffer));
            gl.buffer_data_with_array_buffer_view(self.target, &verts, GL::STATIC_DRAW);
        }
    }

    impl<T, A: Deref<Target = [T]>> BufferTrait for Buffer<T, A>
    where
        Self: PrivBufferTrait<A>,
    {
        fn bind(&self, gl: &GL) {
            gl.bind_buffer(self.target, Some(&self.buffer));
        }

        fn get_count(&self) -> usize {
            self.count
        }

        fn flush(&mut self, gl: &GL) -> Option<()> {
            loop {
                match self.rx.try_recv() {
                    Ok(BufferChange::Update(data, start)) => {
                        self.update(gl, data, start);
                    }
                    Ok(BufferChange::Reset(data)) => {
                        self.reset(gl, data);
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => return None,
                }
            }
            Some(())
        }
    }
}

pub use vertex::{VertexArray, VertexBufferLayout};
mod vertex {
    use super::super::Shader;
    use web_sys::WebGlRenderingContext as GL;

    use super::{BufferTrait, VertexBuffer};

    #[derive(Debug)]
    struct VertexBufferElement {
        type_: u32,
        amount: i32,
        type_size: i32,
        index: String,
        normalized: bool,
    }

    impl VertexBufferElement {
        fn new(type_: u32, amount: i32, type_size: i32, index: String, normalized: bool) -> Self {
            Self {
                type_,
                amount,
                type_size,
                index,
                normalized,
            }
        }
    }

    #[derive(Debug)]
    pub struct VertexBufferLayout {
        elements: Vec<VertexBufferElement>,
        stride: i32,
        offset: i32,
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

    #[derive(Debug)]
    pub struct VertexArray {
        buffers: Vec<VertexBuffer>,
        layouts: Vec<VertexBufferLayout>,
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

        pub fn update(&mut self, gl: &GL) -> Option<()> {
            for buffer in self.buffers.iter_mut() {
                buffer.flush(gl)?;
            }
            Some(())
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
                        console_log!("Location {} not found", element.index);
                    }

                    offset += element.amount * element.type_size;
                }
            }
        }

        pub fn get_count(&self) -> i32 {
            self.layouts
                .iter()
                .map(|l| l.elements.iter().map(|e| e.amount).sum::<i32>())
                .zip(self.buffers.iter())
                .map(|(s, b)| b.get_count() as i32 / s)
                .min()
                .unwrap()
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
