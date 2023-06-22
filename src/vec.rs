use std::ops;

use crate::{impl_algebraic_ops, impl_conversions, impl_index_ops, vec3::Vec3, vec4::Vec4};

impl_algebraic_ops!(Vec3 { x, y, z }, 3);
impl_algebraic_ops!(Vec4 { x, y, z, w }, 4);

impl_conversions!(Vec3 => [f32; 3], |v: &Vec3| {
    [v.x, v.y, v.z]
});
impl_conversions!([f32; 3] => Vec3, |a: &[f32; 3]| {
    Vec3::new(a[0], a[1], a[2])
});

impl_conversions!(Vec4 => [f32; 4], |v: &Vec4| {
    [v.x, v.y, v.z, v.w]
});
impl_conversions!([f32; 4] => Vec4, |a: &[f32; 4]| {
    Vec4::new(a[0], a[1], a[2], a[3])
});

impl_conversions!(Vec4 => Vec3, |v4: &Vec4| {
    Vec3::new(v4.x, v4.y, v4.z)
});

impl_index_ops!(Vec3 { 0 => x, 1 => y, 2 => z } => f32);
impl_index_ops!(Vec4 { 0 => x, 1 => y, 2 => z, 3 => w } => f32);
