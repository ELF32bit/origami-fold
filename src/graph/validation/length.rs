use crate::fold::Graph;

#[derive(Clone, Copy, Debug)]
pub enum LengthError {
	VV,
	VE,
	VF,
	EF,
	EA,
	EFA,
	EL,
	FE,
	FF,
}

pub fn validate_vertices_vertices_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.vertices_vertices.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(LengthError::VV);
	}
	return Ok(());
}

pub fn validate_vertices_edges_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.vertices_edges.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(LengthError::VE);
	}
	return Ok(());
}

pub fn validate_vertices_faces_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.vertices_faces.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(LengthError::VF);
	}
	return Ok(());
}

pub fn validate_edges_faces_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.edges_faces.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(LengthError::EF);
	}
	return Ok(());
}

pub fn validate_edges_assignment_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.edges_assignment.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(LengthError::EA);
	}
	return Ok(());
}

pub fn validate_edges_fold_angle_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.edges_fold_angle.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(LengthError::EFA);
	}
	return Ok(());
}

pub fn validate_edges_length_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.edges_length.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(LengthError::EL);
	}
	return Ok(());
}

pub fn validate_faces_edges_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.faces_edges.len();
	if !(length == 0 || length == graph.faces_vertices.len()) {
		return Err(LengthError::FE);
	}
	return Ok(());
}

pub fn validate_faces_faces_length(graph: &Graph) -> Result<(), LengthError> {
	let length = graph.faces_faces.len();
	if !(length == 0 || length == graph.faces_vertices.len()) {
		return Err(LengthError::FF);
	}
	return Ok(());
}