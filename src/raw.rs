// Scalar trait

use std::ptr;

use apple_sys::Accelerate;
use num_complex::Complex;

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Scalar: Sized {
    type DftInterleavedSetupStruct;

    unsafe fn dft_create_interleaved_setup(
        previous: *mut Self::DftInterleavedSetupStruct,
        length: Accelerate::vDSP_Length,
        direction: Accelerate::vDSP_DFT_Direction,
        real_to_complex: Accelerate::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::DftInterleavedSetupStruct;

    unsafe fn dft_execute_interleaved_setup(
        setup: ptr::NonNull<Self::DftInterleavedSetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    );

    unsafe fn dft_destroy_interleaved_setup(setup: ptr::NonNull<Self::DftInterleavedSetupStruct>);

    unsafe fn distance_squared(
        a: *const Self,
        stride_a: Accelerate::vDSP_Stride,
        b: *const Self,
        stride_b: Accelerate::vDSP_Stride,
        c: *mut Self,
        n: Accelerate::vDSP_Length,
    );

    unsafe fn dot_product(
        a: *const Self,
        stride_a: Accelerate::vDSP_Stride,
        b: *const Self,
        stride_b: Accelerate::vDSP_Stride,
        c: *mut Self,
        n: Accelerate::vDSP_Length,
    );
}

unsafe impl Scalar for f32 {
    type DftInterleavedSetupStruct = Accelerate::vDSP_DFT_Interleaved_SetupStruct;

    unsafe fn dft_create_interleaved_setup(
        previous: *mut Self::DftInterleavedSetupStruct,
        length: Accelerate::vDSP_Length,
        direction: Accelerate::vDSP_DFT_Direction,
        real_to_complex: Accelerate::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::DftInterleavedSetupStruct {
        Accelerate::vDSP_DFT_Interleaved_CreateSetup(previous, length, direction, real_to_complex)
    }

    unsafe fn dft_execute_interleaved_setup(
        setup: ptr::NonNull<Self::DftInterleavedSetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    ) {
        Accelerate::vDSP_DFT_Interleaved_Execute(setup.as_ptr(), input.cast(), output.cast())
    }

    unsafe fn dft_destroy_interleaved_setup(setup: ptr::NonNull<Self::DftInterleavedSetupStruct>) {
        Accelerate::vDSP_DFT_Interleaved_DestroySetup(setup.as_ptr())
    }

    unsafe fn distance_squared(
        a: *const Self,
        stride_a: Accelerate::vDSP_Stride,
        b: *const Self,
        stride_b: Accelerate::vDSP_Stride,
        c: *mut Self,
        n: Accelerate::vDSP_Length,
    ) {
        Accelerate::vDSP_distancesq(a, stride_a, b, stride_b, c, n)
    }

    unsafe fn dot_product(
        a: *const Self,
        stride_a: Accelerate::vDSP_Stride,
        b: *const Self,
        stride_b: Accelerate::vDSP_Stride,
        c: *mut Self,
        n: Accelerate::vDSP_Length,
    ) {
        Accelerate::vDSP_dotpr(a, stride_a, b, stride_b, c, n)
    }
}

unsafe impl Scalar for f64 {
    type DftInterleavedSetupStruct = Accelerate::vDSP_DFT_Interleaved_SetupStructD;

    unsafe fn dft_create_interleaved_setup(
        previous: *mut Self::DftInterleavedSetupStruct,
        length: Accelerate::vDSP_Length,
        direction: Accelerate::vDSP_DFT_Direction,
        real_to_complex: Accelerate::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::DftInterleavedSetupStruct {
        Accelerate::vDSP_DFT_Interleaved_CreateSetupD(previous, length, direction, real_to_complex)
    }

    unsafe fn dft_execute_interleaved_setup(
        setup: ptr::NonNull<Self::DftInterleavedSetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    ) {
        Accelerate::vDSP_DFT_Interleaved_ExecuteD(setup.as_ptr(), input.cast(), output.cast())
    }

    unsafe fn dft_destroy_interleaved_setup(setup: ptr::NonNull<Self::DftInterleavedSetupStruct>) {
        Accelerate::vDSP_DFT_Interleaved_DestroySetupD(setup.as_ptr())
    }

    unsafe fn distance_squared(
        a: *const Self,
        stride_a: Accelerate::vDSP_Stride,
        b: *const Self,
        stride_b: Accelerate::vDSP_Stride,
        c: *mut Self,
        n: Accelerate::vDSP_Length,
    ) {
        Accelerate::vDSP_distancesqD(a, stride_a, b, stride_b, c, n)
    }

    unsafe fn dot_product(
        a: *const Self,
        stride_a: Accelerate::vDSP_Stride,
        b: *const Self,
        stride_b: Accelerate::vDSP_Stride,
        c: *mut Self,
        n: Accelerate::vDSP_Length,
    ) {
        Accelerate::vDSP_dotprD(a, stride_a, b, stride_b, c, n)
    }
}
