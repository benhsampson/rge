use crate::{pt3::Pt3, structure::EuclideanSpace, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub d: f32,
}

impl Plane {
    pub fn new(x: f32, y: f32, z: f32, d: f32) -> Plane {
        Plane { x, y, z, d }
    }

    pub fn normal(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl EuclideanSpace<Vec3> for Plane {
    fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl EuclideanSpace<Pt3> for Plane {
    fn dot(&self, p: &Pt3) -> f32 {
        self.x * p.x + self.y * p.y + self.z * p.z + self.d
    }
}
