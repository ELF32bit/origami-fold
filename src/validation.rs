use super::fold::Fold;
use super::graph::Graph;
use super::graph::EdgeAssignment;
use std::collections::{HashSet, HashMap};

#[derive(Clone, Copy, Debug)]
pub enum FrameError {
	MissingParent(usize, usize),
	InfiniteParents(usize),
}

#[derive(Clone, Copy, Debug)]
pub enum GraphError {
	VCLength,
	FVLength,

	VVLength,
	VELength,
	VFLength,
	EFLength,
	EALength,
	EFALength,
	ELLength,
	FELength,
	FFLength,

	VVIndices,
	VEIndices,
	VFIndices,
	EVIndices,
	EFIndices,
	EOIndices,
	FVIndices,
	FEIndices,
	FFIndices,
	FOIndices,

	VVWithVV,
	VEWithEV,
	VFWithFV,
	EFWithFE,
	FFWithFF,

	VVVEWinding,
	VVVFWinding,
	VEVFWinding,
	FVFEWinding,
	FVFFWinding,
	FEFFWinding,

	EAWithEFA,
}

#[derive(Clone, Copy, Debug)]
pub enum Error {
	FrameError(FrameError),
	GraphError(GraphError),
}

impl From<FrameError> for Error {
	fn from(error: FrameError) -> Self { Error::FrameError(error) }
}

impl From<GraphError> for Error {
	fn from(error: GraphError) -> Self { Error::GraphError(error) }
}

pub fn validate_frame_parents(fold: &Fold, frame_index: usize) -> Result<(), FrameError> {
	let mut frame = fold.get_frame(frame_index).unwrap();
	let mut frame_parents = HashSet::from([frame_index]);
	loop {
		match frame.parent {
			Some(id) => {
				match fold.get_frame(id) {
					Some(frame_parent) => {
						if frame_parents.contains(&id) {
							return Err(FrameError::InfiniteParents(frame_index));
						}
						frame_parents.insert(id);
						frame = frame_parent;
					}
					None => return Err(FrameError::MissingParent(frame_index, id))
				}
			}
			None => break
		}
	}
	return Ok(());
}

pub fn validate_vertices_coordinates(graph: &Graph) -> Result<(), GraphError> {
	if !(graph.vertices_coordinates.len() > 0) { return Ok(()); }
	let coordinates_length = graph.vertices_coordinates[0].len();
	for vertex_coordinates in graph.vertices_coordinates.iter() {
		if vertex_coordinates.len() != coordinates_length {
			return Err(GraphError::VCLength);
		}
	}
	return Ok(());
}

pub fn validate_faces_vertices(graph: &Graph) -> Result<(), GraphError> {
	for face_vertices in graph.faces_vertices.iter() {
		if face_vertices.len() < 3 {
			return Err(GraphError::FVLength);
		}
	}
	return Ok(());
}

pub fn validate_vertices_vertices_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.vertices_vertices.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(GraphError::VVLength);
	}
	return Ok(());
}

pub fn validate_vertices_edges_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.vertices_edges.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(GraphError::VELength);
	}
	return Ok(());
}

pub fn validate_vertices_faces_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.vertices_faces.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(GraphError::VFLength);
	}
	return Ok(());
}

pub fn validate_edges_faces_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.edges_faces.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(GraphError::EFLength);
	}
	return Ok(());
}

pub fn validate_edges_assignment_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.edges_assignment.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(GraphError::EALength);
	}
	return Ok(());
}

pub fn validate_edges_fold_angle_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.edges_fold_angle.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(GraphError::EFALength);
	}
	return Ok(());
}

pub fn validate_edges_length_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.edges_length.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(GraphError::ELLength);
	}
	return Ok(());
}

pub fn validate_faces_edges_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.faces_edges.len();
	if !(length == 0 || length == graph.faces_vertices.len()) {
		return Err(GraphError::FELength);
	}
	return Ok(());
}

pub fn validate_faces_faces_length(graph: &Graph) -> Result<(), GraphError> {
	let length = graph.faces_faces.len();
	if !(length == 0 || length == graph.faces_vertices.len()) {
		return Err(GraphError::FFLength);
	}
	return Ok(());
}

pub fn validate_vertices_vertices_indices(graph: &Graph) -> Result<(), GraphError> {
	let max_index = graph.vertices_coordinates.len();
	for vertex_vertices in graph.vertices_vertices.iter() {
		for &index in vertex_vertices.iter() {
			if index >= max_index {
				return Err(GraphError::VVIndices);
			}
		}
	}
	return Ok(());
}

