use std::ops;

use crate::{
    structure::{Mat, SquareMat, VecSpace},
    vec3::Vec3,
};

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Mat3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        c0r0: f32,
        c0r1: f32,
        c0r2: f32,
        c1r0: f32,
        c1r1: f32,
        c1r2: f32,
        c2r0: f32,
        c2r1: f32,
        c2r2: f32,
    ) -> Self {
        Self::from_columns(
            [c0r0, c0r1, c0r2].into(),
            [c1r0, c1r1, c1r2].into(),
            [c2r0, c2r1, c2r2].into(),
        )
    }

    pub fn from_columns(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }

    pub fn from_angle_x(t: f32) -> Self {
        let (s, c) = t.sin_cos();
        Self::new(
            1., 0., 0., //
            0., c, -s, //
            0., s, c,
        )
    }

    pub fn from_angle_y(t: f32) -> Self {
        let (s, c) = t.sin_cos();
        Self::new(
            c, 0., s, //
            0., 1., 0., //
            -s, 0., c,
        )
    }

    pub fn from_angle_z(t: f32) -> Self {
        let (s, c) = t.sin_cos();
        Self::new(
            c, -s, 0., //
            s, c, 0., //
            0., 0., 1.,
        )
    }

    pub fn from_axis_angle(a: Vec3, t: f32) -> Self {
        // Warning: `a` has to be normalized
        // this will not warn to maximize performance
        let (s, c) = t.sin_cos();
        let Vec3 {
            x: dx,
            y: dy,
            z: dz,
        } = a * (1. - c);
        let dxy = dx * a.y;
        let dxz = dx * a.z;
        let dyz = dy * a.z;
        [
            [c + dx * a.x, dxy - s * a.z, dxz + s * a.y],
            [dxy + s * a.z, c + dy * a.y, dyz - s * a.x],
            [dxz - s * a.y, dyz + s * a.x, c + dz * a.z],
        ]
        .into()
    }

    pub fn make_reflection(a: Vec3) -> Self {
        // Warning: `a` has to be normalized
        let [dx, dy, dz]: [f32; 3] = (a * -2.).into();
        let dxy = dx * a.y;
        let dxz = dx * a.z;
        let dyz = dy * a.z;
        [
            [1. + dx * a.x, dxy, dxz],
            [dxy, 1. + dy * a.y, dyz],
            [dxz, dyz, 1. + dz * a.z],
        ]
        .into()
    }

    pub fn make_involution(a: Vec3) -> Self {
        // Warning: `a` has to be normalized
        let [dx, dy, dz]: [f32; 3] = (a * 2.).into();
        let dxy = dx * a.y;
        let dxz = dx * a.z;
        let dyz = dy * a.z;
        [
            [dx * a.x - 1., dxy, dxz],
            [dxy, dy * a.y - 1., dyz],
            [dxz, dyz, dz * a.z - 1.],
        ]
        .into()
    }

    pub fn make_scale(s: Vec3) -> Self {
        Self::from_diagonal(s)
    }

    pub fn make_scale_along(s: f32, a: Vec3) -> Self {
        // Warning: `a` has to be normalized
        let s = s - 1.;
        let [sx, sy, sz]: [f32; 3] = (a * s).into();
        let sxy = sx * a.y;
        let sxz = sx * a.z;
        let syz = sy * a.z;
        [
            [sx * a.x + 1., sxy, sxz],
            [sxy, sy * a.y + 1., syz],
            [sxz, syz, sz * a.z + 1.],
        ]
        .into()
    }
}

impl Mat for Mat3 {
    type Row = Vec3;
    type Column = Vec3;
    type Transpose = Mat3;

    const ZERO: Self = Self {
        a: Vec3::ZERO,
        b: Vec3::ZERO,
        c: Vec3::ZERO,
    };

    fn transpose(&self) -> Self {
        let m = self;
        Self::new(
            m[0][0], m[1][0], m[2][0], //
            m[0][1], m[1][1], m[2][1], //
            m[0][2], m[1][2], m[2][2],
        )
    }
}

