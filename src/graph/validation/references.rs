use crate::fold::Graph;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum ReferencesError {
	VV,
	VE,
	VF,
	EV,
	EF,
	EO,
	FV,
	FE,
	FF,
	FO,
}

pub fn validate_vertices_vertices_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.vertices_coordinates.len();
	for vertex_vertices in graph.vertices_vertices.iter() {
		for &index in vertex_vertices.iter() {
			if index >= max_index {
				return Err(ReferencesError::VV);
			}
		}
	}
	return Ok(());
}

pub fn validate_vertices_edges_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.edges_vertices.len();
	for vertex_edges in graph.vertices_edges.iter() {
		for &index in vertex_edges.iter() {
			if index >= max_index {
				return Err(ReferencesError::VE);
			}
		}
	}
	return Ok(());
}

pub fn validate_vertices_faces_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.faces_vertices.len();
	for vertex_faces in graph.vertices_faces.iter() {
		for &index in vertex_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(ReferencesError::VF);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_edges_vertices_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.vertices_coordinates.len();
	for edge_vertices in graph.edges_vertices.iter() {
		if edge_vertices.0 >= max_index || edge_vertices.1 >= max_index {
				return Err(ReferencesError::EV);
		}
	}
	return Ok(());
}

pub fn validate_edges_faces_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.faces_vertices.len();
	for edge_faces in graph.edges_faces.iter() {
		for &index in edge_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(ReferencesError::EF);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_edge_orders_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let mut edge_pairs: HashSet<(usize, usize)> = HashSet::new();
	let max_index = graph.edges_vertices.len();
	for edge_order in graph.edge_orders.iter() {
		if edge_pairs.contains(&(edge_order.0, edge_order.1)) {
			return Err(ReferencesError::EO);
		}
		else if edge_order.0 == edge_order.1 {
			return Err(ReferencesError::EO);
		}
		else if edge_order.0 >= max_index || edge_order.1 >= max_index {
			return Err(ReferencesError::EO);
		}
		edge_pairs.insert((edge_order.0, edge_order.1));
		edge_pairs.insert((edge_order.1, edge_order.0));
	}
	return Ok(());
}

pub fn validate_faces_vertices_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.vertices_coordinates.len();
	for face_vertices in graph.faces_vertices.iter() {
		for &index in face_vertices.iter() {
			if index >= max_index {
				return Err(ReferencesError::FV);
			}
		}
	}
	return Ok(());
}

pub fn validate_faces_edges_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.edges_vertices.len();
	for face_edges in graph.faces_edges.iter() {
		for &index in face_edges.iter() {
			if index >= max_index {
				return Err(ReferencesError::FE);
			}
		}
	}
	return Ok(());
}

pub fn validate_faces_faces_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.faces_vertices.len();
	for face_faces in graph.faces_faces.iter() {
		for &index in face_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(ReferencesError::FF);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_face_orders_indices(graph: &Graph) -> Result<(), ReferencesError> {
	let max_index = graph.faces_vertices.len();
	for face_order in graph.face_orders.iter() {
		if face_order.0 >= max_index || face_order.1 >= max_index {
			return Err(ReferencesError::FO);
		}
	}
	return Ok(());
}
