use std::ops;

use crate::{impl_algebraic_ops, impl_conversions, impl_index_ops, vec3::Vec3, vec4::Vec4};

impl_algebraic_ops!(Vec3 { x, y, z }, 3);
impl_algebraic_ops!(Vec4 { x, y, z, w }, 4);

impl_conversions!(Vec3 => [f32; 3], |from: &Vec3| {
    [from.x, from.y, from.z]
});
impl_conversions!([f32; 3] => Vec3, |from: &[f32; 3]| {
    Vec3::new(from[0], from[1], from[2])
});

impl_conversions!(Vec4 => [f32; 4], |from: &Vec4| {
    [from.x, from.y, from.z, from.w]
});
impl_conversions!([f32; 4] => Vec4, |from: &[f32; 4]| {
    Vec4::new(from[0], from[1], from[2], from[3])
});

impl_index_ops!(Vec3 { 0 => x, 1 => y, 2 => z } => f32);
impl_index_ops!(Vec4 { 0 => x, 1 => y, 2 => z, 3 => w } => f32);
