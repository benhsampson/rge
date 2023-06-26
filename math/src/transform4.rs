use crate::{mat4::Mat4, plane::Plane, structure::EuclideanSpace, vec3::Vec3};

pub trait Transform4
where
    Self: Clone,
{
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
    ) -> Self;

    fn from_columns(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> Self;

    fn get_translation(&self) -> Vec3;

    fn set_translation(&mut self, t: Vec3);

    fn invert(&self) -> Option<Self>;

    fn make_reflection(plane: &Plane) -> Self;
}

pub trait T4Mul<Rhs = Self>
where
    Self: Transform4,
{
    fn mul(&self, rhs: &Rhs) -> Rhs;
}

impl Transform4 for Mat4 {
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

    fn from_columns(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> Self {
        Self::from_columns(a.extend(0.), b.extend(0.), c.extend(0.), p.extend(1.))
    }

    fn get_translation(&self) -> Vec3 {
        self.d.xyz()
    }

    fn set_translation(&mut self, t: Vec3) {
        self.d.x = t.x;
        self.d.y = t.y;
        self.d.z = t.z;
    }

    fn invert(&self) -> Option<Self> {
        let a: Vec3 = self.a.into();
        let b: Vec3 = self.b.into();
        let c: Vec3 = self.c.into();
        let d: Vec3 = self.d.into();

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

impl T4Mul for Mat4 {
    fn mul(&self, b: &Self) -> Self {
        let a = self;
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
    }
}

impl T4Mul<Plane> for Mat4 {
    fn mul(&self, f: &Plane) -> Plane {
        let h = self;
        let Plane { x, y, z, d } = f;
        Plane::new(
            x * h[0][0] + y * h[1][0] + z * h[2][0],
            x * h[0][1] + y * h[1][1] + z * h[2][1],
            x * h[0][2] + y * h[1][2] + z * h[2][2],
            x * h[0][3] + y * h[1][3] + z * h[2][3] + d,
        )
    }
}