impl SquareMat for Mat3 {
    type RowColumn = Vec3;

    const IDENTITY: Self = Self {
        a: Vec3::X,
        b: Vec3::Y,
        c: Vec3::Z,
    };

    fn from_diagonal(d: Self::RowColumn) -> Self {
        Self::new(
            d.x, 0., 0., //
            0., d.y, 0., //
            0., 0., d.z,
        )
    }

    fn determinant(&self) -> f32 {
        let m = self;
        m[0][0] * (m[1][1] * m[2][2] - m[2][1] * m[1][2])
            + m[1][0] * (m[2][1] * m[0][2] - m[0][1] * m[2][2])
            + m[2][0] * (m[0][1] * m[1][2] - m[1][1] * m[0][2])
    }

    fn invert(&self) -> Option<Self> {
        let Self { a, b, c } = self;

        let r0 = b.cross(c);
        let r1 = c.cross(a);
        let r2 = a.cross(b);

        let det = r2.dot(c);

        if det != 0. {
            let inv_det = 1. / r2.dot(c);
            Some(Self::from_columns(r0 * inv_det, r1 * inv_det, r2 * inv_det))
        } else {
            None
        }
    }
}

impl ops::Mul<Mat3> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let mut m = Self::ZERO;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    m[i][j] += self[k][j] * rhs[i][k];
                }
            }
        }
        m
    }
}

