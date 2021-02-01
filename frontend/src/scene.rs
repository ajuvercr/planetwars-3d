use crate::{
    engine::physics::{EntityPhysics, IdPhysics, Physics, PhysicsBuilder, TransformTree},
    models::{self, RocketFactory},
};

use crate::engine::{Camera, CameraHandle, Entity};
use crate::engine::{Object, ObjectConfig, ObjectFactory};
use crate::gl::GL;
use crate::util;
use crate::FpsCounter;
use crate::{renderer::Renderer, webgl::Shader};
use cgmath::{Matrix4, Vector3};

pub struct Scene {
    objects: Vec<Object>,

    es: TransformTree<IdPhysics, Matrix4<f32>, Matrix4<f32>>,
    // universe: Universe,
    camera: Camera,
    camera_handle: CameraHandle,

    renderer: Renderer,

    fps_counter: FpsCounter,
}

fn test_es(
    gl: &GL,
    sphere_factory: &ObjectFactory,
    renderer: &mut Renderer,
) -> Option<impl Physics<Matrix4<f32>, ()>> {
    let mut builder = PhysicsBuilder::new(IdPhysics);

    builder = {
        let entity = Entity::default()
            .with_position(Vector3::new(-500.0, 0.0, -500.0))
            .with_ang_speed(Vector3::new(0.0, 4.0, 0.0));

        let mut builder = builder.enter(EntityPhysics::new(entity, None));

        let s1 = sphere_factory.create_renderable(gl)?;
        let e1 = Entity::default()
            .with_position(Vector3::new(0.0, 100.0, 0.0))
            .with_hom_scale(50.0);
        builder = builder.enter(EntityPhysics::new(e1, s1.handle())).close();
        renderer.add_renderable(s1, 4);

        let s2 = sphere_factory.create_renderable(gl)?;
        let e2 = Entity::default()
            .with_position(Vector3::new(0.0, -100.0, 0.0))
            .with_hom_scale(50.0);
        builder = builder.enter(EntityPhysics::new(e2, s2.handle())).close();
        renderer.add_renderable(s2, 4);

        let s3 = sphere_factory.create_renderable(gl)?;
        let e3 = Entity::default()
            .with_position(Vector3::new(100.0, 0.0, 0.0))
            .with_hom_scale(50.0);
        builder = builder.enter(EntityPhysics::new(e3, s3.handle())).close();
        renderer.add_renderable(s3, 4);

        builder.close()
    };

    Some(builder.finish())
}

fn build_rockets(
    gl: &GL,
    ship_factory: &RocketFactory,
    renderer: &mut Renderer,
) -> Option<impl Physics<Matrix4<f32>, ()>> {
    let mut builder = PhysicsBuilder::new(IdPhysics);

    for i in -5..5 {
        for j in -5..5 {
            let ship_entity = Entity::default().with_position(Vector3::new(
                100.0 * i as f32,
                0.0,
                100.0 * j as f32,
            ));
            let mut rocket = PhysicsBuilder::new(EntityPhysics::new(ship_entity, None));
            rocket.add_child(ship_factory.create(gl, renderer).unwrap());
            builder.add_child(rocket.finish());
        }
    }

    Some(builder.finish())
}

impl Scene {
    pub fn new() -> Scene {
        let camera = Camera::new();
        let camera_handle = camera.handle();

        Self {
            es: PhysicsBuilder::new(IdPhysics).finish(),
            objects: Vec::new(),
            // universe: Universe::place_holder(),
            camera,
            camera_handle,

            renderer: Renderer::new(),
            fps_counter: FpsCounter::new(),
        }
    }

    pub fn camera_handle(&self) -> CameraHandle {
        self.camera_handle.clone()
    }

