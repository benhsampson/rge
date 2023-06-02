use crate::{mat4::Mat4, structure::VecSpace, vec3::Vec3};

pub trait Transform4
where
    Self: Clone,
{
    #[allow(clippy::too_many_arguments)]
    fn new(
        c0r0: f32,
        c0r1: f32,
        c0r2: f32,
        c0r3: f32,
        c1r0: f32,
        c1r1: f32,
        c1r2: f32,
        c1r3: f32,
        c2r0: f32,
        c2r1: f32,
        c2r2: f32,
        c2r3: f32,
    ) -> Self;

    fn from_columns(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> Self;

    fn get_translation(&self) -> Vec3;

    fn set_translation(&mut self, p: Vec3);

    fn inverse(&self) -> Option<Self>;
}

impl Transform4 for Mat4 {
    fn new(
        c0r0: f32,
        c0r1: f32,
        c0r2: f32,
        c0r3: f32,
        c1r0: f32,
        c1r1: f32,
        c1r2: f32,
        c1r3: f32,
        c2r0: f32,
        c2r1: f32,
        c2r2: f32,
        c2r3: f32,
    ) -> Self {
        Self::from_columns(
            [c0r0, c0r1, c0r2, c0r3].into(),
            [c1r0, c1r1, c1r2, c1r3].into(),
            [c2r0, c2r1, c2r2, c2r3].into(),
            [0.0, 0.0, 0.0, 1.0].into(),
        )
    }

    fn from_columns(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> Self {
        Self::from_columns(a.extend(0.), b.extend(0.), c.extend(0.), p.extend(1.))
    }

    fn get_translation(&self) -> Vec3 {
        self.d.xyz()
    }

    fn set_translation(&mut self, p: Vec3) {
        self.d.x = p.x;
        self.d.y = p.y;
        self.d.z = p.z;
    }

    fn inverse(&self) -> Option<Self> {
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
}
