use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.dot(*self))
    }

    pub const fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub const fn cross(&self, rhs: Self) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub const fn quarter_counterclockwise(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub const fn quarter_clockwise(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
