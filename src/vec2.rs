use std::{
    f64::consts::PI,
    ops::{Mul, Sub},
};

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
        let radians = PI * (angle / 180.0);
        Self([radians.cos(), radians.sin()])
    }

    pub fn cross(&self, other: Self) -> f64 {
        self.x() * other.y() - self.y() * other.x()
    }

    pub fn normalized(&self) -> Self {
        1.0 / self.magnitude() * self.clone()
    }

    pub fn magnitude(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y()).sqrt()
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self([self.x() - rhs.x(), self.y() - rhs.y()])
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2([self * rhs.x(), self * rhs.y()])
    }
}
