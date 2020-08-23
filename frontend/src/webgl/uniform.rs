use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

use cgmath::{Matrix4, Vector4};
use std::{collections::HashMap, fmt::Debug, ops::Deref, sync::mpsc};

#[derive(Debug, Clone)]
pub struct UniformsHandle {
    inner: mpsc::Sender<UniformUpdate>,
}

impl UniformsHandle {
    pub fn new(tx: mpsc::Sender<UniformUpdate>) -> Self {
        Self { inner: tx }
    }
    pub fn single<S: Into<String>, U: Uniform + 'static>(&self, name: S, uniform: U) -> Option<()> {
        self.inner
            .send(UniformUpdate::Single(name.into(), Box::new(uniform)))
            .ok()
    }
    pub fn batch(&self, uniforms: HashMap<String, Box<dyn Uniform>>) -> Option<()> {
        self.inner.send(UniformUpdate::Batch(uniforms)).ok()
    }
}

pub enum UniformUpdate {
    Single(String, Box<dyn Uniform>),
    Batch(HashMap<String, Box<dyn Uniform>>),
}

/************************************************************************/
/*             Start super ugly generic code, just uniforms             */
/************************************************************************/
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

#[derive(Debug)]
pub struct UniformMat4<A> {
    data: A,
    transpose: bool,
}
impl UniformMat4<Vec<f32>> {
    pub fn new_mat4(mat: Matrix4<f32>) -> Self {
        let mut data = Vec::new();

        data.extend_from_slice(<Vector4<f32> as AsRef<[f32; 4]>>::as_ref(&mat.x));
        data.extend_from_slice(<Vector4<f32> as AsRef<[f32; 4]>>::as_ref(&mat.y));
        data.extend_from_slice(<Vector4<f32> as AsRef<[f32; 4]>>::as_ref(&mat.z));
        data.extend_from_slice(<Vector4<f32> as AsRef<[f32; 4]>>::as_ref(&mat.w));

        Self {
            data,
            transpose: false,
        }
    }
}
impl<A: Deref<Target = [f32]>> UniformMat4<A> {
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
impl<A: Deref<Target = [f32]> + Debug> Uniform for UniformMat4<A> {
    fn set_uniform(&self, gl: &GL, location: &WebGlUniformLocation) {
        gl.uniform_matrix4fv_with_f32_array(Some(location), self.transpose, self.data.deref());
    }
}
