use float_cmp::{ApproxEq, F32Margin};

use crate::{mat3::Mat3, mat4::Mat4, vec3::Vec3, vec4::Vec4};

pub const PRECISION: F32Margin = F32Margin {
    ulps: 2,
    epsilon: 1e-3,
};

impl ApproxEq for &Vec3 {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(other, PRECISION)
    }
}

impl ApproxEq for &Vec4 {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
            && self.w.approx_eq(other.w, margin)
    }
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(other, PRECISION)
    }
}

impl ApproxEq for &Mat3 {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.a.approx_eq(&other.a, margin)
            && self.b.approx_eq(&other.b, margin)
            && self.c.approx_eq(&other.c, margin)
    }
}

impl PartialEq for Mat3 {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(other, PRECISION)
    }
}

impl ApproxEq for &Mat4 {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.a.approx_eq(&other.a, margin)
            && self.b.approx_eq(&other.b, margin)
            && self.c.approx_eq(&other.c, margin)
            && self.d.approx_eq(&other.d, margin)
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(other, PRECISION)
    }
}