impl ops::MulAssign<Mat3> for Mat3 {
    fn mul_assign(&mut self, rhs: Mat3) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.dot(&self.a), rhs.dot(&self.b), rhs.dot(&self.c))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        structure::{Mat, SquareMat, VecSpace},
        vec3::Vec3,
    };

    use super::Mat3;

    #[test]
    fn new_works() {
        let m = Mat3::new(
            1., 2., 3., //
            4., 5., 6., //
            7., 8., 9.,
        );
        assert_eq!(m[0], [1., 2., 3.].into());
        assert_eq!(m[1], [4., 5., 6.].into());
        assert_eq!(m[2], [7., 8., 9.].into());
    }

    #[test]
    fn from_cols_works() {
        let m = Mat3::from_columns(
            [1., 2., 3.].into(), //
            [4., 5., 6.].into(), //
            [7., 8., 9.].into(),
        );
        assert_eq!(m[0], [1., 2., 3.].into());
        assert_eq!(m[1], [4., 5., 6.].into());
        assert_eq!(m[2], [7., 8., 9.].into());
    }

    #[test]
    fn converts() {
        let mut m = Mat3::new(
            1., 2., 3., //
            4., 5., 6., //
            7., 8., 9.,
        );
        let mut a = [[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];
        assert_eq!(m, a.into());
        assert_eq!(m, (&a).into());
        assert_eq!(m, (&mut a).into());

        let mb = &m;
        assert_eq!(a, <&Mat3 as Into<[[f32; 3]; 3]>>::into(mb));
        let mb = &mut m;
        assert_eq!(a, <&Mat3 as Into<[[f32; 3]; 3]>>::into(mb));
        assert_eq!(a, <Mat3 as Into<[[f32; 3]; 3]>>::into(m));
    }

    #[test]
    fn indexes() {
        let m = Mat3::new(
            1., 2., 3., //
            4., 5., 6., //
            7., 8., 9.,
        );
        assert_eq!(m[0][0], 1.);
        assert_eq!(m[0][1], 2.);
        assert_eq!(m[0][2], 3.);
        assert_eq!(m[1][0], 4.);
        assert_eq!(m[1][1], 5.);
        assert_eq!(m[1][2], 6.);
        assert_eq!(m[2][0], 7.);
        assert_eq!(m[2][1], 8.);
        assert_eq!(m[2][2], 9.);
    }

    #[test]
    fn matrix_multiplication_works() {
        let m = Mat3::new(
            1., 2., 3., //
            4., 5., 6., //
            7., 8., 9.,
        );
        let m2 = m * m;
        assert_eq!(
            m2,
            Mat3::new(
                30., 36., 42., //
                66., 81., 96., //
                102., 126., 150.,
            )
        )
    }

    #[test]
    fn transposes() {
        let m = Mat3::new(
            1., 2., 3., //
            4., 5., 6., //
            7., 8., 9.,
        );
        assert_eq!(
            m.transpose(),
            [
                [1., 4., 7.], //
                [2., 5., 8.], //
                [3., 6., 9.],
            ]
            .into()
        );
    }

    #[test]
    fn calculates_determinant() {
        let m = Mat3::new(
            1., 2., 3., //
            4., 5., 6., //
            7., 8., 9.,
        );
        assert_eq!(m.determinant(), 0.);
    }

    #[test]
    fn calculates_inverse() {
        let m = Mat3::new(
            2., 0., -1., //
            5., 1., 0., //
            0., 1., 3.,
        );
        assert_eq!(
            m.invert().unwrap(),
            [
                [3., -15., 5.], //
                [-1., 6., -2.], //
                [1., -5., 2.]
            ]
            .into()
        );
    }

    #[test]
    fn make_rotation_x_works() {
        let mat3 = Mat3::from_angle_x(0.5);
        assert_eq!(
            mat3,
            [
                [1., 0., 0.],
                [0., 0.87758255, -0.47942555],
                [0., 0.47942555, 0.87758255]
            ]
            .into()
        );
    }

    #[test]
    fn make_rotation_y_works() {
        let mat3 = Mat3::from_angle_y(0.5);
        assert_eq!(
            mat3,
            [
                [0.87758255, 0., 0.47942555],
                [0., 1., 0.],
                [-0.47942555, 0., 0.87758255]
            ]
            .into()
        );
    }

    #[test]
    fn make_rotation_z_works() {
        let mat3 = Mat3::from_angle_z(0.5);
        assert_eq!(
            mat3,
            [
                [0.87758255, -0.47942555, 0.],
                [0.47942555, 0.87758255, 0.],
                [0., 0., 1.]
            ]
            .into()
        );
    }

    #[test]
    fn make_rotation_works() {
        let mat3 = Mat3::from_axis_angle(Vec3::new(1., 2., 3.).normalize(), 0.5);
        assert_eq!(
            mat3,
            [
                [0.8863267, -0.3669074, 0.282496],
                [0.4018838, 0.912559, -0.0756672],
                [-0.2300314, 0.1805965, 0.9562795]
            ]
            .into()
        );
    }

    #[test]
    fn make_reflection_works() {
        let mat3 = Mat3::make_reflection(Vec3::new(1., 2., 3.).normalize());
        assert_eq!(
            mat3,
            [
                [6. / 7., -2. / 7., -3. / 7.],
                [-2. / 7., 3. / 7., -6. / 7.],
                [-3. / 7., -6. / 7., -2. / 7.]
            ]
            .into()
        );
    }

    #[test]
    fn make_involution_works() {
        let mat3 = Mat3::make_involution(Vec3::new(1., 2., 3.).normalize());
        assert_eq!(
            mat3,
            [
                [-6. / 7., 2. / 7., 3. / 7.],
                [2. / 7., -3. / 7., 6. / 7.],
                [3. / 7., 6. / 7., 2. / 7.]
            ]
            .into()
        );
    }

    #[test]
    fn make_scale_works() {
        let mat3 = Mat3::make_scale(Vec3::new(1., 2., 3.));
        assert_eq!(mat3, [[1., 0., 0.], [0., 2., 0.], [0., 0., 3.]].into());
    }

    #[test]
    fn make_scale_along_works() {
        let mat3 = Mat3::make_scale_along(2., Vec3::new(1., 2., 3.).normalize());
        assert_eq!(
            mat3,
            [
                [15. / 14., 1. / 7., 3. / 14.],
                [1. / 7., 9. / 7., 3. / 7.],
                [3. / 14., 3. / 7., 23. / 14.]
            ]
            .into()
        );
    }
}
