use std::{marker::PhantomData, ptr};

use num_complex::Complex;

use crate::raw::Scalar;
use crate::sys;

pub struct DftSetup<T: Scalar> {
    raw: ptr::NonNull<T::DftInterleavedSetupStruct>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T: Scalar> Drop for DftSetup<T> {
    fn drop(&mut self) {
        unsafe { T::dft_destroy_interleaved_setup(self.raw) }
    }
}

impl<T: Scalar> DftSetup<T> {
    pub fn new(len: usize, direction: FftDirection) -> Result<Self, NewDftSetupError> {
        Ok(Self {
            raw: ptr::NonNull::new(unsafe {
                T::dft_create_interleaved_setup(
                    ptr::null_mut(),
                    len.try_into().unwrap(),
                    direction as _,
                    false,
                )
            })
            .ok_or(NewDftSetupError(()))?
            .cast(),
            len,
            _marker: PhantomData,
        })
    }

    pub fn execute(&self, input: &[Complex<T>], output: &mut [Complex<T>]) {
        assert_eq!(self.len, input.len());
        assert_eq!(self.len, output.len());
        unsafe { T::dft_execute_interleaved_setup(self.raw, input.as_ptr(), output.as_mut_ptr()) }
    }

    pub fn execute_in_place(&self, data: &mut [Complex<T>]) {
        assert_eq!(self.len, data.len());
        let ptr = data.as_mut_ptr();
        unsafe { T::dft_execute_interleaved_setup(self.raw, ptr, ptr) }
    }
}

#[repr(i32)]
#[non_exhaustive]
pub enum FftDirection {
    Forward = sys::FFT_FORWARD,
    Inverse = sys::FFT_INVERSE,
}

// Errors

#[derive(Debug)]
pub struct NewDftSetupError(());

impl std::fmt::Display for NewDftSetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "len value is unsupported or too large".fmt(f)
    }
}

impl std::error::Error for NewDftSetupError {}
