use crate::{
    plane::Plane,
    pt3::Pt3,
    structure::{EuclideanSpace, VecSpace},
    vec3::Vec3,
};

fn dst_pt_line(q: &Pt3, p: &Pt3, v: &Vec3) -> f32 {
    let a = Vec3::from(q - p).cross(v);
    (a.dot(&a) / v.norm2()).sqrt()
}

fn dst_line_line(p1: &Pt3, v1: &Vec3, p2: &Pt3, v2: &Vec3) -> f32 {
    let dp = Vec3::from(p2 - p1);
    let v12 = v1.norm2();
    let v22 = v2.norm2();
    let v1v2 = v1.dot(v2);
    let det = v1v2.powi(2) - v12 * v22;
    if det.abs() > f32::MIN {
        // Lines are not parallel
        let det = 1. / det;
        let dpv1 = dp.dot(v1);
        let dpv2 = dp.dot(v2);
        let t1 = (v1v2 * dpv2 - v22 * dpv1) * det;
        let t2 = (v12 * dpv2 - v1v2 * dpv1) * det;
        (dp + v2 * t2 - v1 * t1).norm()
    } else {
        // Lines are parallel
        let a = dp.cross(v1);
        (a.dot(&a) / v12).sqrt()
    }
}

fn intersect_line_plane(p: &Pt3, v: &Vec3, f: &Plane) -> Option<Pt3> {
    let fv = f.dot(v);
    if fv.abs() > f32::MIN {
        let q = p - v * (f.dot(p) / fv);
        Some(q)
    } else {
        None
    }
}

fn intersect_three_planes(f1: &Plane, f2: &Plane, f3: &Plane) -> Option<Pt3> {
    let n1 = f1.normal();
    let n2 = f2.normal();
    let n3 = f3.normal();
    let n1xn2 = n1.cross(&n2);
    let det = n1xn2.dot(&n3);
    if det.abs() > f32::MIN {
        let n3xn2 = n3.cross(&n2);
        let n1xn3 = n1.cross(&n3);
        let p = (n3xn2 * f1.d + n1xn3 * f2.d - n1xn2 * f3.d) / det;
        Some(p.into())
    } else {
        None
    }
}

fn intersect_two_planes(f1: &Plane, f2: &Plane) -> Option<(Pt3, Vec3)> {
    let n1 = f1.normal();
    let n2 = f2.normal();
    let v = n1.cross(&n2);
    let det = v.norm2();
    if det.abs() > f32::MIN {
        let vxn2 = v.cross(&n2);
        let n1xv = n1.cross(&v);
        let p = (vxn2 * f1.d + n1xv * f2.d) / det;
        Some((p.into(), v))
    } else {
        None
    }
}
