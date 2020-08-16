use super::AlmostEqual;

#[derive(Clone, Debug, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<
        T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
    > Vector2<T>
{
    pub fn dist2(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn norm2(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl Vector2<f32> {
    pub fn dist(&self, other: &Self) -> f32 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

impl Vector2<f64> {
    pub fn dist(&self, other: &Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

impl<T: AlmostEqual> AlmostEqual for Vector2<T> {
    fn almost_equal(&self, b: &Self) -> bool {
        self.x.almost_equal(&b.x) && self.y.almost_equal(&b.y)
    }
}
