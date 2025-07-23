use crate::fold::Graph;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug)]
pub enum ReflexiveError {
	VV,
	VE,
	VF,
	EF,
	FF,
}

pub fn validate_vertices_vertices_with_vertices_vertices(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.vertices_vertices.len() == 0 { return Ok(()); }

	let mut hash_map: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (vertex_index, vertex_vertices) in graph.vertices_vertices.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &vertex_vertex_index in vertex_vertices.iter() {
			hash_set.insert(vertex_vertex_index);
		}
		hash_map.insert(vertex_index, hash_set);
	}
	for (vertex_index, vertex_vertices) in graph.vertices_vertices.iter().enumerate() {
		for vertex_vertex_index in vertex_vertices.iter() {
			match hash_map.get(vertex_vertex_index) {
				Some(hash_set) => if !hash_set.contains(&vertex_index) {
					return Err(ReflexiveError::VV)
				},
				None => return Err(ReflexiveError::VV)
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_edges_with_edges_vertices(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.vertices_edges.len() == 0 { return Ok(()); }
	if graph.edges_vertices.len() == 0 { return Ok(()); }

	let mut hash_map: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (vertex_index, vertex_edges) in graph.vertices_edges.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &vertex_edge_index in vertex_edges.iter() {
			hash_set.insert(vertex_edge_index);
		}
		hash_map.insert(vertex_index, hash_set);
	}
	for (edge_index, edge_vertices) in graph.edges_vertices.iter().enumerate() {
		for edge_vertex_index in [edge_vertices.0, edge_vertices.1].iter() {
			match hash_map.get(edge_vertex_index) {
				Some(hash_set) => if !hash_set.contains(&edge_index) {
					return Err(ReflexiveError::VE)
				},
				None => return Err(ReflexiveError::VE)
			}
		}
	}

	for (vertex_index, vertex_edges) in graph.vertices_edges.iter().enumerate() {
		for &vertex_edge_index in vertex_edges.iter() {
			let vertex_edge = graph.edges_vertices[vertex_edge_index];
			if vertex_edge.0 != vertex_index && vertex_edge.1 != vertex_index {
				return Err(ReflexiveError::VE)
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_faces_with_faces_vertices(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.vertices_faces.len() == 0 { return Ok(()); }
	if graph.faces_vertices.len() == 0 { return Ok(()); }

	let mut hash_map12: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (vertex_index, vertex_faces) in graph.vertices_faces.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &vertex_face_option in vertex_faces.iter() {
			match vertex_face_option {
				Some(vertex_face_index) => {
					hash_set.insert(vertex_face_index);
				},
				None => continue
			}
		}
		hash_map12.insert(vertex_index, hash_set);
	}
	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		for face_vertex_index in face_vertices.iter() {
			match hash_map12.get(face_vertex_index) {
				Some(hash_set) => if !hash_set.contains(&face_index) {
					return Err(ReflexiveError::VF)
				},
				None => return Err(ReflexiveError::VF)
			}
		}
	}

	let mut hash_map21: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &face_vertex_index in face_vertices.iter() {
			hash_set.insert(face_vertex_index);
		}
		hash_map21.insert(face_index, hash_set);
	}
	for (vertex_index, vertex_faces) in graph.vertices_faces.iter().enumerate() {
		for vertex_face_option in vertex_faces.iter() {
			match vertex_face_option {
				Some(vertex_face_index) => {
					match hash_map21.get(vertex_face_index) {
						Some(hash_set) => if !hash_set.contains(&vertex_index) {
							return Err(ReflexiveError::VF)
						},
						None => return Err(ReflexiveError::VF)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_edges_faces_with_faces_edges(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.edges_faces.len() == 0 { return Ok(()); }
	if graph.faces_edges.len() == 0 { return Ok(()); }

	let mut hash_map12: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (edge_index, edge_faces) in graph.edges_faces.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &edge_face_option in edge_faces.iter() {
			match edge_face_option {
				Some(edge_face_index) => {
					hash_set.insert(edge_face_index);
				},
				None => continue
			}
		}
		hash_map12.insert(edge_index, hash_set);
	}
	for (face_index, face_edges) in graph.faces_edges.iter().enumerate() {
		for face_edge_index in face_edges.iter() {
			match hash_map12.get(face_edge_index) {
				Some(hash_set) => if !hash_set.contains(&face_index) {
					return Err(ReflexiveError::EF)
				},
				None => return Err(ReflexiveError::EF)
			}
		}
	}

	let mut hash_map21: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (face_index, face_edges) in graph.faces_edges.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &face_edge_index in face_edges.iter() {
			hash_set.insert(face_edge_index);
		}
		hash_map21.insert(face_index, hash_set);
	}
	for (edge_index, edge_faces) in graph.edges_faces.iter().enumerate() {
		for edge_face_option in edge_faces.iter() {
			match edge_face_option {
				Some(edge_face_index) => {
					match hash_map21.get(edge_face_index) {
						Some(hash_set) => if !hash_set.contains(&edge_index) {
							return Err(ReflexiveError::EF)
						},
						None => return Err(ReflexiveError::EF)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_faces_with_faces_faces(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.faces_faces.len() == 0 { return Ok(()); }

	let mut hash_map: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (face_index, face_faces) in graph.faces_faces.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &face_face_option in face_faces.iter() {
			match face_face_option {
				Some(face_face_index) => {
					hash_set.insert(face_face_index);
				},
				None => continue
			}
		}
		hash_map.insert(face_index, hash_set);
	}
	for (face_index, face_faces) in graph.faces_faces.iter().enumerate() {
		for face_face_option in face_faces.iter() {
			match face_face_option {
				Some(face_face_index) => {
					match hash_map.get(face_face_index) {
						Some(hash_set) => if !hash_set.contains(&face_index) {
							return Err(ReflexiveError::FF)
						},
						None => return Err(ReflexiveError::FF)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}