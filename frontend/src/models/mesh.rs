use cgmath::{Matrix4, Vector3};

use crate::gl::GL;
use crate::{
    engine::{
        physics::{EntityPhysics, Physics, PhysicsBuilder},
        Entity, ObjectFactory, Renderer,
    },
};

const SHIP_BYTES: &'static [u8] = include_bytes!("../../res/ship.obj");

pub async fn load_ship() -> Option<(Vec<[f32; 3]>, Vec<[usize; 3]>)> {
    use std::io::Cursor;

    let mut verts = Vec::new();
    let mut faces = Vec::new();

    let load = tobj::load_obj_buf(&mut Cursor::new(SHIP_BYTES), true, |p| {
        console_log!("Unexpected material load: {}", p.display());
        unreachable!()
    });

    let mesh = match load {
        Ok((mut model, _material)) => model.pop()?.mesh,
        Err(e) => {
            console_log!("Loading failed {:?}", e);
            return None;
        }
    };

    let positions = &mesh.positions;
    let indices = &mesh.indices;

    for i in (0..positions.len()).step_by(3) {
        verts.push([positions[i], positions[i + 1], positions[i + 2]]);
    }

    for i in (0..indices.len()).step_by(3) {
        faces.push([
            indices[i] as usize,
            indices[i + 1] as usize,
            indices[i + 2] as usize,
        ]);
    }

    Some((verts, faces))
}

const ROCKET_DAE: &'static str = include_str!("../../res/rocket.dae");

pub async fn load_rocket() -> Option<Vec<(String, Vec<[f32; 3]>, Vec<[usize; 3]>)>> {
    let doc = collada::document::ColladaDocument::from_str(ROCKET_DAE).ok()?;
    let os = doc.get_obj_set()?.objects;

    let mut out_objects = Vec::new();

    for obj in &os {
        let name = obj.name.clone();

        let verts = obj
            .vertices
            .iter()
            .map(|collada::Vertex { x, y, z }| [*x as f32, *y as f32, *z as f32])
            .collect();
        let mut faces = Vec::new();

        for geo in &obj.geometry {
            for mesh in &geo.mesh {
                let triags = match mesh {
                    collada::PrimitiveElement::Polylist(_) => return None,
                    collada::PrimitiveElement::Triangles(ref triag) => triag,
                };

                for (v1, v2, v3) in &triags.vertices {
                    faces.push([*v1, *v2, *v3]);
                }
            }
        }

        out_objects.push((name, verts, faces));
    }

    Some(out_objects)
}

pub struct RocketFactory {
    balls: [ObjectFactory; 3],
    rocket: ObjectFactory,
}

impl RocketFactory {
    pub fn new(factories: Vec<(String, ObjectFactory)>) -> Option<Self> {
        let mut b1 = None;
        let mut b2 = None;
        let mut b3 = None;
        let mut rock = None;

        for (s, fac) in factories {
            match &s[..] {
                "Balls1" => b1 = Some(fac),
                "Balls2" => b2 = Some(fac),
                "Balls3" => b3 = Some(fac),
                "Rocket" => rock = Some(fac),
                _ => {
                    console_log!("Didn't expect object {}", s);
                }
            }
        }

        Some(RocketFactory {
            balls: [b1?, b2?, b3?],
            rocket: rock?,
        })
    }

    pub fn create(
        &self,
        gl: &GL,
        renderer: &mut Renderer,
    ) -> Option<impl Physics<Matrix4<f32>, ()>> {
        let base_entity = Entity::default()
            .with_position(Vector3::new(0.0, -20.0, -100.0))
            .with_hom_scale(10.0)
            .with_rotation(Vector3::new(-90.0, 0.0, 0.0));
        let mut builder = PhysicsBuilder::new(EntityPhysics::new(base_entity, None));

        {
            let balls1 = self.balls[0].create_renderable(gl)?;
            let balls_entity = Entity::default().with_ang_speed(Vector3::new(0.0, 0.0, 300.0));
            builder = builder
                .enter(EntityPhysics::new(balls_entity, balls1.handle()))
                .close();
            renderer.add_renderable(balls1, 5);
        }

        {
            let balls2 = self.balls[1].create_renderable(gl)?;
            let balls_entity = Entity::default().with_ang_speed(Vector3::new(0.0, 0.0, -500.0));
            builder = builder
                .enter(EntityPhysics::new(balls_entity, balls2.handle()))
                .close();
            renderer.add_renderable(balls2, 5);
        }

        {
            let balls3 = self.balls[2].create_renderable(gl)?;
            let balls_entity = Entity::default().with_ang_speed(Vector3::new(0.0, 0.0, 150.0));
            builder = builder
                .enter(EntityPhysics::new(balls_entity, balls3.handle()))
                .close();
            renderer.add_renderable(balls3, 5);
        }

        {
            let rocket = self.rocket.create_renderable(gl)?;
            let rocket_entity = Entity::default();
            builder = builder
                .enter(EntityPhysics::new(rocket_entity, rocket.handle()))
                .close();
            renderer.add_renderable(rocket, 5);
        }

        Some(builder.finish())
    }
}
