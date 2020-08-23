use cgmath::{
    perspective,
    prelude::{SquareMatrix, Zero},
    Angle, Deg, Euler, Matrix4, Rad, Vector3,
};

static NEAR: f32 = 0.2;
static FAR: f32 = 2000.0;

pub struct Player {
    position: Vector3<f32>,
    speed: Vector3<f32>,
    ang_speed: Matrix4<f32>,
    rotation: Matrix4<f32>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            position: Vector3::new(0.0, 0.0, 0.0),
            speed: Vector3::zero(),
            ang_speed: Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)).into(),
            rotation: Matrix4::identity(),
        }
    }
}

impl Player {
    pub fn with_position(mut self, position: Vector3<f32>) -> Self {
        self.position = position;
        self
    }

    pub fn with_speed(mut self, speed: Vector3<f32>) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_rotation<A: Angle>(mut self, rotation: Matrix4<f32>) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_ang_speed<A: Angle<Unitless = f32> + Into<Rad<f32>>>(
        mut self,
        ang_speed: Euler<A>,
    ) -> Self {
        self.ang_speed = ang_speed.into();
        self
    }

    pub fn rotation(&self) -> &Matrix4<f32> {
        &self.rotation
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn speed(&self) -> &Vector3<f32> {
        &self.speed
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
        self.rotation = self.rotation * (dt * self.ang_speed);
    }

    #[inline]
    pub fn calc_transform(&self) -> Matrix4<f32> {
        self.rotation + Matrix4::from_translation(self.position)
    }

    pub fn calc_proj_matrix<A: Into<Rad<f32>>>(&self, fov: A, aspect: f32) -> Matrix4<f32> {
        let projection_matrix = perspective(fov, aspect, NEAR, FAR);
        let view_matrix = self.calc_transform().invert().unwrap();

        projection_matrix * view_matrix
    }
}
