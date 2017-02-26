use num::Num;


#[inline]
pub fn sdiv<'a, 'b, T: Copy + Num>(out: &'a mut [T; 9], a: &'b [T; 9], s: T) ->  &'a mut [T; 9] {
    let not_zero = s != T::zero();
    out[0] = if not_zero {a[0] / s} else  {T::zero()};
    out[1] = if not_zero {a[1] / s} else  {T::zero()};
    out[2] = if not_zero {a[2] / s} else  {T::zero()};
    out[3] = if not_zero {a[3] / s} else  {T::zero()};
    out[4] = if not_zero {a[4] / s} else  {T::zero()};
    out[5] = if not_zero {a[5] / s} else  {T::zero()};
    out[6] = if not_zero {a[6] / s} else  {T::zero()};
    out[7] = if not_zero {a[7] / s} else  {T::zero()};
    out[8] = if not_zero {a[8] / s} else  {T::zero()};
    out
}
#[test]
fn test_sdiv() {
    let mut v = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    sdiv(&mut v, &[1, 0, 0, 0, 1, 0, 0, 0, 1], 1);
    assert!(v == [1, 0, 0, 0, 1, 0, 0, 0, 1]);
}
