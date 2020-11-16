mod planet;
use crate::engine::Object;
use crate::webgl::renderer::Renderable;
use std::collections::{HashMap, VecDeque};

use crate::{buffer::VertexArray, renderer::DefaultRenderable, util::*};
use crate::{
    buffer::VertexBuffer, buffer::VertexBufferLayout, engine::Entity, models, shader::Shader,
    webgl::renderer::Renderer,
};
pub use planet::Planet;
use pw_derive::Settings;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use serde_json;
use web_sys::WebGlRenderingContext as GL;

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
        ms.ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, _location: &str) {
        println!("{}", serde_json::to_string_pretty(self).unwrap())
    }
}

enum Action {
    Planets(Planets),
}

pub struct Universe {
    objects: Vec<Object>,
    planets: Planets,

    queue: VecDeque<Action>,
}

pub const PLANET_LAYER: usize = 0;

impl Universe {
    pub async fn init(
        gl: &GL,
        planet_count: usize,
        renderer: &mut Renderer,
    ) -> Result<Self, JsValue> {
        let vert_source = fetch("shaders/basic.vert").await?;
        let frag_source = fetch("shaders/basic.frag").await?;
        let shader_factory = Shader::factory(frag_source, vert_source);

        let vertices = models::gen_sphere_icosahedral(5.0);

        let mut u_handles = Vec::new();

        for i in 0..planet_count {
            let entity = Entity::default().with_hom_scale(0.0);
            let name = format!("Planet {}", i);

            let planet = Planet::new(name, entity);

            let vertex_buffer = VertexBuffer::vertex_buffer(gl, vertices.clone())
                .ok_or("Failed to get vertices")?;

            let mut layout = VertexBufferLayout::new();
            layout.push(GL::FLOAT, 3, 4, "a_position", false);
            layout.push(GL::FLOAT, 3, 4, "a_normal", false);

            let mut vao = VertexArray::new();
            vao.add_buffer(vertex_buffer, layout);

            let shader = shader_factory
                .create_shader(gl, HashMap::new())
                .ok_or("failed to create new shader")?;
            let sphere_renderable = DefaultRenderable::new(None, vao, shader, None);
            u_handles.push(sphere_renderable.handle());
            renderer.add_renderable(sphere_renderable, PLANET_LAYER);
        }

        Ok(Self {
            objects: Vec::new(),
            planets: Planets {
                planets: Vec::new(),
            },
            queue: VecDeque::new(),
        })
    }

    pub fn set_planets(&mut self, planets: Planets) -> Result<(), JsValue> {
        self.queue.push_back(Action::Planets(planets));
        Ok(())
    }
}

impl Renderable for Universe {
    fn render(&mut self, gl: &GL) {
        todo!()
    }

    fn update(&mut self, gl: &GL) -> Option<()> {
        for action in self.queue.drain(..) {
            match action {
                Action::Planets(planets) => {}
            }
        }

        todo!()
    }
}
