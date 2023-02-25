//! This library provides an simple, lightweight, and efficient (though not heavily optimized) implementation of the Black-Scholes-Merton model for pricing European options.
//!
//! Provides methods for pricing options, calculating implied volatility, and calculating the first, second, and third order Greeks.
//!
//! ### Example:
//! ```
//! use blackscholes::{Inputs, OptionType, Pricing};
//! let inputs = Inputs::new(OptionType::Call, 100.0, 100.0, None, 0.05, 0.2, 20.0/365.25, Some(0.2));
//! let price: f32 = inputs.calc_price().unwrap();
//! ```
//!
//! Criterion benchmark can be ran by running:
//! ```
//! cargo bench
//! ```
//!
//! See the [Github Repo](https://github.com/hayden4r4/black76-rust/tree/master) for full source code.  
//! See the [Documentation](https://docs.rs/black76/) for full documentation.

mod common;
mod constants;
mod greeks;
mod implied_volatility;
mod inputs;
mod pricing;

pub use greeks::Greeks;
pub use implied_volatility::ImpliedVolatility;
pub use inputs::{Inputs, OptionType};
pub use pricing::Pricing;
