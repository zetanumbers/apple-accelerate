// Scalar trait

use std::ptr;

use num_complex::Complex;

use crate::sys;

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Scalar: Sized {
    type DftInterleavedSetupStruct;

    unsafe fn dft_create_interleaved_setup(
        previous: *mut Self::DftInterleavedSetupStruct,
        length: sys::vDSP_Length,
        direction: sys::vDSP_DFT_Direction,
        real_to_complex: sys::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::DftInterleavedSetupStruct;

    unsafe fn dft_execute_interleaved_setup(
        setup: ptr::NonNull<Self::DftInterleavedSetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    );

    unsafe fn dft_destroy_interleaved_setup(setup: ptr::NonNull<Self::DftInterleavedSetupStruct>);

    unsafe fn distance_squared(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        stride_b: sys::vDSP_Stride,
        c: *mut Self,
        n: sys::vDSP_Length,
    );

    unsafe fn dot_product(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        stride_b: sys::vDSP_Stride,
        c: *mut Self,
        n: sys::vDSP_Length,
    );

    unsafe fn vector_by_scalar_mul(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        c: *mut Self,
        stride_c: sys::vDSP_Stride,
        n: sys::vDSP_Length,
    );
}

unsafe impl Scalar for f32 {
    type DftInterleavedSetupStruct = sys::vDSP_DFT_Interleaved_SetupStruct;

    unsafe fn dft_create_interleaved_setup(
        previous: *mut Self::DftInterleavedSetupStruct,
        length: sys::vDSP_Length,
        direction: sys::vDSP_DFT_Direction,
        real_to_complex: sys::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::DftInterleavedSetupStruct {
        sys::vDSP_DFT_Interleaved_CreateSetup(previous, length, direction, real_to_complex)
    }

    unsafe fn dft_execute_interleaved_setup(
        setup: ptr::NonNull<Self::DftInterleavedSetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    ) {
        sys::vDSP_DFT_Interleaved_Execute(setup.as_ptr(), input.cast(), output.cast())
    }

    unsafe fn dft_destroy_interleaved_setup(setup: ptr::NonNull<Self::DftInterleavedSetupStruct>) {
        sys::vDSP_DFT_Interleaved_DestroySetup(setup.as_ptr())
    }

    unsafe fn distance_squared(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        stride_b: sys::vDSP_Stride,
        c: *mut Self,
        n: sys::vDSP_Length,
    ) {
        sys::vDSP_distancesq(a, stride_a, b, stride_b, c, n)
    }

    unsafe fn dot_product(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        stride_b: sys::vDSP_Stride,
        c: *mut Self,
        n: sys::vDSP_Length,
    ) {
        sys::vDSP_dotpr(a, stride_a, b, stride_b, c, n)
    }

    unsafe fn vector_by_scalar_mul(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        c: *mut Self,
        stride_c: sys::vDSP_Stride,
        n: sys::vDSP_Length,
    ) {
        sys::vDSP_vsmul(a, stride_a, b, c, stride_c, n)
    }
}

unsafe impl Scalar for f64 {
    type DftInterleavedSetupStruct = sys::vDSP_DFT_Interleaved_SetupStructD;

    unsafe fn dft_create_interleaved_setup(
        previous: *mut Self::DftInterleavedSetupStruct,
        length: sys::vDSP_Length,
        direction: sys::vDSP_DFT_Direction,
        real_to_complex: sys::vDSP_DFT_RealtoComplex,
    ) -> *mut Self::DftInterleavedSetupStruct {
        sys::vDSP_DFT_Interleaved_CreateSetupD(previous, length, direction, real_to_complex)
    }

    unsafe fn dft_execute_interleaved_setup(
        setup: ptr::NonNull<Self::DftInterleavedSetupStruct>,
        input: *const Complex<Self>,
        output: *mut Complex<Self>,
    ) {
        sys::vDSP_DFT_Interleaved_ExecuteD(setup.as_ptr(), input.cast(), output.cast())
    }

    unsafe fn dft_destroy_interleaved_setup(setup: ptr::NonNull<Self::DftInterleavedSetupStruct>) {
        sys::vDSP_DFT_Interleaved_DestroySetupD(setup.as_ptr())
    }

    unsafe fn distance_squared(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        stride_b: sys::vDSP_Stride,
        c: *mut Self,
        n: sys::vDSP_Length,
    ) {
        sys::vDSP_distancesqD(a, stride_a, b, stride_b, c, n)
    }

    unsafe fn dot_product(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        stride_b: sys::vDSP_Stride,
        c: *mut Self,
        n: sys::vDSP_Length,
    ) {
        sys::vDSP_dotprD(a, stride_a, b, stride_b, c, n)
    }

    unsafe fn vector_by_scalar_mul(
        a: *const Self,
        stride_a: sys::vDSP_Stride,
        b: *const Self,
        c: *mut Self,
        stride_c: sys::vDSP_Stride,
        n: sys::vDSP_Length,
    ) {
        sys::vDSP_vsmulD(a, stride_a, b, c, stride_c, n)
    }
}
