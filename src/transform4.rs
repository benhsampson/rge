use crate::{
    impl_mat4, impl_op,
    plane::Plane,
    structure::{EuclideanSpace, Mat, SquareMat},
    vec3::Vec3,
    vec4::Vec4,
};
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Transform4 {
    pub a: Vec4,
    pub b: Vec4,
    pub c: Vec4,
    pub p: Vec4,
}

impl Transform4 {
    #[allow(clippy::too_many_arguments)]
    fn new(
        c0r0: f32,
        c1r0: f32,
        c2r0: f32,
        c3r0: f32,
        c0r1: f32,
        c1r1: f32,
        c2r1: f32,
        c3r1: f32,
        c0r2: f32,
        c1r2: f32,
        c2r2: f32,
        c3r2: f32,
    ) -> Self {
        Self::from_columns(
            [c0r0, c0r1, c0r2, 0.].into(),
            [c1r0, c1r1, c1r2, 0.].into(),
            [c2r0, c2r1, c2r2, 0.].into(),
            [c3r0, c3r1, c3r2, 1.].into(),
        )
    }

    pub fn from_columns(a: Vec4, b: Vec4, c: Vec4, p: Vec4) -> Self {
        Self { a, b, c, p }
    }

    pub fn from_abc_p(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> Self {
        Self::from_columns(a.extend(0.), b.extend(0.), c.extend(0.), p.extend(1.))
    }

    fn get_translation(&self) -> Vec3 {
        self.p.xyz()
    }

    fn set_translation(&mut self, p: Vec3) {
        self.p.x = p.x;
        self.p.y = p.y;
        self.p.z = p.z;
    }

    fn make_reflection(plane: &Plane) -> Self {
        let x = plane.x * -2.;
        let y = plane.y * -2.;
        let z = plane.z * -2.;
        let xy = x * plane.y;
        let xz = x * plane.z;
        let yz = y * plane.z;
        Self::new(
            x * plane.x + 1.,
            xy,
            xz,
            x * plane.d,
            xy,
            y * plane.y + 1.,
            yz,
            y * plane.d,
            xz,
            yz,
            z * plane.z + 1.,
            z * plane.d,
        )
    }
}

impl_mat4!(Transform4 { a, b, c, p });

impl SquareMat for Transform4 {
    type RowColumn = Vec4;

    const IDENTITY: Self = Self {
        a: Vec4::X,
        b: Vec4::Y,
        c: Vec4::Z,
        p: Vec4::W,
    };

    fn from_diagonal(d: Self::RowColumn) -> Self {
        todo!()
    }

    fn determinant(&self) -> f32 {
        todo!()
    }

    fn invert(&self) -> Option<Self> {
        let a: Vec3 = self.a.into();
        let b: Vec3 = self.b.into();
        let c: Vec3 = self.c.into();
        let d: Vec3 = self.p.into();

        let mut s = a.cross(&b);
        let mut t = c.cross(&d);

        let det = s.dot(&c);

        if det != 0. {
            let inv_det = 1. / det;
            s *= inv_det;
            t *= inv_det;

            let v = c * inv_det;

            let r0 = b.cross(&v);
            let r1 = v.cross(&a);

            Some(Self::from_columns(
                r0.extend(-b.dot(&t)),
                r1.extend(a.dot(&t)),
                s.extend(-d.dot(&s)),
                [0., 0., 0., 1.].into(),
            ))
        } else {
            None
        }
    }
}

impl_op!(Transform4 : Transform4, ops::Mul { fn mul |a: &Transform4, b: &Transform4| {
    Transform4::new(
        a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0],
        a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1],
        a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2],
        a[0][0] * b[0][3] + a[0][1] * b[1][3] + a[0][2] * b[2][3] + a[0][3],
        a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0],
        a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1],
        a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2],
        a[1][0] * b[0][3] + a[1][1] * b[1][3] + a[1][2] * b[2][3] + a[1][3],
        a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0],
        a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1],
        a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2],
        a[2][0] * b[0][3] + a[2][1] * b[1][3] + a[2][2] * b[2][3] + a[2][3],
    )
}});
