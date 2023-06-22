use crate::{
    impl_op,
    structure::{EuclideanSpace, Mat, SquareMat},
    vec3::Vec3,
    vec4::Vec4,
};
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub a: Vec4,
    pub b: Vec4,
    pub c: Vec4,
    pub d: Vec4,
}

impl Mat4 {
    pub fn from_columns(a: Vec4, b: Vec4, c: Vec4, d: Vec4) -> Self {
        Self { a, b, c, d }
    }
}

#[macro_export]
macro_rules! impl_mat4 {
    ($M:ty { $($field:ident),+ }) => {
        impl Mat for $M {
            type Row = Vec4;
            type Column = Vec4;
            type Transpose = $M;

            const ZERO: Self = Self {
                $($field: Vec4::ZERO),+
            };

            fn transpose(&self) -> Self::Transpose {
                let m = self;
                [
                    [m[0][0], m[1][0], m[2][0], m[3][0]],
                    [m[0][1], m[1][1], m[2][1], m[3][1]],
                    [m[0][2], m[1][2], m[2][2], m[3][2]],
                    [m[0][3], m[1][3], m[2][3], m[3][3]],
                ]
                .into()
            }
        }

        impl std::ops::Mul<Vec4> for $M {
            type Output = Vec4;

            fn mul(self, rhs: Vec4) -> Self::Output {
                Vec4::new(
                    $(rhs.dot(&self.$field),)+
                )
            }
        }
    };
}

impl_mat4!(Mat4 { a, b, c, d });

impl SquareMat for Mat4 {
    type RowColumn = Vec4;

    const IDENTITY: Self = Self {
        a: Vec4::X,
        b: Vec4::Y,
        c: Vec4::Z,
        d: Vec4::W,
    };

    fn from_diagonal(d: Self::RowColumn) -> Self {
        Self {
            a: [d.x, 0., 0., 0.].into(),
            b: [0., d.y, 0., 0.].into(),
            c: [0., 0., d.z, 0.].into(),
            d: [0., 0., 0., d.w].into(),
        }
    }

    fn determinant(&self) -> f32 {
        todo!()
    }

    fn invert(&self) -> Option<Self> {
        let a: Vec3 = self.a.into();
        let b: Vec3 = self.b.into();
        let c: Vec3 = self.c.into();
        let d: Vec3 = self.d.into();

        let x = self[0][3];
        let y = self[1][3];
        let z = self[2][3];
        let w = self[3][3];

        let mut s = a.cross(&b);
        let mut t = c.cross(&d);
        let mut u = a * y - b * x;
        let mut v = c * w - d * z;

        let det = s.dot(&v) + t.dot(&u);

        if det != 0. {
            let inv_det = 1. / det;
            s *= inv_det;
            t *= inv_det;
            u *= inv_det;
            v *= inv_det;

            let r0 = b.cross(&v) + t * y;
            let r1 = v.cross(&a) - t * x;
            let r2 = d.cross(&u) + s * w;
            let r3 = u.cross(&c) - s * z;

            Some(Self::from_columns(
                [r0.x, r0.y, r0.z, -b.dot(&t)].into(),
                [r1.x, r1.y, r1.z, a.dot(&t)].into(),
                [r2.x, r2.y, r2.z, -d.dot(&s)].into(),
                [r3.x, r3.y, r3.z, c.dot(&s)].into(),
            ))
        } else {
            None
        }
    }
}

impl_op!(Mat4 : Mat4, ops::Mul { fn mul |a: &Mat4, b: &Mat4| {
    let mut m = Mat4::ZERO;
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                m[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    m
}});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_multiplication_works() {
        let m = Mat4::from([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);
        assert_eq!(
            m * m.transpose(),
            [
                [30., 70., 110., 150.],
                [70., 174., 278., 382.],
                [110., 278., 446., 614.],
                [150., 382., 614., 846.]
            ]
            .into()
        );
    }

    #[test]
    fn transpose_works() {
        let m: Mat4 = [
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]
        .into();
        assert_eq!(
            m.transpose(),
            [
                [1., 5., 9., 13.],
                [2., 6., 10., 14.],
                [3., 7., 11., 15.],
                [4., 8., 12., 16.]
            ]
            .into()
        )
    }

    #[test]
    fn test_inverse() {
        let mat4: Mat4 = [
            [6., 1., 2., 4.],
            [4., 3., 3., 2.],
            [2., 5., 5., 6.],
            [8., 7., 6., 3.],
        ]
        .into();
        let inv = mat4.invert().unwrap();
        let expected: Mat4 = [
            [0.16, 0.28, -0.68, 0.28],
            [-0.26, -3.08, 4.48, -1.08],
            [-0.1, 0.2, -0.2, 0.2],
            [0.16, 1.28, -1.68, 0.28],
        ]
        .into();
        assert_eq!(inv, expected);
    }
}
