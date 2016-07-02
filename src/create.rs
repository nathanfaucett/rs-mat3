use num::Num;


#[inline(always)]
pub fn new<T: Num>(
    m11: T, m12: T, m13: T,
    m21: T, m22: T, m23: T,
    m31: T, m32: T, m33: T
) -> [T; 9] {[
    m11, m12, m13,
    m21, m22, m23,
    m31, m32, m33
]}
#[inline(always)]
pub fn create<T: Num>(
    m11: T, m12: T, m13: T,
    m21: T, m22: T, m23: T,
    m31: T, m32: T, m33: T
) -> [T; 9] {new(
    m11, m12, m13,
    m21, m22, m23,
    m31, m32, m33
)}
#[test]
fn test_new() {
    let m = new(1, 0, 0, 0, 1, 0, 0, 0, 1);
    assert!(m[0] == 1);
    assert!(m[1] == 0);
    assert!(m[2] == 0);
    assert!(m[3] == 0);
    assert!(m[4] == 1);
    assert!(m[5] == 0);
    assert!(m[6] == 0);
    assert!(m[7] == 0);
    assert!(m[8] == 1);
}

#[inline(always)]
pub fn new_identity<T: Num>() -> [T; 9] {
    new(
        T::one(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero(),
        T::zero(), T::one(), T::one()
    )
}
#[inline(always)]
pub fn new_zero<T: Num>() -> [T; 9] {
    new(
        T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero()
    )
}

#[inline(always)]
pub fn clone<'b, T: Num>(m: &'b [T; 9]) -> [T; 9] {
    new(
        m[0], m[3], m[6],
        m[1], m[4], m[7],
        m[2], m[5], m[8]
    )
}

#[inline(always)]
pub fn copy<'a, 'b, T: Num>(out: &'a mut [T; 9], a: &'b [T; 9]) -> &'a mut [T; 9] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out[4] = a[4];
    out[5] = a[5];
    out[6] = a[6];
    out[7] = a[7];
    out[8] = a[8];
    out
}
