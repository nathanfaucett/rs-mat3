use num::Num;
use set::identity;


#[inline(always)]
pub fn inverse_mat<'a, 'b, T: Num>(
    out: &'a mut [T; 9],
    m11: T, m12: T, m13: T,
    m21: T, m22: T, m23: T,
    m31: T, m32: T, m33: T
) -> &'a mut [T; 9] {
    let m0 = m22 * m33 - m23 * m32;
    let m3 = m13 * m32 - m12 * m33;
    let m6 = m12 * m23 - m13 * m22;
    let d = m11 * m0 + m21 * m3 + m31 * m6;

    if d != T::zero() {
        let inv_d = T::one() / d;

        out[0] = m0 * inv_d;
        out[1] = (m23 * m31 - m21 * m33) * inv_d;
        out[2] = (m21 * m32 - m22 * m31) * inv_d;

        out[3] = m3 * inv_d;
        out[4] = (m11 * m33 - m13 * m31) * inv_d;
        out[5] = (m12 * m31 - m11 * m32) * inv_d;

        out[6] = m6 * inv_d;
        out[7] = (m13 * m21 - m11 * m23) * inv_d;
        out[8] = (m11 * m22 - m12 * m21) * inv_d;
        out
    } else {
        identity(out)
    }
}
#[test]
fn test_inverse_mat() {
    let mut v = [1, 0, 0, 0, 1, 0, 0, 0, 1];
    inverse_mat(&mut v, 1, 0, 0, 0, 1, 0, 0, 0, 1);
    assert!(v == [1, 0, 0, 0, 1, 0, 0, 0, 1]);
}

pub fn inverse<'a, 'b, T: Num>(out: &'a mut [T; 9], a: &'b [T; 9]) -> &'a mut [T; 9] {
    inverse_mat(
        out,
        a[0], a[3], a[6],
        a[1], a[4], a[7],
        a[2], a[5], a[8]
    )
}
pub fn inverse_mat2<'a, 'b, T: Num>(out: &'a mut [T; 9], a: &'b [T; 4]) -> &'a mut [T; 9] {
    inverse_mat(
        out,
        a[0], a[2], T::zero(),
        a[1], a[3], T::zero(),
        T::zero(), T::zero(), T::one()
    )
}
pub fn inverse_mat32<'a, 'b, T: Num>(out: &'a mut [T; 9], a: &'b [T; 6]) -> &'a mut [T; 9] {
    inverse_mat(
        out,
        a[0], a[2], T::zero(),
        a[1], a[3], T::zero(),
        T::zero(), T::zero(), T::one()
    )
}
pub fn inverse_mat4<'a, 'b, T: Num>(out: &'a mut [T; 9], a: &'b [T; 16]) -> &'a mut [T; 9] {
    inverse_mat(
        out,
        a[0], a[4], a[8],
        a[1], a[5], a[9],
        a[2], a[6], a[10]
    )
}

#[inline(always)]
pub fn determinant<'a, 'b, T: Num>(out: &'b [T; 9]) -> T {
    let a = out[0];
    let b = out[1];
    let c = out[2];
    let d = out[3];
    let e = out[4];
    let f = out[5];
    let g = out[6];
    let h = out[7];
    let i = out[8];
    return a * e * i - a * f * h - b * d * i + b * f * g + c * d * h - c * e * g;
}
#[test]
fn test_determinant() {
    assert_eq!(determinant(&[1, 0, 0, 0, 1, 0, 0, 0, 1]), 1);
}

#[inline(always)]
pub fn transpose<'a, 'b, T: Num>(out: &'a mut [T; 9], a: &'b [T; 9]) -> &'a mut [T; 9] {
    out[0] = a[0];
    out[1] = a[3];
    out[2] = a[6];
    out[3] = a[1];
    out[4] = a[4];
    out[5] = a[7];
    out[6] = a[2];
    out[7] = a[5];
    out[8] = a[8];
    out
}
#[test]
fn test_transpose() {
    let mut v = [1, 0, 0, 0, 1, 0, 0, 0, 1];
    transpose(&mut v, &[1, 0, 0, 0, 1, 0, 0, 0, 1]);
    assert_eq!(v, [1, 0, 0, 0, 1, 0, 0, 0, 1]);
}
