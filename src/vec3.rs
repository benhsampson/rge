use crate::{
    structure::{EuclideanSpace, VecSpace},
    vec4::Vec4,
};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub const X: Self = Self {
        x: 1.,
        y: 0.,
        z: 0.,
    };

    pub const Y: Self = Self {
        x: 0.,
        y: 1.,
        z: 0.,
    };

    pub const Z: Self = Self {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn extend(&self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }
}

impl VecSpace for Vec3 {}

impl EuclideanSpace<Self> for Vec3 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::precision::PRECISION;

    use super::*;

    #[test]
    fn new_works() {
        assert_eq!(
            Vec3::new(1., 2., 3.),
            Vec3 {
                x: 1.,
                y: 2.,
                z: 3.
            }
        );
    }

    #[test]
    fn converts() {
        let mut v = Vec3::new(1., 2., 3.);

        assert_eq!(v, [1., 2., 3.].into());
        assert_eq!(v, (&[1., 2., 3.]).into());
        assert_eq!(v, (&mut [1., 2., 3.]).into());

        let a = [1., 2., 3.];
        assert_eq!(a, <Vec3 as Into<[f32; 3]>>::into(v));
        let vb = &v;
        assert_eq!(a, <&Vec3 as Into<[f32; 3]>>::into(vb));
        let vb = &mut v;
        assert_eq!(a, <&mut Vec3 as Into<[f32; 3]>>::into(vb));
    }

    #[test]
    fn indexes() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v[0], 1.);
        assert_eq!(v[1], 2.);
        assert_eq!(v[2], 3.);
    }

    #[test]
    fn normalizes() {
        let v = Vec3::new(1., 2., 3.);
        let n = v.normalize();
        approx_eq!(f32, n.norm2(), 1., PRECISION);
    }

    #[test]
    fn zero_works() {
        assert_eq!(Vec3::ZERO, Vec3::new(0., 0., 0.));
    }

    #[test]
    fn neg_works() {
        assert_eq!(-Vec3::new(1., 2., 3.), Vec3::new(-1., -2., -3.));
    }

    #[test]
    fn add_works() {
        assert_eq!(
            Vec3::new(1., 2., 3.) + Vec3::new(4., 5., 6.),
            Vec3::new(5., 7., 9.)
        );
    }

    #[test]
    fn add_assign_works() {
        let mut vec3 = Vec3::new(1., 2., 3.);
        vec3 += Vec3::new(4., 5., 6.);
        assert_eq!(vec3, Vec3::new(5., 7., 9.));
    }

    #[test]
    fn sub_works() {
        assert_eq!(
            Vec3::new(1., 2., 3.) - Vec3::new(4., 5., 6.),
            Vec3::new(-3., -3., -3.)
        );
    }

    #[test]
    fn sub_assign_works() {
        let mut vec3 = Vec3::new(1., 2., 3.);
        vec3 -= Vec3::new(4., 5., 6.);
        assert_eq!(vec3, Vec3::new(-3., -3., -3.));
    }

    #[test]
    fn mul_works() {
        assert_eq!(Vec3::new(1., 2., 3.) * 2., Vec3::new(2., 4., 6.));
    }

    #[test]
    fn mul_assign_works() {
        let mut vec3 = Vec3::new(1., 2., 3.);
        vec3 *= 2.;
        assert_eq!(vec3, Vec3::new(2., 4., 6.));
    }

    #[test]
    fn div_works() {
        assert_eq!(Vec3::new(1., 2., 3.) / 2., Vec3::new(0.5, 1., 1.5));
    }

    #[test]
    fn div_assign_works() {
        let mut vec3 = Vec3::new(1., 2., 3.);
        vec3 /= 2.;
        assert_eq!(vec3, Vec3::new(0.5, 1., 1.5));
    }

    #[test]
    fn sqr_magnitude_works() {
        assert_eq!(Vec3::new(1., 2., 3.).norm2(), 14.);
    }

    #[test]
    fn magnitude_works() {
        assert_eq!(Vec3::new(1., 2., 3.).norm(), 14_f32.sqrt());
    }

    #[test]
    fn normalize_works() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.normalize(), v / 14_f32.sqrt());
    }

    #[test]
    fn dot_works() {
        assert_eq!(Vec3::new(1., 2., 3.).dot(&Vec3::new(4., 5., 6.)), 32.);
    }

    #[test]
    fn cross_works() {
        assert_eq!(
            Vec3::new(1., 2., 3.).cross(&Vec3::new(4., 5., 6.)),
            Vec3::new(-3., 6., -3.)
        );
    }

    #[test]
    fn project_works() {
        assert_eq!(
            Vec3::new(1., 2., 3.).project_on(&Vec3::new(4., 5., 6.)),
            Vec3::new(4., 5., 6.) * (32. / 77.)
        );
    }

    #[test]
    fn reject_works() {
        assert_eq!(
            Vec3::new(1., 2., 3.).reject_on(&Vec3::new(4., 5., 6.)),
            Vec3::new(1., 2., 3.) - Vec3::new(4., 5., 6.) * (32. / 77.)
        );
    }
}
