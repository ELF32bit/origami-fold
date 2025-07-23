mod parents;

pub use parents::ParentError;
pub use parents::validate_frame_parents;

use crate::graph::validation::TypeError;
use crate::graph::validation::LengthError;
use crate::graph::validation::ReferencesError;
use crate::graph::validation::ReflexiveError;
use crate::graph::validation::WindingError;

#[derive(Clone, Copy, Debug)]
pub enum Error {
	ParentError(ParentError),
	TypeError(TypeError),
	LengthError(LengthError),
	ReferencesError(ReferencesError),
	ReflexiveError(ReflexiveError),
	WindingError(WindingError),
}

impl From<ParentError> for Error {
	fn from(error: ParentError) -> Self { Error::ParentError(error) }
}

impl From<TypeError> for Error {
	fn from(error: TypeError) -> Self { Error::TypeError(error) }
}


impl From<LengthError> for Error {
	fn from(error: LengthError) -> Self { Error::LengthError(error) }
}

impl From<ReferencesError> for Error {
	fn from(error: ReferencesError) -> Self { Error::ReferencesError(error) }
}

impl From<ReflexiveError> for Error {
	fn from(error: ReflexiveError) -> Self { Error::ReflexiveError(error) }
}

impl From<WindingError> for Error {
	fn from(error: WindingError) -> Self { Error::WindingError(error) }
}