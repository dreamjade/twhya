use numpy::{PyReadonlyArray1, PyArray1};
use pyo3::prelude::*;
use rand::Rng;
use pyo3::wrap_pyfunction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::collections::HashMap;

#[pyfunction]
fn bootstrap_core(
    py: Python,
    valid_pixels: PyReadonlyArray1<f64>,
    valid_radii_integers: PyReadonlyArray1<i32>,
    ring_errors: PyReadonlyArray1<f64>,
    n_bootstrap: usize,
    deconv: bool,
) -> Py<PyArray1<f64>> {
    // Read NumPy arrays into Rust slices (zero-copy)
    let pixels = valid_pixels.as_slice().unwrap();
    let radii = valid_radii_integers.as_slice().unwrap();
    let errors = ring_errors.as_slice().unwrap();

    // Group pixels by their integer ring radius.
    let mut rings: HashMap<i32, Vec<f64>> = HashMap::new();
    for (i, &radius) in radii.iter().enumerate() {
        rings.entry(radius).or_default().push(pixels[i]);
    }

    // Convert HashMap to a Vec for parallel iteration
    let ring_pixel_groups: Vec<(i32, Vec<f64>)> = rings.into_iter().collect();

    // The core of the bootstrap, parallelized with Rayon.
    // We create a parallel iterator from 0 to n_bootstrap. Each iteration
    // calculates the total flux for one bootstrap sample.
    let bootstrap_sums: Vec<f64> = (0..n_bootstrap)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let mut total_flux_for_sample = 0.0;

            // Iterate over each group of pixels (each ring)
            for (radius, pixels_in_ring) in &ring_pixel_groups {
                let n_pixels = pixels_in_ring.len();
                if n_pixels == 0 {
                    continue;
                }
                
                // For this sample, calculate the sum of resampled pixels in this ring.
                // This is equivalent to np.random.randint + indexing + np.sum.
                let ring_sum: f64 = (0..n_pixels)
                    .map(|_| *pixels_in_ring.choose(&mut rng).unwrap())
                    .sum();
                
                if deconv {
                    let err = if *radius >= 0 && (*radius as usize) < errors.len() { errors[*radius as usize] } else { 1.0 };
                    let f = if err > 0.0 { rng.gen_range(-err..err) } else { 0.0 };
                    total_flux_for_sample += ring_sum * (1.0 + f);
                } else {
                    total_flux_for_sample += ring_sum;
                }
            }
            total_flux_for_sample
        })
        .collect();

    // Return the results as a new NumPy array
    PyArray1::from_vec_bound(py, bootstrap_sums).unbind()
}

// This function defines the Python module.
#[pymodule]
fn rust_bootstrap(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bootstrap_core, m)?)?;
    Ok(())
}