#[cfg(not(feature = "real_is_rug"))]
pub mod f64;

#[cfg(not(feature = "real_is_rug"))]
pub use f64::Real;

#[cfg(feature = "real_is_rug")]
pub mod rug;

#[cfg(feature = "real_is_rug")]
pub use rug::Real;