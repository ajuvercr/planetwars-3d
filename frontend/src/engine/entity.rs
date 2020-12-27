use cgmath::{Angle, Deg, Euler, Matrix4, Vector3, InnerSpace};
use pw_derive::Settings;
use serde::{Deserialize, Serialize};

// was settings
#[derive(Debug, Clone, Serialize, Deserialize, AddGetterVal, AddSetter, Settings)]
pub struct Entity {
    #[get_val]
    #[set]
    position: Vec3,
    #[get_val]
    #[set]
    speed: Vec3,
    #[get_val]
    #[set]
    rotation: Vec3,
    #[get_val]
    #[set]
    ang_speed: Vec3,

    #[get_val]
    #[set]
    scale: Vec3,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            position: Vec3::zero(),
            speed: Vec3::zero(),
            ang_speed: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::one(),
        }
    }
}

impl Entity {
    pub fn is_hit(&self, origin: Vector3<f32>, direction: Vector3<f32>) -> bool {
        let pos: Vector3<f32> = self.position.into();
        let scale_max = self.scale.max();

        let o_min_c = origin - pos;
        let big_d = cgmath::dot(direction, o_min_c).powi(2) - (o_min_c.magnitude2() - scale_max.powi(2));
        if big_d < 0.0 {
            return false;
        }

        let big_d_sqrt = big_d.sqrt();
        let distance = - cgmath::dot(direction, o_min_c) + big_d_sqrt;

        distance > 0.0
    }

    pub fn with_position(mut self, position: Vector3<f32>) -> Self {
        self.position = position.into();
        self
    }

    pub fn with_speed(mut self, speed: Vector3<f32>) -> Self {
        self.speed = speed.into();
        self
    }

    pub fn with_rotation<A: Angle>(mut self, rotation: Vector3<f32>) -> Self {
        self.rotation = rotation.into();
        self
    }

    pub fn with_ang_speed(mut self, ang_speed: Vector3<f32>) -> Self {
        self.ang_speed = ang_speed.into();
        self
    }

    pub fn with_hom_scale(mut self, s: f32) -> Self {
        self.scale = Vec3::new(s, s, s);
        self
    }

    pub fn with_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.scale = Vec3::new(x, y, z);
        self
    }

    pub fn mat_rotation(&self) -> Matrix4<f32> {
        let Vec3 { x, y, z } = self.rotation;
        Euler::new(Deg(x), Deg(y), Deg(z)).into()
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
        self.rotation += self.ang_speed * dt;
    }

    /// Matrix to transform vertices to the correct location in the world
    #[inline]
    pub fn world_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position.into())
            * self.mat_rotation()
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }
}

pub use vec3::Vec3;
mod vec3 {
    use cgmath::Vector3;

    use std::ops::{Add, AddAssign, Mul, MulAssign};

    use pw_derive::Settings;
    use serde::{Deserialize, Serialize};

    // was settings
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, Settings)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl Vec3 {
        #[inline]
        pub fn zero() -> Self {
            Self::new(0.0, 0.0, 0.0)
        }

        #[inline]
        pub fn one() -> Self {
            Self::new(1.0, 1.0, 1.0)
        }

        #[inline]
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }

        pub fn max(&self) -> f32 {
            self.x.max(self.y.max(self.z))
        }
    }

    impl Mul<f32> for Vec3 {
        // The multiplication of rational numbers is a closed operation.
        type Output = Self;
        fn mul(mut self, d: f32) -> Self {
            self.x *= d;
            self.y *= d;
            self.z *= d;
            self
        }
    }

    impl MulAssign<f32> for Vec3 {
        // The multiplication of rational numbers is a closed operation.
        fn mul_assign(&mut self, d: f32) {
            self.x *= d;
            self.y *= d;
            self.z *= d;
        }
    }

    impl Add<&Vec3> for Vec3 {
        // The multiplication of rational numbers is a closed operation.
        type Output = Self;
        fn add(mut self, rhs: &Vec3) -> Self {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
            self
        }
    }

    impl Add<Vec3> for Vec3 {
        // The multiplication of rational numbers is a closed operation.
        type Output = Self;
        fn add(mut self, rhs: Vec3) -> Self {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
            self
        }
    }

    impl AddAssign<&Vec3> for Vec3 {
        // The multiplication of rational numbers is a closed operation.
        fn add_assign(&mut self, rhs: &Vec3) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }

    impl AddAssign<Vec3> for Vec3 {
        // The multiplication of rational numbers is a closed operation.
        fn add_assign(&mut self, rhs: Vec3) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }

    impl From<Vector3<f32>> for Vec3 {
        fn from(vec: Vector3<f32>) -> Self {
            Self {
                x: vec.x,
                y: vec.y,
                z: vec.z,
            }
        }
    }

    impl Into<Vector3<f32>> for Vec3 {
        fn into(self) -> Vector3<f32> {
            Vector3::new(self.x, self.y, self.z)
        }
    }
}
