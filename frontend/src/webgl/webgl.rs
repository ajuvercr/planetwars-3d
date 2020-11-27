use crate::{SettingsInst, models::gen_cube_faces};
use crate::models::gen_sphere_faces;
use crate::universe::Planets;
use crate::universe::Universe;
use crate::util;
use crate::webgl::renderer::BatchRenderable;
use crate::webgl::renderer::BatchRenderableHandle;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::engine::{Object, ObjectConfig, ObjectFactory};
use pw_settings::SettingsTrait;

use super::{renderer::Renderer, Shader};
use crate::uniform::Uniform3f;
use crate::{
    engine::{Camera, CameraHandle, Entity},
    set_settings,
};
use cgmath::Vector3;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;

#[wasm_bindgen]
pub struct WebGl {
    canvas: HtmlCanvasElement,
    gl: GL,

    objects: Vec<Object>,
    universe: Universe,

    camera: Camera,
    camera_handle: CameraHandle,

    renderer: Renderer,

    fps_counter: util::FpsCounter,
}

unsafe impl Send for WebGl {}

unsafe impl Sync for WebGl {}

fn create_object(r: &BatchRenderableHandle, entity: Entity) -> Option<Object> {
    let handle = r.push()?;
    handle.single(
        "u_reverseLightDirection",
        Uniform3f::new(0.28735632183908044, 0.4022988505747126, 0.5747126436781609),
    );
    Some(Object::new(handle, entity))
}

#[wasm_bindgen]
impl WebGl {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: String) -> Result<WebGl, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");

        let document = window.document().expect("should have a document on window");
        let canvas: HtmlCanvasElement = document
            .get_element_by_id(&canvas_id)
            .unwrap()
            .dyn_into()
            .unwrap();

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let camera = Camera::new();
        let camera_handle = camera.handle();
        // camera_handle.reset_position(0.0, 0.0, 5.0);

        Ok(Self {
            canvas,
            gl,

            objects: Vec::new(),
            universe: Universe::place_holder(),

            camera,
            camera_handle,

            renderer: Renderer::new(),
            fps_counter: util::FpsCounter::new(),
        })
    }

    pub fn camera_handle(&self) -> CameraHandle {
        self.camera_handle.clone()
    }

    pub fn resize(&mut self) {
        let width = self.canvas.parent_element().unwrap().client_width();
        let height = self.canvas.parent_element().unwrap().client_height();

        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.gl.viewport(0, 0, width, height);

        self.camera_handle.set_aspect(width as f32 / height as f32);
    }

    pub async fn init_renderer(mut self) -> Result<WebGl, JsValue> {
        self.resize();

        let gl = &self.gl;

        // Clear the canvas AND the depth buffer.
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // Turn on culling. By default backfacing triangles
        // will be culled.
        gl.enable(GL::CULL_FACE);

        // Enable the depth buffer
        gl.enable(GL::DEPTH_TEST);

        {
            use pw_settings::SettingsTrait;
            let settings = SettingsInst::new_settings();
            console_log!("Settings {:?}", settings);
            // let settings = SettingsInst::default_settings
        }

        // {
        //     let planets = self.universe.init(gl, &mut self.renderer, "universe.json").await?;

        //     let js_value = JsValue::from_serde(&planets.to_settings())
        //         .map_err(|_| "Serde Failed")
        //         .unwrap();
        //     println!("js value {:?}", js_value);
        //     unsafe { set_settings(js_value) };
        // }

        let shader_factory = {
            let vert_source = util::fetch("shaders/basic.vert").await?;
            let frag_source = util::fetch("shaders/basic.frag").await?;
            Shader::factory(frag_source, vert_source)
        };

        let sphere_factory = {
            let (verts, faces) = gen_sphere_faces(3);
            ObjectFactory::new(ObjectConfig::Mean, verts, faces, shader_factory.clone())
        };

        let cube_factory = {
            let (verts, faces) = gen_cube_faces();
            ObjectFactory::new(ObjectConfig::Simple, verts, faces, shader_factory.clone())
        };

        let ship_factory = {
            let (verts, faces) = util::load_ship().await.ok_or("Ship loading failed!")?;
            ObjectFactory::new(ObjectConfig::Mean, verts, faces, shader_factory.clone())
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

        let ship_creation_handle = {
            let ship_renderable = BatchRenderable::new(
                ship_factory
                    .create_renderable(gl)
                    .ok_or("Failed to created renderable ship")?,
            );
            let handle = ship_renderable.handle();
            self.renderer.add_renderable(ship_renderable, 0);
            handle
        };

        for i in -10..10 {
            for j in 0..7 {
                let mut sphere_entity = sphere_entity.clone();
                sphere_entity
                    .set_position(Vector3::new((i * 50) as f32, (j * 100) as f32, -500.0).into());
                self.objects
                    .push(create_object(&ship_creation_handle, sphere_entity).ok_or("bla")?);
            }
        }

        let sphere_factory = {
            let (verts, faces) = gen_sphere_faces(1);
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

    pub fn handle_client_update(&mut self, val: &JsValue) {
        match val.into_serde::<Planets>() {
            Ok(planets) => match self.universe.set_planets(&planets) {
                Ok(_) => {}
                Err(e) => {
                    console_log!("Woops something failed {:?}", e)
                }
            },
            Err(e) => {
                console_log!("Serde failed {:?}", e)
            }
        }
    }

    pub fn update(&mut self, dt: f64) -> Result<(), JsValue> {
        self.fps_counter.update(dt);

        self.camera.update().ok_or("Couldn't update camera")?;
        let gl = &self.gl;

        let camera = &self.camera;

        self.universe.update(dt, camera);
        self.objects
            .iter_mut()
            .for_each(|object| object.update(dt as f32, camera));

        self.renderer
            .update(gl)
            .ok_or("Renderer didn't update well")?;
        Ok(())
    }

    pub fn render_gl(&mut self) -> Result<(), JsValue> {
        self.renderer.render(&self.gl);
        Ok(())
    }
}
