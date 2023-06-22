use crate::{impl_algebraic_ops, impl_conversions, impl_index_ops, pt3::Pt3, vec3::Vec3};
use std::ops;

impl_algebraic_ops!(Pt3 { x, y, z }, 3);
impl_algebraic_ops!(Pt3 : Vec3 { x, y, z }, 3);

impl_conversions!(Pt3 => [f32; 3], |p: &Pt3| {
    [p.x, p.y, p.z]
});

impl_conversions!([f32; 3] => Pt3, |a: &[f32; 3]| {
    Pt3::new(a[0], a[1], a[2])
});

impl_conversions!(Pt3 => Vec3, |p: &Pt3| {
    Vec3::new(p.x, p.y, p.z)
});

impl_conversions!(Vec3 => Pt3, |v: &Vec3| {
    Pt3::new(v.x, v.y, v.z)
});

impl_index_ops!(Pt3 { 0 => x, 1 => y, 2 => z } => f32);
