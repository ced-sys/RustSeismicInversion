//!Seimic reflectivity models
use ndarray::Array1;

///Create a layered reflectivity model
pub fn create_layered_reflectivity(n_samples: usize, coefficients: &[f64])-> Array1<f64> {
    let mut reflectivity=Array1::zeros(n_samples);

    //Place reflectivity coefficients at regular intervals
    let layer_spacing=n_samples/ (coefficients.len()+1);

    for (i, &coeff) in coefficients.iter().enumerate(){
        let position=(i+1)* layer_spacing;
        if position <n_samples{
            reflectivity[position]=coeff;
        }
    }

    println!("  Layer spacing: {} samples", layer_spacing);
    println!("  Reflectivity coefficients: {:?}", coefficients);

    reflectivity
}

/// Create a random reflectivity series(for testing)
pub fn create_random_reflectivity(n_samples: usize, density: f64) -> Array1<f64> {
    use rand::Rng;

    let mut rng=rand::thread_rng();
    let mut reflectivity=Array1::zeros(n_samples);
    
    for i in 0..n_samples{
        if rng.r#gen::<f64>() < density{
            reflectivity[i]=rng.gen_range(-0.2..0.2);
        }
    }

    reflectivity
}

