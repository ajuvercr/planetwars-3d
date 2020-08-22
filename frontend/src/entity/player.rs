
use cgmath::{Point3, prelude::{Zero, SquareMatrix}, Vector3, Matrix4};

static NEAR: f32 = 0.2;
static FAR: f32 = 2000.0;

pub struct Player {
    position: Point3<f32>,
    speed: Vector3<f32>,
    ang_speed: Vector3<f32>,
    rotation: Matrix4<f32>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            position: Point3::new(0.0, 0.0, 0.0),
            speed: Vector3::zero(),
            ang_speed: Vector3::zero(),
            rotation: Matrix4::identity(),
        }
    }
}

impl Player {
    pub fn with_position(mut self, position: Point3<f32>) -> Self {
        self.position = position;
        self
    }

    pub fn with_speed(mut self, speed: Vector3<f32>) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_rotation(mut self, rotation: Matrix4<f32>) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn rotation(&self) -> &Matrix4<f32> {
        &self.rotation
    }

    pub fn position(&self) -> &Point3<f32> {
        &self.position
    }

    pub fn speed(&self) -> &Vector3<f32> {
        &self.speed
    }
}