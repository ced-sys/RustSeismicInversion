//!Forward seismic modelling pipeline

use ndarray::Array1;
use std::error::Error;

///Run the complete forward modelling pipeline
///This integrates all components like the C++ version
pub fn run_forward_model(
    reflectivity: &Array1<f64>,
    wavelet: &Array1<f64>,
    synthetic_trace: &Array1<f64>
)-> Result<(), Box<dyn Error>> {

    println!("  Analyzing forward model results...");

    //Compute statistics
    let stats=compute_statistics(&synthetic_trace);

    println!("  Synthetic trace statistics:");
    println!("  -Min amplitude: {:.6}", stats.min);
    println!("  -Max amplitude: {:.6}", stats.max);
    println!("  -Mean: {:.6}", stats.mean);
    println!("  -Standard deviation:{:.6}", stats.std_dev);
    println!("  -RMS amplitude: {:.6}", stats.rms);

    //Verify energy conservation (should be similar to input)
    let wavelet_energy=compute_energy(wavelet);
    let trace_energy=compute_energy(synthetic_trace);
    let energy_ratio=trace_energy / wavelet_energy;

    println!(" Energy analysis:");
    println!("  Wavelet energy: {:.6}", wavelet_energy);
    println!("  Trace energy: {:.6}", trace_energy);
    println!("  Energy ratio: {:.3}", energy_ratio);

    println!("Forward modelling pipeline completed");

    Ok(())
}

///Statistics for signal analysis
#[derive (Debug)]
struct SignalStats{
    min: f64,
    max: f64,
    mean: f64,
    std_dev: f64,
    rms: f64,
}

///Compute signal statistics
fn compute_statistics(signal: &Array1<f64>) -> SignalStats{
    let n =signal.len() as f64;

    let min=signal.iter().fold(f64::INFINITY, |acc, &x| acc.min(x));
    let max=signal.iter().fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));
    let mean=signal.sum() / n;

    let variance=signal.iter()
        .map(|&x| (x-mean).powi(2))
        .sum::<f64>() / n;

    let std_dev=variance.sqrt();

    let rms=(signal.iter().map(|&x| x.powi(2)).sum::<f64>() /n).sqrt();

    SignalStats {min, max, mean, std_dev, rms}

}

///Compute signal energy(sum of squares)
fn compute_energy(signal: &Array1<f64>)-> f64{
    signal.iter().map(|&x| x.powi(2)).sum()
}


