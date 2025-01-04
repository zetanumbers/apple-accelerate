#![allow(non_camel_case_types, non_upper_case_globals)]

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {}

pub type vDSP_Length = ::std::os::raw::c_ulong;
pub type vDSP_Stride = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DSPComplex {
    pub real: f32,
    pub imag: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DSPDoubleComplex {
    pub real: f64,
    pub imag: f64,
}

unsafe extern "C" {
    pub fn vDSP_distancesq(
        __A: *const f32,
        __IA: vDSP_Stride,
        __B: *const f32,
        __IB: vDSP_Stride,
        __C: *mut f32,
        __N: vDSP_Length,
    );
}
unsafe extern "C" {
    pub fn vDSP_distancesqD(
        __A: *const f64,
        __IA: vDSP_Stride,
        __B: *const f64,
        __IB: vDSP_Stride,
        __C: *mut f64,
        __N: vDSP_Length,
    );
}
unsafe extern "C" {
    pub fn vDSP_dotpr(
        __A: *const f32,
        __IA: vDSP_Stride,
        __B: *const f32,
        __IB: vDSP_Stride,
        __C: *mut f32,
        __N: vDSP_Length,
    );
}
unsafe extern "C" {
    pub fn vDSP_dotprD(
        __A: *const f64,
        __IA: vDSP_Stride,
        __B: *const f64,
        __IB: vDSP_Stride,
        __C: *mut f64,
        __N: vDSP_Length,
    );
}
unsafe extern "C" {
    pub fn vDSP_vsmul(
        __A: *const f32,
        __IA: vDSP_Stride,
        __B: *const f32,
        __C: *mut f32,
        __IC: vDSP_Stride,
        __N: vDSP_Length,
    );
}
unsafe extern "C" {
    pub fn vDSP_vsmulD(
        __A: *const f64,
        __IA: vDSP_Stride,
        __B: *const f64,
        __C: *mut f64,
        __IC: vDSP_Stride,
        __N: vDSP_Length,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vDSP_DFT_Interleaved_SetupStruct {
    _unused: [u8; 0],
}
pub type vDSP_DFT_Interleaved_Setup = *mut vDSP_DFT_Interleaved_SetupStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vDSP_DFT_Interleaved_SetupStructD {
    _unused: [u8; 0],
}
pub type vDSP_DFT_Interleaved_SetupD = *mut vDSP_DFT_Interleaved_SetupStructD;

pub type vDSP_DFT_Direction = ::std::os::raw::c_int;
pub const FFT_FORWARD: vDSP_DFT_Direction = 1;
pub const FFT_INVERSE: vDSP_DFT_Direction = -1;
pub type vDSP_DFT_RealtoComplex = bool;

unsafe extern "C" {
    pub fn vDSP_DFT_Interleaved_CreateSetup(
        Previous: vDSP_DFT_Interleaved_Setup,
        Length: vDSP_Length,
        Direction: vDSP_DFT_Direction,
        RealtoComplex: vDSP_DFT_RealtoComplex,
    ) -> vDSP_DFT_Interleaved_Setup;
}
unsafe extern "C" {
    pub fn vDSP_DFT_Interleaved_CreateSetupD(
        Previous: vDSP_DFT_Interleaved_SetupD,
        Length: vDSP_Length,
        Direction: vDSP_DFT_Direction,
        RealtoComplex: vDSP_DFT_RealtoComplex,
    ) -> vDSP_DFT_Interleaved_SetupD;
}
unsafe extern "C" {
    pub fn vDSP_DFT_Interleaved_Execute(
        Setup: vDSP_DFT_Interleaved_Setup,
        Iri: *const DSPComplex,
        Ori: *mut DSPComplex,
    );
}
unsafe extern "C" {
    pub fn vDSP_DFT_Interleaved_ExecuteD(
        Setup: vDSP_DFT_Interleaved_SetupD,
        Iri: *const DSPDoubleComplex,
        Ori: *mut DSPDoubleComplex,
    );
}
unsafe extern "C" {
    pub fn vDSP_DFT_Interleaved_DestroySetup(Setup: vDSP_DFT_Interleaved_Setup);
}
unsafe extern "C" {
    pub fn vDSP_DFT_Interleaved_DestroySetupD(Setup: vDSP_DFT_Interleaved_SetupD);
}
