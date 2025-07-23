use crate::fold::Graph;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum WindingError {
	VVVE,
	VVVF,
	VEVF,
	FVFE,
	FVFF,
	FEFF,
}

pub fn validate_vertices_vertices_and_vertices_edges_winding(graph: &Graph) -> Result<(), WindingError> {
	if graph.vertices_vertices.len() == 0 { return Ok(()); }
	if graph.vertices_edges.len() == 0 { return Ok(()); }
	if graph.edges_vertices.len() == 0 { return Ok(()); }

	for (vertex_index, vertex_vertices) in graph.vertices_vertices.iter().enumerate() {
		for (index, &vertex_vertex_index) in vertex_vertices.iter().enumerate() {
			let vertex_edge_index = graph.vertices_edges[vertex_index][index];
			let vertex_edge = graph.edges_vertices[vertex_edge_index];

			if vertex_edge == (vertex_index, vertex_vertex_index) { continue; }
			else if vertex_edge == (vertex_vertex_index, vertex_index) { continue; }
			else { return Err(WindingError::VVVE); }
		}
	}
	return Ok(());
}

pub fn validate_vertices_vertices_and_vertices_faces_winding(graph: &Graph) -> Result<(), WindingError> {
	if graph.vertices_vertices.len() == 0 { return Ok(()); }
	if graph.vertices_faces.len() == 0 { return Ok(()); }
	if graph.faces_vertices.len() == 0 { return Ok(()); }

	let mut faces_edges: Vec<HashSet<(usize, usize)>> = Vec::new();
	faces_edges.resize(graph.faces_vertices.len(), HashSet::new());
	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		let d = face_vertices.len();
		for (index, &face_vertex_index) in face_vertices.iter().enumerate() {
			let next_face_vertex_index = face_vertices[(index + 1) % d];
			faces_edges[face_index].insert((face_vertex_index, next_face_vertex_index));
			faces_edges[face_index].insert((next_face_vertex_index, face_vertex_index));
		}
	}

	for (vertex_index, vertex_vertices) in graph.vertices_vertices.iter().enumerate() {
		let d = vertex_vertices.len();
		for index in 0..d {
			let vertex_vertex_index = vertex_vertices[index];
			let vertex_next_vertex_index = vertex_vertices[(index + 1) % d];
			let vertex_edge = (vertex_vertex_index, vertex_next_vertex_index);
			let vertex_face_option = graph.vertices_faces[vertex_index][index];

			match vertex_face_option {
				Some(vertex_face_index) => {
					let vertex_face_edges = &faces_edges[vertex_face_index];
					if !vertex_face_edges.contains(&vertex_edge) {
						return Err(WindingError::VVVE);
					}
				}
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_edges_and_vertices_faces_winding(graph: &Graph) -> Result<(), WindingError> {
	if graph.vertices_edges.len() == 0 { return Ok(()); }
	if graph.vertices_faces.len() == 0 { return Ok(()); }
	if graph.faces_vertices.len() == 0 { return Ok(()); }

	let mut faces_edges_pairs: Vec<HashSet<(usize, usize)>> = Vec::new();
	faces_edges_pairs.resize(graph.faces_vertices.len(), HashSet::new());
	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		let d = face_vertices.len();
		for (index, &face_vertex_index) in face_vertices.iter().enumerate() {
			let next_face_vertex_index = face_vertices[(index + 1) % d];
			faces_edges_pairs[face_index].insert((face_vertex_index, next_face_vertex_index));
			faces_edges_pairs[face_index].insert((next_face_vertex_index, face_vertex_index));
		}
	}

	for (vertex_index, vertex_edges) in graph.vertices_edges.iter().enumerate() {
		let d = vertex_edges.len();
		for index in 0..d {
			let vertex_edge_index = vertex_edges[index];
			let vertex_next_edge_index = vertex_edges[(index + 1) % d];
			let vertex_edges_pair = (vertex_edge_index, vertex_next_edge_index);
			let vertex_face_option = graph.vertices_faces[vertex_index][index];

			match vertex_face_option {
				Some(vertex_face_index) => {
					let vertex_face_edges_pairs = &faces_edges_pairs[vertex_face_index];
					if !vertex_face_edges_pairs.contains(&vertex_edges_pair) {
						return Err(WindingError::VEVF);
					}
				}
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_vertices_and_faces_edges_winding(graph: &Graph) -> Result<(), WindingError> {
	if graph.faces_vertices.len() == 0 { return Ok(()); }
	if graph.faces_edges.len() == 0 { return Ok(()); }
	if graph.edges_vertices.len() == 0 { return Ok(()); }

	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		let d = face_vertices.len();
		for (index, &face_vertex_index) in face_vertices.iter().enumerate() {
			let face_next_vertex_index = face_vertices[(index + 1) % d];
			let face_edge_index = graph.faces_edges[face_index][index];
			let face_edge = graph.edges_vertices[face_edge_index];

			if face_edge == (face_vertex_index, face_next_vertex_index) { continue; }
			else if face_edge == (face_next_vertex_index, face_vertex_index) { continue; }
			else { return Err(WindingError::FVFE); }
		}
	}

	return Ok(());
}

pub fn validate_faces_vertices_and_faces_faces_winding(graph: &Graph) -> Result<(), WindingError> {
	if graph.faces_vertices.len() == 0 { return Ok(()); }
	if graph.faces_faces.len() == 0 { return Ok(()); }

	let mut faces_edges: Vec<HashSet<(usize, usize)>> = Vec::new();
	faces_edges.resize(graph.faces_vertices.len(), HashSet::new());
	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		let d = face_vertices.len();
		for (index, &face_vertex_index) in face_vertices.iter().enumerate() {
			let next_face_vertex_index = face_vertices[(index + 1) % d];
			faces_edges[face_index].insert((face_vertex_index, next_face_vertex_index));
			faces_edges[face_index].insert((next_face_vertex_index, face_vertex_index));
		}
	}

	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		let d = face_vertices.len();
		for index in 0..d {
			let face_vertex_index = face_vertices[index];
			let face_next_vertex_index = face_vertices[(index + 1) % d];
			let face_edge = (face_vertex_index, face_next_vertex_index);
			let face_face_option = graph.faces_faces[face_index][index];

			match face_face_option {
				Some(face_face_index) => {
					let face_face_edges = &faces_edges[face_face_index];
					if !face_face_edges.contains(&face_edge) {
						return Err(WindingError::FVFF);
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_edges_and_faces_faces_winding(graph: &Graph) -> Result<(), WindingError> {
	if graph.faces_edges.len() == 0 { return Ok(()); }
	if graph.faces_faces.len() == 0 { return Ok(()); }
	if graph.edges_faces.len() == 0 { return Ok(()); }

	for (face_index, face_edges) in graph.faces_edges.iter().enumerate() {
		let d = face_edges.len();
		for index in 0..d {
			let face_edge_index = face_edges[index];
			let face_edge_faces = &graph.edges_faces[face_edge_index];
			let face_face_option = graph.faces_faces[face_index][index];

			if face_face_option.is_none() { continue; }
			else if !face_edge_faces.contains(&face_face_option) {
				return Err(WindingError::FEFF);
			}
		}
	}

	return Ok(());
}