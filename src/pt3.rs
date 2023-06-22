use crate::structure::{EuclideanSpace, VecSpace};

#[derive(Clone, Copy, Debug)]
pub struct Pt3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Pt3 {
    pub fn new(x: f32, y: f32, z: f32) -> Pt3 {
        Pt3 { x, y, z }
    }
}

impl VecSpace for Pt3 {}

impl EuclideanSpace<Pt3> for Pt3 {
    fn dot(&self, other: &Pt3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn project_on(&self, on: &Pt3) -> Pt3 {
        todo!();
    }

    fn reject_on(&self, on: &Pt3) -> Pt3 {
        todo!();
    }
}
