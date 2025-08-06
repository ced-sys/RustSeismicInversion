///!Seismic wavelet generation
use ndarray::Array1;
use std::f64::consts::PI;

pub fn ricker_wavelet(freq: f64, dt: f64, n_samples: usize) -> Array1<f64>{
    let mut wavelet=Array1::zeros(n_samples);
    let t_center=(n_samples as f64 * dt) /2.0;

    for i in 0..n_samples{
        let t=i as f64 * dt-t_center;
        let arg=(PI * freq *t).powi(2);
        wavelet[i]=(1.0-2.0*arg)*(-arg).exp();
    }

    //Normalize the wavelet
    let max_val=wavelet.iter().fold(0.0f64, |acc, &x| acc.max(x.abs()));

    if max_val > 0.0{
        wavelet.mapv_inplace(|x| x /max_val);
    }

    println!("  Frequency: {} Hz", freq);
    println!("  Time sampling: {} s", dt);
    println!("  Amplitude normalized to max=1.0");

    wavelet
}
///Generate and Ormsby wavelet(bandpass)
pub fn ormsby_wavelet(f1: f64, f2: f64, f3: f64, f4: f64, dt: f64, n_samples: usize) -> Array1<f64> {
    //Implementation for Ormsby wavelet (bandpass filter)
    //This is more complex- placeholder for now
    println!("Ormsby wavelet: {}-{}-{}-{} Hz", f1, f2, f3, f4);
    ricker_wavelet((f2+f3)/2.0, dt, n_samples) //Simplified version
}

