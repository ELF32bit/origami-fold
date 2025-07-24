#[cfg(feature = "real_is_f32")]
pub mod f32;

#[cfg(feature = "real_is_f32")]
pub use f32::Real;

#[cfg(not(any(feature = "real_is_f32", feature = "real_is_rug")))]
pub mod f64;

#[cfg(not(any(feature = "real_is_f32", feature = "real_is_rug")))]
pub use f64::Real;

#[cfg(feature = "real_is_rug")]
pub mod rug;

#[cfg(feature = "real_is_rug")]
pub use rug::Real;