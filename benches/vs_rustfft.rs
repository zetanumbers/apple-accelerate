#![feature(test)]
extern crate rustfft;
extern crate test;

use paste::paste;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::Fft;
use std::hint::black_box;
use std::sync::Arc;
use test::Bencher;

// Make fft using scalar planner
fn bench_scalar_32(b: &mut Bencher, len: usize) {
    let mut planner = rustfft::FftPlannerScalar::new();
    let fft: Arc<dyn Fft<f32>> = planner.plan_fft_forward(len);

    let mut buffer: Vec<Complex<f32>> = black_box(vec![Complex::zero(); len]);
    let mut scratch: Vec<Complex<f32>> =
        black_box(vec![Complex::zero(); fft.get_inplace_scratch_len()]);
    b.iter(|| {
        fft.process_with_scratch(&mut buffer, &mut scratch);
    });
}

// Make fft using scalar planner
fn bench_scalar_64(b: &mut Bencher, len: usize) {
    let mut planner = rustfft::FftPlannerScalar::new();
    let fft: Arc<dyn Fft<f64>> = planner.plan_fft_forward(len);

    let mut buffer: Vec<Complex<f64>> = black_box(vec![Complex::zero(); len]);
    let mut scratch: Vec<Complex<f64>> =
        black_box(vec![Complex::zero(); fft.get_inplace_scratch_len()]);
    b.iter(|| {
        fft.process_with_scratch(&mut buffer, &mut scratch);
    });
}

// Make fft using sse planner
fn bench_neon_32(b: &mut Bencher, len: usize) {
    let mut planner = rustfft::FftPlannerNeon::new().unwrap();
    let fft: Arc<dyn Fft<f32>> = planner.plan_fft_forward(len);

    let mut buffer: Vec<Complex<f32>> = black_box(vec![Complex::zero(); len]);
    let mut scratch: Vec<Complex<f32>> =
        black_box(vec![Complex::zero(); fft.get_inplace_scratch_len()]);
    b.iter(|| {
        fft.process_with_scratch(&mut buffer, &mut scratch);
    });
}

// Make fft using sse planner
fn bench_neon_64(b: &mut Bencher, len: usize) {
    let mut planner = rustfft::FftPlannerNeon::new().unwrap();
    let fft: Arc<dyn Fft<f64>> = planner.plan_fft_forward(len);

    let mut buffer: Vec<Complex<f64>> = black_box(vec![Complex::zero(); len]);
    let mut scratch: Vec<Complex<f64>> =
        black_box(vec![Complex::zero(); fft.get_inplace_scratch_len()]);
    b.iter(|| {
        fft.process_with_scratch(&mut buffer, &mut scratch);
    });
}

fn bench_accelerate_32(b: &mut Bencher, len: usize) {
    let setup =
        apple_accelerate_dft::DftSetup::new(len, apple_accelerate_dft::Direction::Forward).unwrap();

    let mut buffer: Vec<Complex<f32>> = black_box(vec![Complex::zero(); len]);
    // Warmup
    setup.execute_in_place(&mut buffer);

    b.iter(|| {
        setup.execute_in_place(&mut buffer);
    });
}

fn bench_accelerate_64(b: &mut Bencher, len: usize) {
    let setup =
        apple_accelerate_dft::DftSetup::new(len, apple_accelerate_dft::Direction::Forward).unwrap();

    let mut buffer: Vec<Complex<f64>> = black_box(vec![Complex::zero(); len]);
    // Warmup
    setup.execute_in_place(&mut buffer);

    b.iter(|| {
        setup.execute_in_place(&mut buffer);
    });
}

// Create benches using functions taking one argument
macro_rules! make_benches {
    ($name:ident, [ $($len:literal),* ]) => {
        paste! {
            $(
                #[bench]
                fn [<$name _ $len _f32_scalar>](b: &mut Bencher)  {
                    bench_scalar_32(b, $len);
                }

                #[bench]
                fn [<$name _ $len _f64_scalar>](b: &mut Bencher)  {
                    bench_scalar_64(b, $len);
                }

                #[bench]
                fn [<$name _ $len _f32_neon>](b: &mut Bencher)  {
                    bench_neon_32(b, $len);
                }

                #[bench]
                fn [<$name _ $len _f64_neon>](b: &mut Bencher)  {
                    bench_neon_64(b, $len);
                }

                #[bench]
                fn [<$name _ $len _f32_accelerate>](b: &mut Bencher)  {
                    bench_accelerate_32(b, $len);
                }

                #[bench]
                fn [<$name _ $len _f64_accelerate>](b: &mut Bencher)  {
                    bench_accelerate_64(b, $len);
                }
            )*
        }
    }
}

make_benches!(dft, [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096]);
make_benches!(dft, [12, 24, 48, 96, 192, 384, 768]);
make_benches!(dft, [20, 40, 80, 160, 320, 640]);
make_benches!(dft, [36, 72, 144, 288, 576, 1152]);
make_benches!(dft, [60, 120, 240, 480, 960, 1920]);
