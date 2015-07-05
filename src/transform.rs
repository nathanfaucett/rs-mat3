use num::Num;


#[inline(always)]
pub fn scale<T: Num>(out: &mut [T; 9], a: [T; 9], v: [T; 3]) -> &mut [T; 9] {
    let x = v[0];
    let y = v[1];
    let z = v[2];

    out[0] = a[0] * x;
    out[3] = a[3] * y;
    out[6] = a[6] * z;
    out[1] = a[1] * x;
    out[4] = a[4] * y;
    out[7] = a[7] * z;
    out[2] = a[2] * x;
    out[5] = a[5] * y;
    out[8] = a[8] * z;
    out
}

#[inline(always)]
pub fn from_quat<T: Num>(out: &mut [T; 9], q: [T; 4]) -> &mut [T; 9] {
    let x = q[0];
    let y = q[1];
    let z = q[2];
    let w = q[3];
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;

    out[0] = T::one() - (yy + zz);
    out[1] = xy + wz;
    out[2] = xz - wy;

    out[3] = xy - wz;
    out[4] = T::one() - (xx + zz);
    out[5] = yz + wx;

    out[6] = xz + wy;
    out[7] = yz - wx;
    out[8] = T::one() - (xx + yy);
    out
}
