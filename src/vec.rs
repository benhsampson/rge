use crate::{vec3::Vec3, vec4::Vec4};
use std::ops;

macro_rules! impl_array_conversions {
    ($Container:ident { $($field:ident : $index:expr),+ }, $n:expr) => {
        impl From<[f32; $n]> for $Container {
            fn from(a: [f32; $n]) -> Self {
                Self { $($field: a[$index]),+ }
            }
        }

        impl From<&[f32; $n]> for $Container {
            fn from(a: &[f32; $n]) -> Self {
                (*a).into()
            }
        }

        impl From<&mut [f32; $n]> for $Container {
            fn from(a: &mut [f32; $n]) -> Self {
                (*a).into()
            }
        }

        impl From<$Container> for [f32; $n] {
            fn from(c: $Container) -> Self {
                match c {
                    $Container { $($field),+ } => [$($field),+],
                }
            }
        }

        impl From<&$Container> for [f32; $n] {
            fn from(c: &$Container) -> Self {
                match c {
                    $Container { $($field),+ } => [$(*$field),+],
                }
            }
        }

        impl From<&mut $Container> for [f32; $n] {
            fn from(c: &mut $Container) -> Self {
                match c {
                    $Container { $($field),+ } => [$(*$field),+],
                }
            }
        }
    };
}

macro_rules! impl_index_operators {
    ($Container:ident { $($field:ident: $index:expr),+ }) => {
        impl ops::Index<usize> for $Container {
            type Output = f32;

            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    $($index => &self.$field),+,
                    _ => panic!("Index out of bounds for {}", stringify!($Container)),
                }
            }
        }

        impl ops::IndexMut<usize> for $Container {
            fn index_mut(&mut self, i: usize) -> &mut Self::Output {
                match i {
                    $($index => &mut self.$field),+,
                    _ => panic!("Index out of bounds for {}", stringify!($Container)),
                }
            }
        }
    };
}

macro_rules! impl_algebraic_operators {
    ($VecN:ident { $($field:ident),+ }, $n:expr) => {
        impl ops::Neg for $VecN {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self * -1.
            }
        }

        impl ops::Add<Self> for $VecN {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new($(self.$field + rhs.$field),+)
            }
        }

        impl ops::AddAssign<Self> for $VecN {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl ops::Sub<Self> for $VecN {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::new($(self.$field - rhs.$field),+)
            }
        }

        impl ops::SubAssign<Self> for $VecN {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl ops::Mul<f32> for $VecN {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::new($(self.$field * rhs),+)
            }
        }

        impl ops::MulAssign<f32> for $VecN {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        impl ops::Div<f32> for $VecN {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                let inv = 1. / rhs;
                Self::new($(self.$field * inv),+)
            }
        }

        impl ops::DivAssign<f32> for $VecN {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }
    };
}

impl_algebraic_operators!(Vec3 { x, y, z }, 3);
impl_algebraic_operators!(Vec4 { x, y, z, w }, 4);

impl_array_conversions!(Vec3 { x: 0, y: 1, z: 2 }, 3);
impl_array_conversions!(
    Vec4 {
        x: 0,
        y: 1,
        z: 2,
        w: 3
    },
    4
);

impl_index_operators!(Vec3 { x: 0, y: 1, z: 2 });
impl_index_operators!(Vec4 {
    x: 0,
    y: 1,
    z: 2,
    w: 3
});
