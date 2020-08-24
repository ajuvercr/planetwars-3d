use crate::set_info;
use cgmath::{
    perspective,
    prelude::{SquareMatrix, Zero},
    Deg, Euler, Matrix4, Vector3,
};
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

    position: Vector3<f32>,
    rotation: Vector3<f32>,

    world_view_projection_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,

    tx: mpsc::Sender<CameraEvent>,
    rx: mpsc::Receiver<CameraEvent>,
}

impl Camera {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let projection_matrix = perspective(Deg(90.0), 1.0, 0.2, 2000.0);

        Camera {
            near: 0.2,
            far: 2000.0,
            fov: 90.0,
            aspect: 1.0,
            world_view_projection_matrix: Matrix4::identity(),
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            tx,
            rx,
            projection_matrix,
        }
    }

    fn reset_projection(&mut self) {
        self.projection_matrix = perspective(Deg(90.0), self.aspect, self.near, self.far);
    }

    fn reset_world_view_projection_matrix(&mut self) {
        let view_matrix = self.world_matrix().invert().unwrap(); // Transform to be used to put vertex in view, without projection
        self.world_view_projection_matrix = self.projection_matrix * view_matrix;
    }

    fn world_matrix(&self) -> Matrix4<f32> {
        let Vector3 { x, y, z } = self.rotation;
        let rot: Matrix4<_> = Euler::new(Deg(x), Deg(y), Deg(z)).into();
        Matrix4::from_translation(self.position) * rot
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
                    self.rotation += delta;
                    reset_world = true;
                }
                Ok(CameraEvent::ResetAngle(angle)) => {
                    self.rotation = angle;
                    reset_world = true;
                }
                Ok(CameraEvent::AddPosition(delta)) => {
                    self.position += (self.world_matrix() * delta.extend(0.0)).truncate();
                    reset_world = true;
                }
                Ok(CameraEvent::ResetPosition(position)) => {
                    self.position = position;
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

        // Rust analyser says it should be in unsafe and later that it shouldn't be in unsafe :/
        #[allow(unused_unsafe)]
        unsafe {
            set_info(
                self.position.x,
                self.position.y,
                self.position.z,
                self.rotation.x,
                self.rotation.y,
                self.rotation.z,
            );
        }

        Some(())
    }

    /// Let this entity be the camera, how should the world be transformedd to be in this view
    pub fn world_view_projection_matrix(&self) -> Matrix4<f32> {
        self.world_view_projection_matrix
    }
}
