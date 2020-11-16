use super::{Camera, Entity};
use crate::uniform::{Uniform3f, UniformMat4};
use crate::webgl::buffer::{IndexBuffer, VertexArray, VertexBuffer, VertexBufferLayout};
use crate::webgl::renderer::{DefaultRenderable, Renderer};
use crate::webgl::shader::ShaderFactory;
use crate::webgl::uniform::UniformsHandle;

use std::collections::HashMap;
use web_sys::WebGlRenderingContext as GL;

use super::{Float, Index, Mesh, Vector};

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

fn face_weight(vertices: &Vec<Vector<Float>>, i1: Index, i2: Index, i3: Index) -> Float {
    // A = (x1y2 + x2y3 + x3y1 – x1y3 – x2y1 – x3y2)/2.
    let [x1, y1, z1] = vertices[i1];
    let [x2, y2, z2] = vertices[i2];
    let [x3, y3, z3] = vertices[i3];

    0.5 * ((x2 * y1 - x3 * y1 - x1 * y2 + x3 * y2 + x1 * y3 - x2 * y3).powi(2)
        + (x2 * z1 - x3 * z1 - x1 * z2 + x3 * z2 + x1 * z3 - x2 * z3).powi(2)
        + (y2 * z1 - y3 * z1 - y1 * z2 + y3 * z2 + y1 * z3 - y2 * z3).powi(2))
    .sqrt()
}

#[inline]
fn push_vector<A>([x, y, z]: Vector<A>, vs: &mut Vec<A>) {
    vs.push(x);
    vs.push(y);
    vs.push(z);
}

fn build_normal_vertices(vertices: &Vec<Vector<Float>>, faces: &Vec<Vector<Index>>) -> Mesh {
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

    Mesh::NotIndexed {
        vertices: o_vertices,
        normals: o_normals,
    }
}

fn build_mean_vertices(vertices: &Vec<Vector<Float>>, faces: &Vec<Vector<Index>>) -> Mesh {
    let mut o_vertices = Vec::new();
    let mut o_normals = Vec::new();
    let mut o_indices = Vec::new();

    let v_count = vertices.len();
    let mut map = Vec::with_capacity(v_count);
    for _ in 0..v_count {
        map.push(Vec::new());
    }

    let mut face_normals = Vec::with_capacity(faces.len());
    let mut face_weights = Vec::with_capacity(faces.len());
    for (i, &[f1, f2, f3]) in faces.iter().enumerate() {
        face_normals.push(face_normal(vertices, [f1, f2, f3]));
        face_weights.push(face_weight(vertices, f1, f2, f3));

        map[f1].push(i);
        map[f2].push(i);
        map[f3].push(i);

        o_indices.push(f1 as u16);
        o_indices.push(f2 as u16);
        o_indices.push(f3 as u16);
    }

    for (i, vertex) in vertices.iter().enumerate() {
        let [mut x, mut y, mut z] = [0.0, 0.0, 0.0];
        for face_index in &map[i] {
            let [dx, dy, dz] = face_normals[*face_index];
            let weight = face_weights[*face_index];
            x += dx * weight;
            y += dy * weight;
            z += dz * weight;
        }

        push_vector(*vertex, &mut o_vertices);
        push_vector(normalize([x, y, z]), &mut o_normals);
    }

    Mesh::Indexed {
        vertices: o_vertices,
        normals: o_normals,
        indices: o_indices,
    }
}

pub fn build_vertices(
    config: ObjectConfig,
    vertices: &Vec<Vector<Float>>,
    faces: &Vec<Vector<Index>>,
) -> Mesh {
    match config {
        ObjectConfig::Simple => build_normal_vertices(vertices, faces),
        ObjectConfig::Mean => build_mean_vertices(vertices, faces),
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ObjectConfig {
    Simple,
    Mean,
}

pub struct ObjectFactory {
    settings: ObjectConfig,

    object: (Vec<Vector<Float>>, Vec<Vector<Index>>),

    mesh: Mesh,

    shader_factory: ShaderFactory,
}

impl ObjectFactory {
    pub fn new(
        settings: ObjectConfig,
        verts: Vec<Vector<Float>>,
        faces: Vec<Vector<Index>>,
        shader_factory: ShaderFactory,
    ) -> Self {
        let mesh = build_vertices(settings, &verts, &faces);

        Self {
            settings,
            mesh,
            object: (verts, faces),
            shader_factory,
        }
    }

    /// I don't know if this function is useful at all, it's more like a 'similar' function, same Config etc.
    pub fn update_object(&mut self, verts: Vec<Vector<Float>>, faces: Vec<Vector<Index>>) {
        let mesh = build_vertices(self.settings, &verts, &faces);

        self.object = (verts, faces);
        self.mesh = mesh;
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

        let (vertex_buffer, normal_buffer, index_buffer) = match &self.mesh {
            Mesh::Indexed {
                vertices,
                normals,
                indices,
            } => (
                VertexBuffer::vertex_buffer(gl, vertices.clone())?,
                VertexBuffer::vertex_buffer(gl, normals.clone())?,
                IndexBuffer::index_buffer(gl, indices.clone()),
            ),
            Mesh::NotIndexed { vertices, normals } => (
                VertexBuffer::vertex_buffer(gl, vertices.clone())?,
                VertexBuffer::vertex_buffer(gl, normals.clone())?,
                None,
            ),
        };

        let mut layout = VertexBufferLayout::new();
        layout.push(GL::FLOAT, 3, 4, "a_position", false);

        let mut normal_layout = VertexBufferLayout::new();
        normal_layout.push(GL::FLOAT, 3, 4, "a_normal", false);

        let mut vao = VertexArray::new();
        vao.add_buffer(vertex_buffer, layout);
        vao.add_buffer(normal_buffer, normal_layout);

        Some(DefaultRenderable::new(index_buffer, vao, shader, None))
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
