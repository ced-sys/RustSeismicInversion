//!Rust Seismic Inversion tool
mod forward_modelling;
mod convolution;
mod models;
mod wavelets;
mod utils;

use std::error::Error;
use ndarray::Array1;

fn main()->Result<(), Box<dyn Error>> {
    println!("Rust Seismic Inversion Tool Starting...");
    println!("Initializing forward modelling pipeline...");

    //Step 1: Create reflectivity model
    println!("\nStep 1: Defining reflectivity model...");
    let reflectivity=models::create_layered_reflectivity(100, &[0.1, -0.05, 0.15, -0.08]);
    println!("  Created reflectivity series with {} samples", reflectivity.len());

    //Step 2: Generate Ricker wavelet
    println!("\nStep2: Generating Ricker wavelet...");
    let dt=0.001; //1ms sampling
    let freq=30.0;
    let wavelet =wavelets::ricker_wavelet(freq, dt, 200);
    println!("  Generated {} Hz Ricker wavelet with {} samples", freq, wavelet.len());

    //Step 3: Convolve using FFt
    println!("\nStep 3: FFt-based convolution...");
    let synthetic_trace=convolution::fft_convolve(&reflectivity, &wavelet)?;
    println!("  Convolution completed, output has {} samples", synthetic_trace.len());

    //Step 4: Run forward modelling pipeline
    println!("\nStep 4: Forward modelling pipeline...");
    forward_modelling::run_forward_model(&reflectivity, &wavelet, &synthetic_trace)?;

    //Step 5: Export results
    println!("\nStep 5: Exportng results...");
    utils::export_to_csv(&synthetic_trace, "synthetic_trace.csv")?;
    utils::export_to_csv(&reflectivity, "reflectivity.csv")?;
    utils::export_to_csv(&wavelet, "wavelet.csv")?;

    println!("\n Seismic forward modelling completed successfully!");
    println!("Results exported to CSV files for visualization");

    Ok(())
}

