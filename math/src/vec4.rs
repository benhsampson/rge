use crate::{
    structure::{EuclideanSpace, VecSpace},
    vec3::Vec3,
};

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };

    pub const X: Self = Self {
        x: 1.,
        y: 0.,
        z: 0.,
        w: 0.,
    };

    pub const Y: Self = Self {
        x: 0.,
        y: 1.,
        z: 0.,
        w: 0.,
    };

    pub const Z: Self = Self {
        x: 0.,
        y: 0.,
        z: 1.,
        w: 0.,
    };

    pub const W: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 1.,
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl VecSpace for Vec4 {}

impl EuclideanSpace<Self> for Vec4 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}
