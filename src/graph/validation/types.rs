use crate::fold::Graph;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum TypeError {
	VC(usize, usize),
	EL(usize),
	EO(usize),
	FV(usize),
	FO(usize),
}

pub fn validate_vertices_coordinates(graph: &Graph) -> Result<(), TypeError> {
	if !(graph.vertices_coordinates.len() > 0) { return Ok(()); }
	let expected_coordinates_length = graph.vertices_coordinates[0].len();
	for (vertex_index, vertex_coordinates) in graph.vertices_coordinates.iter().enumerate() {
		if vertex_coordinates.len() != expected_coordinates_length {
			return Err(TypeError::VC(vertex_index, expected_coordinates_length));
		}
	}
	return Ok(());
}

pub fn validate_edges_length(graph: &Graph) -> Result<(), TypeError> {
	for (edge_index, edge_length) in graph.edges_length.iter().enumerate() {
		if *edge_length < 0.0 { return Err(TypeError::EL(edge_index)); }
	}
	return Ok(());
}

pub fn validate_edge_orders(graph: &Graph) -> Result<(), TypeError> {
	let mut edge_pairs: HashSet<(usize, usize)> = HashSet::new();
	for (edge_index, edge_order) in graph.face_orders.iter().enumerate() {
		if edge_order.0 == edge_order.1 {
			return Err(TypeError::EO(edge_index));
		}
		if edge_pairs.contains(&(edge_order.0, edge_order.1)) {
			return Err(TypeError::FO(edge_index));
		}
		edge_pairs.insert((edge_order.0, edge_order.1));
		edge_pairs.insert((edge_order.1, edge_order.0));
	}
	return Ok(());
}

pub fn validate_faces_vertices(graph: &Graph) -> Result<(), TypeError> {
	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		if face_vertices.len() < 3 {
			return Err(TypeError::FV(face_index));
		}
	}
	return Ok(());
}

pub fn validate_face_orders(graph: &Graph) -> Result<(), TypeError> {
	let mut face_pairs: HashSet<(usize, usize)> = HashSet::new();
	for (face_index, face_order) in graph.face_orders.iter().enumerate() {
		if face_order.0 == face_order.1 {
			return Err(TypeError::FO(face_index));
		}
		if face_pairs.contains(&(face_order.0, face_order.1)) {
			return Err(TypeError::FO(face_index));
		}
		face_pairs.insert((face_order.0, face_order.1));
		face_pairs.insert((face_order.1, face_order.0));
	}
	return Ok(());
}