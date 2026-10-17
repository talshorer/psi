use std::ops::{Add, AddAssign, Deref, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Vec2 {
    pub(crate) x: f64,
    pub(crate) y: f64,
}
impl Vec2 {
    pub(crate) fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct NormalizedVec2(Vec2);

impl NormalizedVec2 {
    pub(crate) const fn const_new(v: Vec2) -> Option<Self> {
        if (v.x * v.x + v.y * v.y) == 1.0 {
            Some(Self(v))
        } else {
            None
        }
    }
}

impl From<Vec2> for NormalizedVec2 {
    fn from(v: Vec2) -> Self {
        let scale = (v.x * v.x + v.y * v.y).sqrt();
        Self((1.0 / scale) * v)
    }
}

impl Deref for NormalizedVec2 {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
