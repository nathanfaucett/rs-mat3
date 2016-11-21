use num::Unsigned;


#[inline(always)]
pub fn set<'a, 'b, T: Unsigned>(
    out: &'a mut [T; 9],
    m11: T, m12: T, m13: T,
    m21: T, m22: T, m23: T,
    m31: T, m32: T, m33: T
) -> &'a mut [T; 9] {
    out[0] = m11;
    out[1] = m21;
    out[2] = m31;
    out[3] = m12;
    out[4] = m22;
    out[5] = m32;
    out[6] = m13;
    out[7] = m23;
    out[8] = m33;
    out
}

#[inline(always)]
pub fn zero<'a, 'b, T: Unsigned>(out: &'a mut [T; 9]) -> &'a mut [T; 9] {
    set(out,
        T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero()
    )
}
#[inline(always)]
pub fn identity<'a, 'b, T: Unsigned>(out: &'a mut [T; 9]) -> &'a mut [T; 9] {
    set(out,
        T::one(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero(),
        T::zero(), T::zero(), T::one()
    )
}

#[inline(always)]
pub fn from_mat2<'a, 'b, T: Unsigned>(out: &'a mut [T; 9], m: &'b [T; 4]) -> &'a mut [T; 9] {
    set(
        out,
        m[0], m[2], T::zero(),
        m[1], m[3], T::zero(),
        T::zero(), T::zero(), T::one()
    )
}
#[inline(always)]
pub fn from_mat32<'a, 'b, T: Unsigned>(out: &'a mut [T; 9], m: &'b [T; 6]) -> &'a mut [T; 9] {
    set(
        out,
        m[0], m[2], T::zero(),
        m[1], m[3], T::zero(),
        T::zero(), T::zero(), T::one()
    )
}
#[inline(always)]
pub fn from_mat3<'a, 'b, T: Unsigned>(out: &'a mut [T; 9], m: &'b [T; 16]) -> &'a mut [T; 9] {
    set(
        out,
        m[0], m[4], m[8],
        m[1], m[5], m[9],
        m[2], m[6], m[10]
    )
}