    pub async fn init_renderer(mut self, gl: &GL) -> Result<Self, String> {
        // self.universe.init(gl, &mut self.renderer, "universe.json").await?;

        let shader_factory = {
            let vert_source = util::fetch("shaders/basic.vert").await?;
            let frag_source = util::fetch("shaders/basic.frag").await?;
            Shader::factory(frag_source, vert_source)
        };

        let sphere_factory = {
            let (verts, faces) = models::gen_sphere_faces(3);
            ObjectFactory::new(ObjectConfig::Mean, verts, faces, shader_factory.clone())
        };

        self.es
            .add_child(test_es(gl, &sphere_factory, &mut self.renderer).unwrap());

        let cube_factory = {
            let (verts, faces) = models::gen_cube_faces();
            ObjectFactory::new(ObjectConfig::Simple, verts, faces, shader_factory.clone())
        };

        let ship_factory: RocketFactory = {
            let parts = models::load_rocket().await.ok_or("Ship loading failed!")?;
            let parts = parts
                .into_iter()
                .map(|(name, verts, faces)| {
                    (
                        name,
                        ObjectFactory::new(
                            ObjectConfig::Simple,
                            verts,
                            faces,
                            shader_factory.clone(),
                        ),
                    )
                })
                .collect();
            RocketFactory::new(parts).unwrap()
        };

        // Setup sphere
        let sphere_entity = Entity::default()
            .with_position(Vector3::new(0.0, 0.0, -500.0))
            .with_ang_speed(Vector3::new(30.0, 60.0, 0.0)); //.with_speed(Vector3::new(5.0, 0.0, 10.0));
        {
            let mut sphere_entity = sphere_entity.clone();
            sphere_entity.set_position(Vector3::new(0.0, 0.0, -800.0).into());
            sphere_entity.set_scale(Vector3::new(50.0, 50.0, 50.0).into());
            self.objects.push(
                sphere_factory
                    .create(gl, &mut self.renderer, sphere_entity)
                    .ok_or("Sphere creation failed")?,
            );
        }

        self.es
            .add_child(build_rockets(gl, &ship_factory, &mut self.renderer).unwrap());

        // let ship_creation_handle = {
        //     let ship_renderable = BatchRenderable::new(
        //         ship_factory
        //             .create_renderable(gl)
        //             .ok_or("Failed to created renderable ship")?,
        //     );
        //     let handle = ship_renderable.handle();
        //     self.renderer.add_renderable(ship_renderable, 0);
        //     handle
        // };

        // for i in -2..2 {
        //     for j in 0..3 {
        //         let mut sphere_entity = sphere_entity.clone();
        //         sphere_entity
        //             .set_position(Vector3::new((i * 50) as f32, (j * 100) as f32, -500.0).into());
        //         self.objects
        //             .push(create_object(&ship_creation_handle, sphere_entity).ok_or("bla")?);
        //     }
        // }

        let sphere_factory = {
            let (verts, faces) = models::gen_sphere_faces(1);
            ObjectFactory::new(ObjectConfig::Simple, verts, faces, shader_factory.clone())
        };

        let sphere_entity = Entity::default()
            .with_position(Vector3::new(-500.0, 0.0, -500.0))
            .with_hom_scale(50.0)
            .with_ang_speed(Vector3::new(30.0, 60.0, 0.0)); //.with_speed(Vector3::new(5.0, 0.0, 10.0));
        self.objects.push(
            sphere_factory
                .create(gl, &mut self.renderer, sphere_entity)
                .ok_or("Sphere creation failed")?,
        );

        let cube_entity = Entity::default()
            .with_position(Vector3::new(500.0, 0.0, -500.0))
            .with_hom_scale(50.0)
            .with_ang_speed(Vector3::new(10.0, 30.0, 0.0)); //.with_speed(Vector3::new(5.0, 0.0, 10.0));
        self.objects.push(
            cube_factory
                .create(gl, &mut self.renderer, cube_entity)
                .ok_or("Cube creation failed")?,
        );

        // Setup floor
        let cube_entity = Entity::default()
            .with_position(Vector3::new(0.0, -100.0, 0.0))
            .with_scale(5000.0, 5.0, 5000.0);
        self.objects.push(
            cube_factory
                .create(gl, &mut self.renderer, cube_entity)
                .ok_or("Cube creation failed")?,
        );

        Ok(self)
    }

    pub fn update(&mut self, gl: &GL, dt: f64) -> Result<(), String> {
        self.fps_counter.update(dt);

        self.camera.update().ok_or("Couldn't update camera")?;

        let camera = &self.camera;
        self.renderer.world_view_projection_matrix = camera.world_view_projection_matrix();

        // self.universe.update(dt, camera);
        self.es
            .update(&Matrix4::from_scale(1.0), dt as f32, &mut self.renderer);

        self.objects
            .iter_mut()
            .for_each(|object| object.update(dt as f32, camera));

        self.renderer
            .update(gl)
            .ok_or("Renderer didn't update well")?;
        Ok(())
    }

    pub fn render_gl(&mut self, gl: &GL) -> Result<(), String> {
        self.renderer.render(gl);
        Ok(())
    }
}
