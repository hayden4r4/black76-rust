use crate::{constants::*, Inputs, OptionType};
use num_traits::NumCast;
use statrs::distribution::{ContinuousCDF, Normal};

/// Calculates the d1 and d2 values for the option.
/// # Requires
/// f, k, r, q, t, sigma.
/// # Returns
/// Tuple (f32, f32) of (d1, d2)
pub fn calc_d1d2(inputs: &Inputs) -> Result<(f32, f32), String> {
    let sigma = inputs
        .sigma
        .ok_or("Expected Some(f32) for self.sigma, received None")?;
    let (f, k) = if inputs.shifted {
        calc_shifted_f_k(&inputs)
    } else {
        (inputs.f, inputs.k)
    };

    // Calculating numerator of d1
    let numd1 = (f / k).ln() + (sigma.powi(2) / 2.0) * inputs.t;

    // Calculating denominator of d1 and d2
    let den = sigma * (inputs.t.sqrt());

    let d1 = numd1 / den;
    let d2 = d1 - den;

    Ok((d1, d2))
}

/// Calculates the nd1 and nd2 values for the option.
/// # Requires
/// f, k, r, q, t, sigma
/// # Returns
/// Tuple (f32, f32) of (nd1, nd2)
pub fn calc_nd1nd2(inputs: &Inputs) -> Result<(f32, f32), String> {
    let nd1nd2 = {
        let d1d2 = calc_d1d2(&inputs)?;

        let n: Normal = Normal::new(N_MEAN as f64, N_STD_DEV as f64).unwrap();

        let num_cast_err: String = "Failed to cast f64 to f32".into();
        // Calculates the nd1 and nd2 values
        // Checks if OptionType is Call or Put
        match inputs.option_type {
            OptionType::Call => (
                NumCast::from(n.cdf(NumCast::from(d1d2.0).ok_or(&num_cast_err)?))
                    .ok_or(&num_cast_err)?,
                NumCast::from(n.cdf(NumCast::from(d1d2.1).ok_or(&num_cast_err)?))
                    .ok_or(&num_cast_err)?,
            ),
            OptionType::Put => (
                NumCast::from(n.cdf(NumCast::from(-d1d2.0).ok_or(&num_cast_err)?))
                    .ok_or(&num_cast_err)?,
                NumCast::from(n.cdf(NumCast::from(-d1d2.1).ok_or(&num_cast_err)?))
                    .ok_or(&num_cast_err)?,
            ),
        }
    };
    Ok(nd1nd2)
}

/// Calculates the n probability density function (PDF) for the given input.
/// # Returns
/// f32 of the value of the n probability density function.
pub fn calc_npdf(x: f32) -> f32 {
    let d: f32 = (x - N_MEAN) / N_STD_DEV;
    (-HALF * d.powi(2)).exp() / (SQRT_2PI * N_STD_DEV)
}

/// # Returns
/// f32 of the derivative of the nd1.
pub fn calc_nprimed1(inputs: &Inputs) -> Result<f32, String> {
    let (d1, _) = calc_d1d2(&inputs)?;

    // Get the standard n probability density function value of d1
    let nprimed1 = calc_npdf(d1);
    Ok(nprimed1)
}

/// # Returns
/// f32 of the derivative of the nd2.
pub fn calc_nprimed2(inputs: &Inputs) -> Result<f32, String> {
    let (_, d2) = calc_d1d2(&inputs)?;

    // Get the standard n probability density function value of d1
    let nprimed2 = calc_npdf(d2);
    Ok(nprimed2)
}

/// Calculates the shifted future price and strike price.
/// Implemented from [Pricing Interest Rate Derivatives in a Negative Yield Environment (2017) by Lavinia Rognone](https://core.ac.uk/download/pdf/85145501.pdf)
/// # Notes
/// - Shift (Θ) > 0
/// - Future Price (F) + Shift (Θ) > 0
/// - Strike Price (K) + Shift (Θ) > 0
/// - Risk Free Rate (R) + Shift (Θ) > 0
///
/// Shift in this implementation is calculated as:
/// Θ = min(F, K, R) + shift_factor
/// Where `shift_factor` is an arbitrary value (0.01 here).
///
/// # Returns
/// Tuple (f32, f32) of (shifted_f, shifted_k)
pub fn calc_shifted_f_k(inputs: &Inputs) -> (f32, f32) {
    // Get the minimum of the future price, strike price, and risk free rate
    let min = f32::min(inputs.f, inputs.k).min(inputs.r);
    // Arbitrary shift factor (should this be a parameter?)
    let shift_factor = 0.01;

    let shift = if min < 0.0 {
        min.abs() + shift_factor
    } else {
        0.0
    };

    // Add Shifts to both future price and strike price
    let f = inputs.f + shift;
    let k = inputs.k + shift;

    (f, k)
}
