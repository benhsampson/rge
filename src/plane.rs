use std::ops;

use crate::{pt3::Pt3, structure::EuclideanSpace, transform4::Transform4, vec3::Vec3};

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

    fn project_on(&self, on: &Vec3) -> Vec3 {
        todo!();
    }

    fn reject_on(&self, on: &Vec3) -> Vec3 {
        todo!();
    }
}

impl EuclideanSpace<Pt3> for Plane {
    fn dot(&self, p: &Pt3) -> f32 {
        self.x * p.x + self.y * p.y + self.z * p.z + self.d
    }

    fn project_on(&self, on: &Pt3) -> Pt3 {
        todo!();
    }

    fn reject_on(&self, on: &Pt3) -> Pt3 {
        todo!();
    }
}

impl ops::Mul<Transform4> for Plane {
    type Output = Plane;

    fn mul(self, H: Transform4) -> Self::Output {
        let Plane { x, y, z, d } = self;
        Plane::new(
            x * H[0][0] + y * H[1][0] + z * H[2][0],
            x * H[0][1] + y * H[1][1] + z * H[2][1],
            x * H[0][2] + y * H[1][2] + z * H[2][2],
            x * H[0][3] + y * H[1][3] + z * H[2][3] + d,
        )
    }
}
