use rge_math::{impl_algebraic_ops, impl_op, impl_op_assign};
use std::ops;

#[derive(Clone, Copy)]
struct ColorRGB {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl ColorRGB {
    fn new(r: f32, g: f32, b: f32, a: Option<f32>) -> Self {
        Self {
            r,
            g,
            b,
            a: a.unwrap_or(1.),
        }
    }
}

impl_algebraic_ops!(ColorRGB { r, g, b, a }, 4);

impl_op!(ColorRGB : ColorRGB, ops::Mul { fn mul |lhs: &ColorRGB, rhs: &ColorRGB| {
    ColorRGB::new(lhs.r * rhs.r, lhs.g * rhs.g, lhs.b * rhs.b, Some(lhs.a * rhs.a))
}});

impl_op_assign!(ColorRGB, ColorRGB, ops::MulAssign { fn mul_assign |lhs: &mut ColorRGB, rhs: &ColorRGB| {
    *lhs = *lhs * rhs;
}});
