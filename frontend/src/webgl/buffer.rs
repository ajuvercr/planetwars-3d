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

    #[derive(Debug)]
    pub struct Buffer<T, A: Deref<Target = [T]>> {
        buffer: WebGlBuffer,
        data: Option<A>,
        count: usize,
        target: u32,
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
    use crate::Shader;
    use web_sys::WebGlRenderingContext as GL;
    use yew::services::ConsoleService;

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
                        ConsoleService::error(&format!("Location {} not found", element.index));
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
