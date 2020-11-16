use super::{Camera, Entity};
use crate::uniform::{Uniform3f, UniformMat4};
use crate::webgl::buffer::{VertexArray, VertexBuffer, VertexBufferLayout};
use crate::webgl::renderer::{DefaultRenderable, Renderer};
use crate::webgl::shader::ShaderFactory;
use crate::webgl::uniform::UniformsHandle;

use std::collections::HashMap;
use web_sys::WebGlRenderingContext as GL;

use super::{Float, Index, Vector};

#[inline]
fn normalize([x, y, z]: Vector<Float>) -> Vector<Float> {
    let mag = (x * x + y * y + z * z).sqrt();
    [x / mag, y / mag, z / mag]
}

fn face_normal(vertices: &Vec<Vector<Float>>, [i1, i2, i3]: Vector<Index>) -> Vector<Float> {
    // So for a triangle p1, p2, p3, if the vector U = p2 - p1 and the vector V = p3 - p1 then the normal N = U X V and can be calculated by:
    // Nx = UyVz - UzVy
    // Ny = UzVx - UxVz
    // Nz = UxVy - UyVx
    let [x1, y1, z1] = vertices[i1];
    let [x2, y2, z2] = vertices[i2];
    let [x3, y3, z3] = vertices[i3];

    normalize([
        (y2 - y1) * (z3 - z1) - (z2 - z1) * (y3 - y1),
        (z2 - z1) * (x3 - x1) - (x2 - x1) * (z3 - z1),
        (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1),
    ])
}

#[inline]
fn push_vector<A>([x, y, z]: Vector<A>, vs: &mut Vec<A>) {
    vs.push(x);
    vs.push(y);
    vs.push(z);
}

// TODO Use object config
pub fn build_vertices(
    _config: ObjectConfig,
    vertices: &Vec<Vector<Float>>,
    faces: &Vec<Vector<Index>>,
) -> (Vec<Float>, Vec<Float>) {
    let mut o_vertices = Vec::new();
    let mut o_normals = Vec::new();

    for &[i1, i2, i3] in faces {
        let normal = face_normal(vertices, [i1, i2, i3]);

        push_vector(vertices[i1], &mut o_vertices);
        push_vector(vertices[i2], &mut o_vertices);
        push_vector(vertices[i3], &mut o_vertices);

        push_vector(normal, &mut o_normals);
        push_vector(normal, &mut o_normals);
        push_vector(normal, &mut o_normals);
    }

    (o_vertices, o_normals)
}

#[derive(Copy, Clone, Debug)]
pub enum ObjectConfig {
    Simple,
}

pub struct ObjectFactory {
    settings: ObjectConfig,

    object: (Vec<Vector<Float>>, Vec<Vector<Index>>),

    vertices: Vec<Float>,
    normals: Vec<Float>,

    shader_factory: ShaderFactory,
}

impl ObjectFactory {
    pub fn new(
        settings: ObjectConfig,
        verts: Vec<Vector<Float>>,
        faces: Vec<Vector<Index>>,
        shader_factory: ShaderFactory,
    ) -> Self {
        let (vertices, normals) = build_vertices(settings, &verts, &faces);

        Self {
            settings,
            vertices,
            normals,
            object: (verts, faces),
            shader_factory,
        }
    }

    /// I don't know if this function is useful at all, it's more like a 'similar' function, same Config etc.
    pub fn update_object(&mut self, verts: Vec<Vector<Float>>, faces: Vec<Vector<Index>>) {
        let (vertices, normals) = build_vertices(self.settings, &verts, &faces);

        self.object = (verts, faces);
        self.vertices = vertices;
        self.normals = normals;
    }

    // TODO this "u_reverseLightDirection" shouldn't be here
    pub fn create(&self, gl: &GL, renderer: &mut Renderer, entity: Entity) -> Option<Object> {
        let renderable = self.create_renderable(gl)?;

        let uniforms = renderable.handle();
        uniforms.single(
            "u_reverseLightDirection",
            Uniform3f::new(0.28735632183908044, 0.4022988505747126, 0.5747126436781609),
        );
        renderer.add_renderable(renderable, 0);

        Some(Object { uniforms, entity })
    }

    pub fn create_renderable(&self, gl: &GL) -> Option<DefaultRenderable> {
        let shader = self.shader_factory.create_shader(gl, HashMap::new())?;

        let vertex_buffer = VertexBuffer::vertex_buffer(gl, self.vertices.clone())?;
        let normal_buffer = VertexBuffer::vertex_buffer(gl, self.normals.clone())?;

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);

        let mut normal_layout = VertexBufferLayout::new();
        normal_layout.push(GL::FLOAT, 3, 4, "a_normal", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);
        vao.add_buffer(normal_buffer, normal_layout);

        Some(DefaultRenderable::new(None, vao, shader, None))
    }
}

pub struct Object {
    uniforms: UniformsHandle,
    entity: Entity,
}

impl Object {
    pub fn new(uniforms: UniformsHandle, entity: Entity) -> Self {
        Self { uniforms, entity }
    }
    pub fn update(&mut self, dt: f32, camera: &Camera) {
        self.entity.update(dt);

        self.uniforms.single(
            "u_worldViewProjection",
            UniformMat4::new_mat4(camera.world_view_projection_matrix()),
        );
        self.uniforms
            .single("u_world", UniformMat4::new_mat4(self.entity.world_matrix()));
    }
}
