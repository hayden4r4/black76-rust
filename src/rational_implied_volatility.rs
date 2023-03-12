use crate::{Inputs, OptionType};
use libc::c_double;

#[link(name = "liblets_be_rational")]
extern "C" {
    // double price, double F, double K, double T, double q
    fn implied_volatility_from_a_transformed_rational_guess(
        price: c_double,
        F: c_double,
        K: c_double,
        T: c_double,
        q: c_double,
    ) -> c_double;
}

pub trait RationalImpliedVolatility {
    fn calc_rational_iv(&self) -> Result<f64, String>;
}

impl RationalImpliedVolatility for Inputs {
    fn calc_rational_iv(&self) -> Result<f64, String> {
        let p: c_double = match self.p {
            Some(p) => p.into(),
            None => return Err("Option price is required".to_string()),
        };
        let f: c_double = self.f.into();
        let k: c_double = self.k.into();
        let t: c_double = self.t.into();
        let q: c_double = match self.option_type {
            OptionType::Call => 1.0,
            OptionType::Put => -1.0,
        };
        let sigma = unsafe { implied_volatility_from_a_transformed_rational_guess(p, f, k, t, q) };
        Ok(sigma)
    }
}
