//!FFT-based convolution engine
//! Equivalent to the FFTW implementation

use ndarray::Array1;
use rustfft::{FftPlanner, num_complex::Complex64};
use num_complex::Complex;
use std::error::Error;

/// Perform FFt-Based convolution of two real signals
/// This is equivalent to the C++ convolve() function
pub fn fft_convolve(signal1: &Array1<f64>, signal2: &Array1<f64>) -> Result<Array1<f64>, Box<dyn Error>> {

    //Determine output length (same as the C++ logic)
    let len1=signal1.len();
    let len2=signal2.len();
    let output_len=len1+len2-1;

    //Find next power of 2 for FFT efficiency
    let fft_len=next_power_of_2(output_len);

    println!("  Input lengths: {} +{} samples", len1, len2);
    println!("  Output length: {} samples", output_len);
    println!("  FFT length: {} samples (padded)", fft_len);

    //Create FFT planner
    let mut planner=FftPlanner::new();
    let fft=planner.plan_fft_forward(fft_len);
    let ifft=planner.plan_fft_inverse(fft_len);

    //Convert signals to complex and pad with zeros
    let mut signal1_complex=to_complex_padded(signal1, fft_len);
    let mut signal2_complex=to_complex_padded(signal2, fft_len);

    //Forward FFT
    fft.process(&mut signal1_complex);
    fft.process(&mut signal2_complex);

    //Multiply in frequency domains
    for i in 0..fft_len{
        signal1_complex[i]*=signal2_complex[i];
    }

    //Inverse FFT
    ifft.process(&mut signal1_complex);

    //Extract real part and normalize
    let mut result=Array1::zeros(output_len);
    for i in 0..output_len{
        result[i]=signal1_complex[i].re / fft_len as f64;
    }

    println!("  FFt convolution completed");

    Ok(result)
}

/// Convert real signal to complext with zero padding
fn to_complex_padded(signal: &Array1<f64>, target_len: usize)-> Vec<Complex64> {
    let mut complex_signal=vec![Complex::new(0.0, 0.0); target_len];

    for (i, &val) in signal.iter().enumerate(){
        if i<target_len{
            complex_signal[i]=Complex::new(val, 0.0);
        }
    }

    complex_signal
}

/// Find next power of 2 (for FFT Efficiency)
fn next_power_of_2(n: usize) -> usize{
    if n<=1{
        return 1;
    }
    let mut power=1;
    while power < n{
        power <<=1;
    }
    power
}

#[cfg(test)]
mod tests{
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_next_power_of_2() {
        assert_eq!(next_power_of_2(1), 1);
        assert_eq!(next_power_of_2(5), 8);
        assert_eq!(next_power_of_2(16), 16);
        assert_eq!(next_power_of_2(17), 32);
    }

    #[test]
    fn test_fft_convolve(){
        let signal1=Array1::from(vec![1.0, 2.0, 3.0]);
        let signal2=Array1::from(vec![0.5, 1.0]);

        let result=fft_convolve(&signal1, &signal2).unwrap();

        //Expeted result of convolution
        let expected=vec![0.5, 1.5, 2.5, 3.0];

        for (i, &expected_val) in expected.iter().enumerate(){
            assert_abs_diff_eq!(result[i], expected_val, epsilon=1e-10);
        }
    }
}
