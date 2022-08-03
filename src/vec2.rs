use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2(pub [f64; 2]);

impl Vec2 {
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    /// Returns a unit vector with the specified angle
    pub fn from_angle(angle: f64) -> Self {
        let radians = (PI / 180.0) * angle;
        Self([radians.cos(), radians.sin()])
    }

    pub fn cross(&self, other: Self) -> f64 {
        self.x() * other.y() - self.y() * other.x()
    }
}