pub fn validate_vertices_edges_indices(graph: &Graph) -> Result<(), GraphError> {
	let max_index = graph.edges_vertices.len();
	for vertex_edges in graph.vertices_edges.iter() {
		for &index in vertex_edges.iter() {
			if index >= max_index {
				return Err(GraphError::VEIndices);
			}
		}
	}
	return Ok(());
}

pub fn validate_vertices_faces_indices(graph: &Graph) -> Result<(), GraphError> {
	let max_index = graph.faces_vertices.len();
	for vertex_faces in graph.vertices_faces.iter() {
		for &index in vertex_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(GraphError::VFIndices);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_edges_vertices_indices(graph: &Graph) -> Result<(), GraphError> {
	let max_index = graph.vertices_coordinates.len();
	for edge_vertices in graph.edges_vertices.iter() {
		if edge_vertices.0 >= max_index || edge_vertices.1 >= max_index {
				return Err(GraphError::EVIndices);
		}
	}
	return Ok(());
}

pub fn validate_edges_faces_indices(graph: &Graph) -> Result<(), GraphError> {
	let max_index = graph.faces_vertices.len();
	for edge_faces in graph.edges_faces.iter() {
		for &index in edge_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(GraphError::EFIndices);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_edge_orders_indices(graph: &Graph) -> Result<(), GraphError> {
	let mut edge_pairs: HashSet<(usize, usize)> = HashSet::new();
	let max_index = graph.edges_vertices.len();
	for edge_order in graph.edge_orders.iter() {
		if edge_pairs.contains(&(edge_order.0, edge_order.1)) {
			return Err(GraphError::EOIndices);
		}
		else if edge_order.0 == edge_order.1 {
			return Err(GraphError::EOIndices);
		}
		else if edge_order.0 >= max_index || edge_order.1 >= max_index {
			return Err(GraphError::EOIndices);
		}
		edge_pairs.insert((edge_order.0, edge_order.1));
		edge_pairs.insert((edge_order.1, edge_order.0));
	}
	return Ok(());
}

pub fn validate_faces_vertices_indices(graph: &Graph) -> Result<(), GraphError> {
	let max_index = graph.vertices_coordinates.len();
	for face_vertices in graph.faces_vertices.iter() {
		for &index in face_vertices.iter() {
			if index >= max_index {
				return Err(GraphError::FVIndices);
			}
		}
	}
	return Ok(());
}

pub fn validate_faces_edges_indices(graph: &Graph) -> Result<(), GraphError> {
	let max_index = graph.edges_vertices.len();
	for face_edges in graph.faces_edges.iter() {
		for &index in face_edges.iter() {
			if index >= max_index {
				return Err(GraphError::FEIndices);
			}
		}
	}
	return Ok(());
}

pub fn validate_faces_faces_indices(graph: &Graph) -> Result<(), GraphError> {
	let max_index = graph.faces_vertices.len();
	for face_faces in graph.faces_faces.iter() {
		for &index in face_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(GraphError::FFIndices);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_face_orders_indices(graph: &Graph) -> Result<(), GraphError> {
	let mut face_pairs: HashSet<(usize, usize)> = HashSet::new();
	let max_index = graph.faces_vertices.len();
	for face_order in graph.face_orders.iter() {
		if face_pairs.contains(&(face_order.0, face_order.1)) {
			return Err(GraphError::FOIndices);
		}
		else if face_order.0 == face_order.1 {
			return Err(GraphError::FOIndices);
		}
		else if face_order.0 >= max_index || face_order.1 >= max_index {
			return Err(GraphError::FOIndices);
		}
		face_pairs.insert((face_order.0, face_order.1));
		face_pairs.insert((face_order.1, face_order.0));
	}
	return Ok(());
}

pub fn validate_vertices_vertices_with_vertices_vertices(graph: &Graph) -> Result<(), GraphError> {
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
					return Err(GraphError::VVWithVV)
				},
				None => return Err(GraphError::VVWithVV)
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_edges_with_edges_vertices(graph: &Graph) -> Result<(), GraphError> {
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
					return Err(GraphError::VEWithEV)
				},
				None => return Err(GraphError::VEWithEV)
			}
		}
	}

	for (vertex_index, vertex_edges) in graph.vertices_edges.iter().enumerate() {
		for &vertex_edge_index in vertex_edges.iter() {
			let vertex_edge = graph.edges_vertices[vertex_edge_index];
			if vertex_edge.0 != vertex_index && vertex_edge.1 != vertex_index {
				return Err(GraphError::VEWithEV)
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_faces_with_faces_vertices(graph: &Graph) -> Result<(), GraphError> {
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
					return Err(GraphError::VFWithFV)
				},
				None => return Err(GraphError::VFWithFV)
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
							return Err(GraphError::VFWithFV)
						},
						None => return Err(GraphError::VFWithFV)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_edges_faces_with_faces_edges(graph: &Graph) -> Result<(), GraphError> {
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
					return Err(GraphError::EFWithFE)
				},
				None => return Err(GraphError::EFWithFE)
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
							return Err(GraphError::EFWithFE)
						},
						None => return Err(GraphError::EFWithFE)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_faces_with_faces_faces(graph: &Graph) -> Result<(), GraphError> {
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
							return Err(GraphError::FFWithFF)
						},
						None => return Err(GraphError::FFWithFF)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_vertices_and_vertices_edges_winding(graph: &Graph) -> Result<(), GraphError> {
	if graph.vertices_vertices.len() == 0 { return Ok(()); }
	if graph.vertices_edges.len() == 0 { return Ok(()); }
	if graph.edges_vertices.len() == 0 { return Ok(()); }

	for (vertex_index, vertex_vertices) in graph.vertices_vertices.iter().enumerate() {
		for (index, &vertex_vertex_index) in vertex_vertices.iter().enumerate() {
			let vertex_edge_index = graph.vertices_edges[vertex_index][index];
			let vertex_edge = graph.edges_vertices[vertex_edge_index];

			if vertex_edge == (vertex_index, vertex_vertex_index) { continue; }
			else if vertex_edge == (vertex_vertex_index, vertex_index) { continue; }
			else { return Err(GraphError::VVVEWinding); }
		}
	}
	return Ok(());
}

pub fn validate_vertices_vertices_and_vertices_faces_winding(graph: &Graph) -> Result<(), GraphError> {
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
						return Err(GraphError::VVVEWinding);
					}
				}
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_edges_and_vertices_faces_winding(graph: &Graph) -> Result<(), GraphError> {
	if graph.vertices_edges.len() == 0 { return Ok(()); }
	if graph.vertices_faces.len() == 0 { return Ok(()); }
	if graph.faces_edges.len() == 0 { return Ok(()); }

	let mut faces_edges_pairs: Vec<HashSet<(usize, usize)>> = Vec::new();
	faces_edges_pairs.resize(graph.faces_edges.len(), HashSet::new());
	for (face_index, face_edges) in graph.faces_edges.iter().enumerate() {
		let d = face_edges.len();
		for (index, &face_edge_index) in face_edges.iter().enumerate() {
			let next_face_edge_index = face_edges[(index + 1) % d];
			faces_edges_pairs[face_index].insert((face_edge_index, next_face_edge_index));
			faces_edges_pairs[face_index].insert((next_face_edge_index, face_edge_index));
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
						return Err(GraphError::VEVFWinding);
					}
				}
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_vertices_and_faces_edges_winding(graph: &Graph) -> Result<(), GraphError> {
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
			else { return Err(GraphError::FVFEWinding); }
		}
	}

	return Ok(());
}

pub fn validate_faces_vertices_and_faces_faces_winding(graph: &Graph) -> Result<(), GraphError> {
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
						return Err(GraphError::FVFFWinding);
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_edges_and_faces_faces_winding(graph: &Graph) -> Result<(), GraphError> {
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
				return Err(GraphError::FEFFWinding);
			}
		}
	}

	return Ok(());
}

pub fn validate_edges_assignment_with_edges_fold_angle(graph: &Graph) -> Result<(), GraphError> {
	if graph.edges_assignment.len() == 0 { return Ok(()); }
	if graph.edges_fold_angle.len() == 0 { return Ok(()); }

	for (edge_index, edge_assignment) in graph.edges_assignment.iter().enumerate() {
		let fold_angle: f64;
		match graph.edges_fold_angle[edge_index].as_f64() {
			Some(angle) => fold_angle = angle,
			None => continue
		}

		match edge_assignment {
			EdgeAssignment::Mountain => if fold_angle >= 0.0 {
				return Err(GraphError::EAWithEFA);
			},
			EdgeAssignment::Valley => if fold_angle <= 0.0 {
				return Err(GraphError::EAWithEFA);
			},
			_ => if fold_angle != 0.0 {
				return Err(GraphError::EAWithEFA);
			}
		}
	}

	return Ok(());
}