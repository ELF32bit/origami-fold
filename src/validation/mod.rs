mod parents;
mod types;
mod length;
mod references;
mod reflexive;
mod winding;

use parents::ParentError;
use types::TypeError;
use length::LengthError;
use references::ReferencesError;
use reflexive::ReflexiveError;
use winding::WindingError;

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

pub use parents::validate_frame_parents;

pub use types::validate_vertices_coordinates;
pub use types::validate_edges_assignment_with_edges_fold_angle;
pub use types::validate_edges_length;
pub use types::validate_edge_orders;
pub use types::validate_faces_vertices;
pub use types::validate_face_orders;

pub use length::validate_vertices_vertices_length;
pub use length::validate_vertices_edges_length;
pub use length::validate_vertices_faces_length;
pub use length::validate_edges_faces_length;
pub use length::validate_edges_assignment_length;
pub use length::validate_edges_fold_angle_length;
pub use length::validate_edges_length_length;
pub use length::validate_faces_edges_length;
pub use length::validate_faces_faces_length;

pub use references::validate_vertices_vertices_indices;
pub use references::validate_vertices_edges_indices;
pub use references::validate_vertices_faces_indices;
pub use references::validate_edges_vertices_indices;
pub use references::validate_edges_faces_indices;
pub use references::validate_edge_orders_indices;
pub use references::validate_faces_vertices_indices;
pub use references::validate_faces_edges_indices;
pub use references::validate_faces_faces_indices;
pub use references::validate_face_orders_indices;

pub use reflexive::validate_vertices_vertices_with_vertices_vertices;
pub use reflexive::validate_vertices_edges_with_edges_vertices;
pub use reflexive::validate_vertices_faces_with_faces_vertices;
pub use reflexive::validate_edges_faces_with_faces_edges;
pub use reflexive::validate_faces_faces_with_faces_faces;

pub use winding::validate_vertices_vertices_and_vertices_edges_winding;
pub use winding::validate_vertices_vertices_and_vertices_faces_winding;
pub use winding::validate_vertices_edges_and_vertices_faces_winding;
pub use winding::validate_faces_vertices_and_faces_edges_winding;
pub use winding::validate_faces_vertices_and_faces_faces_winding;
pub use winding::validate_faces_edges_and_faces_faces_winding;