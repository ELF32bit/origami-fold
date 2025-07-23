#[cfg(not(feature = "arbitrary_precision"))]
mod float;

#[cfg(not(feature = "arbitrary_precision"))]
pub use float::Real;

#[cfg(feature = "arbitrary_precision")]
mod real;

#[cfg(feature = "arbitrary_precision")]
pub use real::Real;