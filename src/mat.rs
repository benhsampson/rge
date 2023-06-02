use std::ops;

use crate::{mat3::Mat3, mat4::Mat4, vec3::Vec3, vec4::Vec4};

macro_rules! impl_array_conversions {
    ($MatN:ident { $($field:ident : $index:expr),+ }, $n: expr) => {
        impl From<[[f32; $n]; $n]> for $MatN {
            fn from(m: [[f32; $n]; $n]) -> Self {
                Self { $($field: m[$index].into()),+ }
            }
        }

        impl From<&[[f32; $n]; $n]> for $MatN {
            fn from(m: &[[f32; $n]; $n]) -> Self {
                Self { $($field: m[$index].into()),+ }
            }
        }

        impl From<&mut [[f32; $n]; $n]> for $MatN {
            fn from(m: &mut [[f32; $n]; $n]) -> Self {
                Self { $($field: m[$index].into()),+ }
            }
        }

        impl From<$MatN> for [[f32; $n]; $n] {
            fn from(m: $MatN) -> Self {
                match m {
                    $MatN { $($field),+ } => [$($field.into()),+]
                }
            }
        }

        impl From<&$MatN> for [[f32; $n]; $n] {
            fn from(m: &$MatN) -> Self {
                match m {
                    $MatN { $($field),+ } => [$($field.into()),+]
                }
            }
        }

        impl From<&mut $MatN> for [[f32; $n]; $n] {
            fn from(m: &mut $MatN) -> Self {
                match m {
                    $MatN { $($field),+ } => [$($field.into()),+]
                }
            }
        }
    }
}

macro_rules! impl_index_operators {
    ($MatN:ident { $($field:ident : $index:expr),+ }, $O:ty) => {
        impl ops::Index<usize> for $MatN {
            type Output = $O;

            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    $($index => &self.$field),+,
                    _ => panic!("Index out of bounds for {}", stringify!($MatN)),
                }
            }
        }

        impl ops::IndexMut<usize> for $MatN {
            fn index_mut(&mut self, i: usize) -> &mut Self::Output {
                match i {
                    $($index => &mut self.$field),+,
                    _ => panic!("Index out of bounds for {}", stringify!($MatN)),
                }
            }
        }
    };
}

macro_rules! impl_algebraic_operators {
    ($MatN:ident { $($field:ident),+ }, $VecN:ident) => {
        impl ops::Neg for $MatN {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self * -1.
            }
        }

        impl ops::Add<Self> for $MatN {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self { $($field: self.$field + rhs.$field),+ }
            }
        }

        impl ops::AddAssign<Self> for $MatN {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl ops::Sub<Self> for $MatN {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self { $($field: self.$field - rhs.$field),+ }
            }
        }

        impl ops::SubAssign<Self> for $MatN {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl ops::Mul<f32> for $MatN {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self { $($field: self.$field * rhs),+ }
            }
        }

        impl ops::MulAssign<f32> for $MatN {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        impl ops::Div<f32> for $MatN {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                let inv = 1. / rhs;
                Self { $($field: self.$field * inv),+ }
            }
        }

        impl ops::DivAssign<f32> for $MatN {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }
    };
}

impl_algebraic_operators!(Mat3 { a, b, c }, Vec3);
impl_algebraic_operators!(Mat4 { a, b, c, d }, Vec4);

impl_array_conversions!(Mat3 { a: 0, b: 1, c: 2 }, 3);
impl_array_conversions!(
    Mat4 {
        a: 0,
        b: 1,
        c: 2,
        d: 3
    },
    4
);

impl_index_operators!(Mat3 { a: 0, b: 1, c: 2 }, Vec3);
impl_index_operators!(
    Mat4 {
        a: 0,
        b: 1,
        c: 2,
        d: 3
    },
    Vec4
);
