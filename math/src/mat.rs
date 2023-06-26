use std::ops;

use crate::{
    impl_algebraic_ops, impl_conversions, impl_index_ops, mat3::Mat3, mat4::Mat4, vec3::Vec3,
    vec4::Vec4,
};

impl_algebraic_ops!(Mat3 { a, b, c }, Vec3);
impl_algebraic_ops!(Mat4 { a, b, c, d }, Vec4);

impl_conversions!(Mat3 => [[f32; 3]; 3], |m: &Mat3| {
    [m.a.into(), m.b.into(), m.c.into()]
});
impl_conversions!([[f32; 3]; 3] => Mat3, |a: &[[f32; 3]; 3]| {
    Mat3::from_columns(a[0].into(), a[1].into(), a[2].into())
});
impl_conversions!(Mat4 => [[f32; 4]; 4], |m: &Mat4| {
    [m.a.into(), m.b.into(), m.c.into(), m.d.into()]
});
impl_conversions!([[f32; 4]; 4] => Mat4, |a: &[[f32; 4]; 4]| {
    Mat4::from_columns(a[0].into(), a[1].into(), a[2].into(), a[3].into())
});

impl_index_ops!(Mat3 { 0 => a, 1 => b, 2 => c } => Vec3);
impl_index_ops!(Mat4 { 0 => a, 1 => b, 2 => c, 3 => d } => Vec4);
