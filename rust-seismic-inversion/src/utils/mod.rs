///Utility functions for I/O and analysis

use ndarray::Array1;
use std::error::Error;
use std::fs::File;
use std::io::Write;


///Export array to CSV file for visualization
pub fn export_to_csv(data: &Array1<f64>, filename: &str)-> Result<(), Box<dyn Error>>{
    let mut file=File::create(filename)?;

    //Write header
    for(i, &value) in data.iter().enumerate(){
        writeln!(file, "{},{:.12}", i, value)?;
    }

    println!("  Exported {} sample to {}", data.len(), filename);

    Ok(())
}

///Create a simple ASCII plot (for quick visualization)
pub fn ascii_plot(data: &Array1<f64>, width: usize, height: usize){
    let min_val=data.iter().fold(f64::INFINITY, |acc, &x| acc.min(x));
    let max_val=data.iter().fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));
    let value_range=max_val-min_val;

    if value_range==0.0{
        println!("  All values are constant: {}", min_val);
        return;
    }

    println!("  ASCII Plot ({}x{}):", width, height);
    println!("  Range:{:.3} to {:.3}", min_val, max_val);

    //Simple downsampling
    let step=if data.len()> width{
        data.len()/width
    }else{
        1
    };

    for row in (0..height).rev(){
        let threshold=min_val+(row as f64/height as f64)* value_range;
        print!("  ");

        for i in (0..data.len()).step_by(step).take(width){
            if data[i] >=threshold {
                print!("*");
            }else{
                print!(" ");
            }
        }
        println!();
    }
}

