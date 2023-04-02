use crate::{common::*, constants::*, Inputs, OptionType};
use num_traits::Float;
pub trait Pricing<T>
where
    T: Float,
{
    fn calc_price(&self) -> Result<T, String>;
}

impl Pricing<f32> for Inputs {
    /// Calculates the price of the option.
    /// # Requires
    /// f, k, r, q, t, sigma.
    /// # Returns
    /// f32 of the price of the option.
    /// # Example
    /// ```
    /// use black76::{Inputs, OptionType, Pricing};
    /// let inputs = Inputs::new(OptionType::Call, 100.0, 100.0, None, 0.05, 20.0/365.25, Some(0.2));
    /// let price = inputs.calc_price().unwrap();
    /// ```
    fn calc_price(&self) -> Result<f32, String> {
        // Calculates the price of the option
        let (nd1, nd2): (f32, f32) = calc_nd1nd2(&self)?;
        let (f, k) = if self.shifted {
            calc_shifted_f_k(&self)
        } else {
            (self.f, self.k)
        };
        let price: f32 = match self.option_type {
            OptionType::Call => f32::max(0.0, E.powf(-self.r * self.t) * (nd1 * f - nd2 * k)),
            OptionType::Put => f32::max(0.0, E.powf(-self.r * self.t) * (nd2 * k - nd1 * f)),
        };
        Ok(price)
    }
}
