use super::{Entity, Vec3};
use crate::set_info;
use cgmath::Vector4;
use cgmath::{perspective, prelude::SquareMatrix, Deg, Matrix4, Vector3};
use cgmath::InnerSpace;
use std::sync::mpsc;
use wasm_bindgen::prelude::*;
pub enum CameraEvent {
    AddAngle(Vector3<f32>),
    ResetAngle(Vector3<f32>),

    AddPosition(Vector3<f32>),
    ResetPosition(Vector3<f32>),

    SetAspect(f32),
    SetNear(f32),
    SetFar(f32),
    SetFov(f32),
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct CameraHandle {
    tx: mpsc::Sender<CameraEvent>,
}

#[wasm_bindgen]
impl CameraHandle {
    pub fn reset_angle(&self, x: f32, y: f32, z: f32) {
        self.tx
            .send(CameraEvent::ResetAngle(Vector3::new(x, y, z)))
            .unwrap();
    }
    pub fn add_angle(&self, x: f32, y: f32, z: f32) {
        self.tx
            .send(CameraEvent::AddAngle(Vector3::new(x, y, z)))
            .unwrap();
    }

    pub fn reset_position(&self, x: f32, y: f32, z: f32) {
        self.tx
            .send(CameraEvent::ResetPosition(Vector3::new(x, y, z)))
            .unwrap();
    }
    pub fn add_position(&self, x: f32, y: f32, z: f32) {
        self.tx
            .send(CameraEvent::AddPosition(Vector3::new(x, y, z)))
            .unwrap();
    }

    pub fn set_near(&self, near: f32) {
        self.tx.send(CameraEvent::SetNear(near)).unwrap();
    }
    pub fn set_far(&self, far: f32) {
        self.tx.send(CameraEvent::SetFar(far)).unwrap();
    }
    pub fn set_aspect(&self, aspect: f32) {
        self.tx.send(CameraEvent::SetAspect(aspect)).unwrap();
    }
    pub fn set_fov(&self, fov: f32) {
        self.tx.send(CameraEvent::SetFov(fov)).unwrap();
    }
}

pub struct Camera {
    near: f32,
    far: f32,
    aspect: f32,
    fov: f32,

    entity: Entity,

    world_view_projection_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,

    tx: mpsc::Sender<CameraEvent>,
    rx: mpsc::Receiver<CameraEvent>,
}

impl Camera {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let projection_matrix = perspective(Deg(90.0), 1.0, 0.2, 10000.0);

        Camera {
            near: 0.5,
            far: 10000.0,
            fov: 120.0,
            aspect: 1.0,
            world_view_projection_matrix: Matrix4::identity(),
            entity: Entity::default(),

            tx,
            rx,
            projection_matrix,
        }
    }


    pub fn handle_click(&self, x: f32, y: f32) -> (Vector3<f32>, Vector3<f32>) {
        let mat = self.world_view_projection_matrix.invert().unwrap();
        let coord_near = Vector4::new(x * self.near, y * self.near, -self.near, self.near);
        let coord_far = Vector4::new(x * self.far, y * self.far, self.far, self.far);

        let near = (mat * coord_near).truncate();
        let far = (mat * coord_far).truncate();

        (near, (far - near).normalize())
    }


    fn reset_projection(&mut self) {
        // self.projection_matrix = frustum(-0.5 * self.aspect, 0.5 * self.aspect, -0.5, 0.5, self.near, self.far);
        self.projection_matrix = perspective(Deg(45.0), self.aspect, self.near, self.far);
    }

    fn reset_world_view_projection_matrix(&mut self) {
        let view_matrix = self.world_matrix().invert().unwrap(); // Transform to be used to put vertex in view, without projection
        self.world_view_projection_matrix = self.projection_matrix * view_matrix;
    }

    fn world_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.entity.position().into()) * self.entity.mat_rotation()
    }

    pub fn handle(&self) -> CameraHandle {
        CameraHandle {
            tx: self.tx.clone(),
        }
    }

    pub fn update(&mut self) -> Option<()> {
        let mut reset_world = false;
        let mut reset_projection = false;
        loop {
            match self.rx.try_recv() {
                Ok(CameraEvent::AddAngle(delta)) => {
                    let delta: Vec3 = delta.into();
                    self.entity.set_rotation(self.entity.rotation() + delta);
                    reset_world = true;
                }
                Ok(CameraEvent::ResetAngle(angle)) => {
                    self.entity.set_rotation(angle.into());
                    reset_world = true;
                }
                Ok(CameraEvent::AddPosition(delta)) => {
                    let delta: Vector4<f32> = delta.extend(1.0);
                    let rotation = self.entity.mat_rotation();
                    let delta: Vec3 = (rotation * delta).truncate().into();
                    self.entity.set_position(self.entity.position() + delta);
                    reset_world = true;
                }
                Ok(CameraEvent::ResetPosition(Vector3 { x, y, z })) => {
                    self.entity.set_position(Vec3::new(x, y, z));
                    reset_world = true;
                }
                Ok(CameraEvent::SetAspect(aspect)) => {
                    self.aspect = aspect;
                    reset_projection = true;
                }
                Ok(CameraEvent::SetFar(far)) => {
                    self.far = far;
                    reset_projection = true;
                }
                Ok(CameraEvent::SetNear(near)) => {
                    self.near = near;
                    reset_projection = true;
                }
                Ok(CameraEvent::SetFov(fov)) => {
                    self.fov = fov;
                    reset_projection = true;
                }
                Err(mpsc::TryRecvError::Disconnected) => return None,
                Err(mpsc::TryRecvError::Empty) => break,
            }
        }

        if reset_projection {
            self.reset_projection();
        }

        if reset_world {
            self.reset_world_view_projection_matrix();
        }

        unsafe {
            let position = self.entity.position();
            let rotation = self.entity.rotation();
            set_info(
                position.x, position.y, position.z, rotation.x, rotation.y, rotation.z,
            );
        }

        Some(())
    }

    /// Let this entity be the camera, how should the world be transformedd to be in this view
    pub fn world_view_projection_matrix(&self) -> Matrix4<f32> {
        self.world_view_projection_matrix
    }
}
