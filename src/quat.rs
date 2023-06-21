use crate::{impl_conversions, mat3::Mat3, structure::VecSpace};
use std::ops;

use crate::{impl_op, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quat {
        Quat { x, y, z, w }
    }

    pub fn get_vec_part(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl_conversions!(Quat => [f32; 4], |from: &Quat| {
    [from.x, from.y, from.z, from.w]
});

impl_conversions!([f32; 4] => Quat, |from: &[f32; 4]| {
    Quat::new(from[0], from[1], from[2], from[3])
});

impl_op!(Quat : Quat, ops::Mul { fn mul |lhs: &Quat, rhs: &Quat| {
    let [x1, y1, z1, w1]: [f32; 4] = lhs.into();
    let [x2, y2, z2, w2]: [f32; 4] = rhs.into();
    Quat::new(
        w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
        w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
        w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2,
        w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
    ) }
});

impl_op!(Vec3 : Quat, ops::Mul { fn mul |v: &Vec3, q: &Quat| {
    let b = q.get_vec_part();
    let b2 = b.x * b.x + b.y * b.y + b.z * b.z;
    v * (q.w * q.w - b2) + b * (v.dot(&b) * 2.) + b.cross(v) * (q.w * 2.)
}});

impl_conversions!(Quat => Mat3, |q: &Quat| {
    let [x, y, z, w]: [f32; 4] = q.into();
    let x2 = x * x;
    let y2 = y * y;
    let z2 = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;
    Mat3::new(
        1. - 2. * (y2 + z2), 2. * (xy - wz), 2. * (xz + wy),
        2. * (xy + wz), 1. - 2. * (x2 + z2), 2. * (yz - wx),
        2. * (xz - xy), 2. * (yz + wx), 1. - 2. * (x2 + y2)
    )
});

impl_conversions!(Mat3 => Quat, |m: &Mat3| {
    let m00 = m[0][0];
    let m11 = m[1][1];
    let m22 = m[2][2];
    let sum = m00 + m11 + m22;

    if sum > 0. {
        let w = (sum + 1.).sqrt() * 0.5;
        let f = 0.25 / w;
        let x = (m[2][1] - m[1][2]) * f;
        let y = (m[0][2] - m[2][0]) * f;
        let z = (m[1][0] - m[0][1]) * f;
        Quat::new(x, y, z, w)
    } else if (m00 > m11) && (m00 > m22) {
        // x is largest
        let x = (m00 - m11 - m22 + 1.).sqrt() * 0.5;
        let f = 0.25 / x;
        let y = (m[1][0] + m[0][1]) * f;
        let z = (m[0][2] + m[2][0]) * f;
        let w = (m[2][1] - m[1][2]) * f;
        Quat::new(x, y, z, w)
    } else if m11 > m22 {
        // y is largest
        let y = (m11 - m00 - m22 + 1.).sqrt() * 0.5;
        let f = 0.25 / y;
        let x = (m[1][0] + m[0][1]) * f;
        let z = (m[2][1] + m[1][2]) * f;
        let w = (m[0][2] - m[2][0]) * f;
        Quat::new(x, y, z, w)
    } else {
        // z is largest
        let z = (m22 - m00 - m11 + 1.).sqrt() * 0.5;
        let f = 0.25 / z;
        let x = (m[0][2] + m[2][0]) * f;
        let y = (m[2][1] + m[1][2]) * f;
        let w = (m[1][0] - m[0][1]) * f;
        Quat::new(x, y, z, w)
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let m = Mat3::new(
            0.5403023, -0.841471, 0., //
            0.841471, 0.5403023, 0., //
            0., 0., 1.,
        );
        let q = Quat::new(0., 0., 0., 1.);
        assert_eq!(q, m.into());
    }
}
