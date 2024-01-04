use std::{marker::PhantomData, ptr};

use apple_sys::Accelerate;
use num_complex::Complex;

pub struct DftSetup<T: Scalar> {
    raw: ptr::NonNull<T::SetupStruct>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T: Scalar> Drop for DftSetup<T> {
    fn drop(&mut self) {
        unsafe { T::destroy_interleaved_setup(self.raw) }
    }
}

impl<T: Scalar> DftSetup<T> {
    pub fn new(len: usize, direction: Direction) -> Result<Self, NewDftSetupError> {
        Ok(Self {
            raw: ptr::NonNull::new(unsafe {
                T::create_interleaved_setup(
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
        unsafe { T::execute_interleaved_setup(self.raw, input.as_ptr(), output.as_mut_ptr()) }
    }

    pub fn execute_in_place(&self, data: &mut [Complex<T>]) {
        assert_eq!(self.len, data.len());
        let ptr = data.as_mut_ptr();
        unsafe { T::execute_interleaved_setup(self.raw, ptr, ptr) }
    }
}

#[repr(i32)]
#[non_exhaustive]
pub enum Direction {
    Forward = Accelerate::FFT_FORWARD,
    Inverse = Accelerate::FFT_INVERSE,
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

// Scalar trait

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Scalar: Sized {
    type SetupStruct;

    unsafe fn create_interleaved_setup(
        previous: *mut Self::SetupStruct,
        length: Accelerate::vDSP_Length,
        direction: Accelerate::vDSP_DFT_Direction,
        real_to_complex: Accelerate::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::SetupStruct;

    unsafe fn execute_interleaved_setup(
        setup: ptr::NonNull<Self::SetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    );

    unsafe fn destroy_interleaved_setup(setup: ptr::NonNull<Self::SetupStruct>);
}

unsafe impl Scalar for f32 {
    type SetupStruct = Accelerate::vDSP_DFT_Interleaved_SetupStruct;

    unsafe fn create_interleaved_setup(
        previous: *mut Self::SetupStruct,
        length: Accelerate::vDSP_Length,
        direction: Accelerate::vDSP_DFT_Direction,
        real_to_complex: Accelerate::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::SetupStruct {
        Accelerate::vDSP_DFT_Interleaved_CreateSetup(previous, length, direction, real_to_complex)
    }

    unsafe fn execute_interleaved_setup(
        setup: ptr::NonNull<Self::SetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    ) {
        Accelerate::vDSP_DFT_Interleaved_Execute(setup.as_ptr(), input.cast(), output.cast())
    }

    unsafe fn destroy_interleaved_setup(setup: ptr::NonNull<Self::SetupStruct>) {
        Accelerate::vDSP_DFT_Interleaved_DestroySetup(setup.as_ptr())
    }
}

unsafe impl Scalar for f64 {
    type SetupStruct = Accelerate::vDSP_DFT_Interleaved_SetupStructD;

    unsafe fn create_interleaved_setup(
        previous: *mut Self::SetupStruct,
        length: Accelerate::vDSP_Length,
        direction: Accelerate::vDSP_DFT_Direction,
        real_to_complex: Accelerate::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::SetupStruct {
        Accelerate::vDSP_DFT_Interleaved_CreateSetupD(previous, length, direction, real_to_complex)
    }

    unsafe fn execute_interleaved_setup(
        setup: ptr::NonNull<Self::SetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    ) {
        Accelerate::vDSP_DFT_Interleaved_ExecuteD(setup.as_ptr(), input.cast(), output.cast())
    }

    unsafe fn destroy_interleaved_setup(setup: ptr::NonNull<Self::SetupStruct>) {
        Accelerate::vDSP_DFT_Interleaved_DestroySetupD(setup.as_ptr())
    }
}
