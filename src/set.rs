use num::Num;


#[inline(always)]
pub fn set<T: Num>(
    out: &mut [T; 9],
    m11: T, m12: T, m13: T,
    m21: T, m22: T, m23: T,
    m31: T, m32: T, m33: T
) -> &mut [T; 9] {
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
pub fn zero<T: Num>(out: &mut [T; 9]) -> &mut [T; 9] {
    set(out,
        T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero()
    )
}
#[inline(always)]
pub fn identity<T: Num>(out: &mut [T; 9]) -> &mut [T; 9] {
    set(out,
        T::one(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero(),
        T::zero(), T::zero(), T::one()
    )
}
