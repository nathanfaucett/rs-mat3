use num::Num;


#[inline]
pub fn mul<'a, 'b, T: Num>(out: &'a mut [T; 9], a: &'b [T; 9], b: &'b [T; 9]) ->  &'a mut [T; 9] {
    let a11 = a[0];
    let a12 = a[3];
    let a13 = a[6];
    let a21 = a[1];
    let a22 = a[4];
    let a23 = a[7];
    let a31 = a[2];
    let a32 = a[5];
    let a33 = a[8];

    let b11 = b[0];
    let b12 = b[3];
    let b13 = b[6];
    let b21 = b[1];
    let b22 = b[4];
    let b23 = b[7];
    let b31 = b[2];
    let b32 = b[5];
    let b33 = b[8];

    out[0] = a11 * b11 + a21 * b12 + a31 * b13;
    out[3] = a12 * b11 + a22 * b12 + a32 * b13;
    out[6] = a13 * b11 + a23 * b12 + a33 * b13;

    out[1] = a11 * b21 + a21 * b22 + a31 * b23;
    out[4] = a12 * b21 + a22 * b22 + a32 * b23;
    out[7] = a13 * b21 + a23 * b22 + a33 * b23;

    out[2] = a11 * b31 + a21 * b32 + a31 * b33;
    out[5] = a12 * b31 + a22 * b32 + a32 * b33;
    out[8] = a13 * b31 + a23 * b32 + a33 * b33;
    out
}
#[test]
fn test_mul() {
    let mut v = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    mul(&mut v, &[1, 0, 0, 0, 1, 0, 0, 0, 1], &[1, 0, 0, 0, 1, 0, 0, 0, 1]);
    assert!(v == [1, 0, 0, 0, 1, 0, 0, 0, 1]);
}

#[inline]
pub fn smul<'a, 'b, T: Num>(out: &'a mut [T; 9], a: &'b [T; 9], s: T) ->  &'a mut [T; 9] {
    out[0] = a[0] * s;
    out[1] = a[1] * s;
    out[2] = a[2] * s;
    out[3] = a[3] * s;
    out[4] = a[4] * s;
    out[5] = a[5] * s;
    out[6] = a[6] * s;
    out[7] = a[7] * s;
    out[8] = a[8] * s;
    out
}
#[test]
fn test_smul() {
    let mut v = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    smul(&mut v, &[1, 0, 0, 0, 1, 0, 0, 0, 1], 1);
    assert!(v == [1, 0, 0, 0, 1, 0, 0, 0, 1]);
}
