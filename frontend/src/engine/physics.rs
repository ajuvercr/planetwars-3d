use std::marker::PhantomData;
use super::Entity;


use crate::{renderer::{Renderer}, uniform::{Uniform3f, UniformMat4, UniformsHandle}};

pub struct EntityPhysics {
    core: Entity,
    handle: Option<UniformsHandle>,
}

impl EntityPhysics {
    pub fn new<H: Into<Option<UniformsHandle>>>(core: Entity, handle: H) -> Self {
        Self {core, handle: handle.into()}
    }
}

impl Physics<Matrix4<f32>, Matrix4<f32>> for EntityPhysics {
    fn update(&mut self, t: &Matrix4<f32>, dt: f32, renderer: &mut Renderer) -> Option<Matrix4<f32>> {
        self.core.update(dt);

        let mat = t * self.core.world_matrix();

        if let Some(h) = self.handle.as_mut() {
            h.single("u_world", UniformMat4::new_mat4(mat));
            h.single("u_worldViewProjection", UniformMat4::new_mat4(renderer.world_view_projection_matrix));
            h.single(
                "u_color",
                Uniform3f::new(1.0, 1.0, 1.0),
            );
            h.single(
                "u_reverseLightDirection",
                Uniform3f::new(0.28735632183908044, 0.4022988505747126, 0.5747126436781609),
            );
        }

        Some(mat)
    }
}

pub struct IdPhysics;
impl<A: Clone> Physics<A, A> for IdPhysics {
    fn update(&mut self, t: &A, _: f32, _: &mut Renderer) -> Option<A> {
        Some(t.clone())
    }
}


/// In physics, you are expected to send the world uniform to DefaultRenderable
pub trait Physics<A, B> {
    fn update(&mut self, t: &A, dt: f32, renderer: &mut Renderer) -> Option<B>;
}

pub struct TransformTree<S: Physics<A, B>, A, B> {
    pd: PhantomData<(A, B)>,
    state: S,
    children: Vec<Box<dyn Physics<B, ()>>>,
}

impl<S: Physics<A, B>, A, B> TransformTree<S, A, B> {
    fn add_child<P: Physics<B, ()> + 'static>(&mut self, child: P) {
        self.children.push(Box::new(child));
    }
}

impl<S: Physics<A, B>, A, B> Physics<A, ()> for TransformTree<S, A, B> {
    fn update(&mut self, t: &A, dt: f32, renderer: &mut Renderer) -> Option<()> {
        // Step to node
        let b = self.state.update(t, dt, renderer)?;

        // Walk children
        self.children.iter_mut()
            .for_each(|x| x.update(&b, dt, renderer).expect("Fold wasn't the right idea"));

        Some(())
    }
}

pub use builder::PhysicsBuilder;
use cgmath::Matrix4;

mod builder {
    use super::*;

    pub struct PhysicsBuilder<S: Physics<A, B>, A, B, P> {
        current: TransformTree<S, A, B>,
        parent: P,
    }

    impl<S: Physics<A, B>, A, B> PhysicsBuilder<S, A, B, ()> {
        pub fn finish(self) -> TransformTree<S, A, B> {
            self.current
        }

        pub fn new(state: S) -> Self {
            Self {
                current: TransformTree {
                    pd: PhantomData,
                    state: state,
                    children: Vec::new(),
                },
                parent: (),
            }
        }
    }

    impl<S: Physics<A, B>, A, B, P> PhysicsBuilder<S, A, B, P> {
        pub fn enter<S2: Physics<B, C>, C>(self, state: S2) -> PhysicsBuilder<S2, B, C, Self> {
            PhysicsBuilder {
                current: TransformTree {
                    pd: PhantomData,
                    state: state,
                    children: Vec::new(),
                },
                parent: self,
            }
        }
    }

    impl<S: Physics<A, B>, A, B: 'static, S2: Physics<B, C> + 'static, C: 'static, P> PhysicsBuilder<S2, B, C, PhysicsBuilder<S, A, B, P>> {
        pub fn close(self) -> PhysicsBuilder<S, A, B, P> {
            let PhysicsBuilder { current, mut parent} = self;
            parent.current.add_child(current);
            parent
        }
    }
}
