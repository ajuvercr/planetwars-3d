use cgmath::{prelude::Zero, Angle, Deg, Euler, Matrix4, Vector3};

pub struct Entity {
    position: Vector3<f32>,
    speed: Vector3<f32>,
    rotation: Vector3<f32>,
    ang_speed: Vector3<f32>,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            position: Vector3::zero(),
            speed: Vector3::zero(),
            ang_speed: Vector3::zero(),
            rotation: Vector3::zero(),
        }
    }
}

impl Entity {
    pub fn with_position(mut self, position: Vector3<f32>) -> Self {
        self.position = position;
        self
    }

    pub fn with_speed(mut self, speed: Vector3<f32>) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_rotation<A: Angle>(mut self, rotation: Vector3<f32>) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_ang_speed(mut self, ang_speed: Vector3<f32>) -> Self {
        self.ang_speed = ang_speed;
        self
    }

    pub fn rotation(&self) -> Matrix4<f32> {
        let Vector3 { x, y, z } = self.rotation;
        Euler::new(Deg(x), Deg(y), Deg(z)).into()
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn speed(&self) -> &Vector3<f32> {
        &self.speed
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
        self.rotation += self.ang_speed * dt;
    }

    /// Matrix to transform vertices to the correct location in the world
    #[inline]
    pub fn world_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) * self.rotation()
    }
}
