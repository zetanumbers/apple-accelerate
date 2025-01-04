mod dft;
pub mod raw;
mod sys;

use std::mem;

pub use dft::*;

#[track_caller]
pub fn dot_product<T: raw::Scalar>(a: &[T], b: &[T]) -> T {
    let n = a.len();
    assert_eq!(n, b.len(), "input slices must have equal lengths");
    let n = n.try_into().expect("input slice length is too large");
    let mut out = mem::MaybeUninit::uninit();
    unsafe {
        T::dot_product(a.as_ptr(), 1, b.as_ptr(), 1, out.as_mut_ptr(), n);
        out.assume_init()
    }
}

#[track_caller]
pub fn distance_squared<T: raw::Scalar>(a: &[T], b: &[T]) -> T {
    let n = a.len();
    assert_eq!(n, b.len(), "input slices must have equal lengths");
    let n = n.try_into().expect("input slice length is too large");
    let mut out = mem::MaybeUninit::uninit();
    unsafe {
        T::distance_squared(a.as_ptr(), 1, b.as_ptr(), 1, out.as_mut_ptr(), n);
        out.assume_init()
    }
}

#[track_caller]
pub fn scale<T: raw::Scalar>(a: &[T], b: T, c: &mut [T]) {
    let n = a.len();
    assert_eq!(
        n,
        c.len(),
        "input and output slices must have equal lengths"
    );
    let n = n.try_into().expect("input slice length is too large");
    unsafe { T::vector_by_scalar_mul(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, n) };
}

#[track_caller]
pub fn scale_inplace<T: raw::Scalar>(a: &mut [T], b: T) {
    let n = a.len().try_into().expect("input slice length is too large");
    let a = a.as_mut_ptr();
    unsafe { T::vector_by_scalar_mul(a, 1, &b, a, 1, n) };
}
