mod planet;
use crate::engine::{Object, ObjectConfig, ObjectFactory};
use crate::models::gen_sphere_faces;
use crate::engine::rendering::uniform::Uniform3f;
use crate::engine::rendering::renderer::{BatchRenderable, BatchRenderableHandle};
use crate::{engine::{Camera, Renderer}, util::fetch};

use crate::engine::rendering::uniform::UniformsHandle;
use crate::engine::rendering::shader::Shader;
pub use planet::Planet;
use pw_derive::Settings;
use serde::{Deserialize, Serialize};

use cgmath::Vector3;

use crate::gl::GL;
use serde_json;

#[derive(Debug, Clone, Settings, Serialize, Deserialize)]
pub struct Planets {
    planets: Vec<Planet>,
}

impl Default for Planets {
    fn default() -> Self {
        Self {
            planets: Vec::new(),
        }
    }
}

impl Planets {
    pub async fn load(location: &str) -> Self {
        let ms = fetch(location).await;
        match ms.and_then(|s| serde_json::from_str(&s).map_err(|e| format!("{:?}", e))) {
            Ok(e) => e,
            Err(e) => {
                console_log!("Planets failed {:?}", e);
                Self::default()
            }
        }
    }

    pub fn save(&self, _location: &str) {
        println!("{}", serde_json::to_string_pretty(self).unwrap())
    }
}

pub struct Universe {
    last_clicked: Vec<usize>,
    uniforms: Vec<UniformsHandle>,
    objects: Vec<Object>,
    planet_factory: BatchRenderableHandle,
}

pub const PLANET_LAYER: usize = 0;

impl Universe {
    /// Creates a non functional Universe, like the real one.
    /// Call and wait for `Universe::init` before use!
    pub fn place_holder() -> Self {
        Self {
            last_clicked: Vec::new(),
            uniforms: Vec::new(),
            objects: Vec::new(),
            planet_factory: BatchRenderableHandle::place_holder(),
        }
    }

    pub fn handle_click(&mut self, origin: Vector3<f32>, direction: Vector3<f32>) {
        for i in self.last_clicked.drain(..) {
            self.uniforms[i].single("u_color", Uniform3f::new(1.0, 1.0, 1.0));
        }

        for (i, (o, u)) in self.objects.iter().zip(&self.uniforms).enumerate() {
            if o.click_hit(origin, direction) {
                self.last_clicked.push(i);
                u.single("u_color", Uniform3f::new(1.0, 0.0, 1.0));
            }
        }
    }

    pub async fn init(
        &mut self,
        gl: &GL,
        renderer: &mut Renderer,
        location: &str,
    ) -> Result<Planets, String> {
        self.planet_factory = {
            let vert_source = fetch("shaders/basic.vert").await?;
            let frag_source = fetch("shaders/basic.frag").await?;
            let shader_factory = Shader::factory(frag_source, vert_source);

            let (verts, faces) = gen_sphere_faces(3);
            let factory =
                ObjectFactory::new(ObjectConfig::Mean, verts, faces, shader_factory.clone());
            let renderable = factory
                .create_renderable(gl)
                .ok_or("Failed to create planet renderable")?;

            let ship_renderable = BatchRenderable::new(renderable);
            let handle = ship_renderable.handle();
            renderer.add_renderable(ship_renderable, 0);
            handle
        };

        let planets = Planets::load(location).await;
        self.set_planets(&planets)?;

        Ok(planets)
    }

    pub fn set_planets(&mut self, planets: &Planets) -> Result<(), String> {
        // Set visable or not, or create new Objects
        for planet in &planets.planets[self.objects.len()..] {
            let handle = self
                .planet_factory
                .push()
                .ok_or("Couldn't push hard enough")?;
            handle.single(
                "u_reverseLightDirection",
                Uniform3f::new(0.28735632183908044, 0.4022988505747126, 0.5747126436781609),
            );
            handle.single("u_color", Uniform3f::new(1.0, 1.0, 1.0));
            let obj = Object::new(handle, planet.location.clone());
            self.uniforms.push(obj.uniform_handle());
            self.objects.push(obj);
        }

        // Set new planet's entities
        for (planet, object) in planets.planets.iter().zip(&mut self.objects) {
            if planet.disabled {
                object.disable();
            } else {
                object.enable();
            }

            object.set_entity(planet.location.clone());
        }

        for object in &self.objects[planets.planets.len()..] {
            object.disable();
        }

        Ok(())
    }

    pub fn update(&mut self, dt: f64, camera: &Camera) {
        self.objects
            .iter_mut()
            .for_each(|o| o.update(dt as f32, camera));
    }
}
