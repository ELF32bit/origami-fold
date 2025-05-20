use crate::fold;

use super::fold::Fold;
use super::graph::Graph;
use super::graph::EdgeAssignment;
use std::collections::{HashSet, HashMap};

pub enum Error {
	Generic
}

pub fn validate_frame_parents(fold: &Fold, frame_index: usize) -> Result<(), Error> {
	let mut frame = fold.get_frame(frame_index).unwrap();
	let mut frame_parents = HashSet::from([frame_index]);

	loop {
		match frame.parent {
			Some(id) => {
				match fold.get_frame(id) {
					Some(frame_parent) => {
						if frame_parents.contains(&id) {
							return Err(Error::Generic);
						}
						frame_parents.insert(id);
						frame = frame_parent;
					}
					None => return Err(Error::Generic)
				}
			}
			None => break
		}
	}

	return Ok(());
}

pub fn validate_vertices_vertices_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.vertices_vertices.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_vertices_edges_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.vertices_edges.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_vertices_faces_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.vertices_faces.len();
	if !(length == 0 || length == graph.vertices_coordinates.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_edges_faces_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.edges_faces.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_edges_assignment_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.edges_assignment.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_edges_fold_angle_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.edges_fold_angle.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_edges_length_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.edges_length.len();
	if !(length == 0 || length == graph.edges_vertices.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_faces_edges_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.faces_edges.len();
	if !(length == 0 || length == graph.faces_vertices.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_faces_faces_length(graph: &Graph) -> Result<(), Error> {
	let length = graph.faces_faces.len();
	if !(length == 0 || length == graph.faces_vertices.len()) {
		return Err(Error::Generic);
	}
	return Ok(());
}

pub fn validate_vertices_vertices_indices(graph: &Graph) -> Result<(), Error> {
	let max_index = graph.vertices_coordinates.len();
	for vertex_vertices in graph.vertices_vertices.iter() {
		for &index in vertex_vertices.iter() {
			if index >= max_index {
				return Err(Error::Generic);
			}
		}
	}
	return Ok(());
}

pub fn validate_vertices_edges_indices(graph: &Graph) -> Result<(), Error> {
	let max_index = graph.edges_vertices.len();
	for vertex_edges in graph.vertices_edges.iter() {
		for &index in vertex_edges.iter() {
			if index >= max_index {
				return Err(Error::Generic);
			}
		}
	}
	return Ok(());
}

pub fn validate_vertices_faces_indices(graph: &Graph) -> Result<(), Error> {
	let max_index = graph.faces_vertices.len();
	for vertex_faces in graph.vertices_faces.iter() {
		for &index in vertex_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(Error::Generic);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_edges_vertices_indices(graph: &Graph) -> Result<(), Error> {
	let max_index = graph.vertices_coordinates.len();
	for edge_vertices in graph.edges_vertices.iter() {
		if edge_vertices.0 >= max_index || edge_vertices.1 >= max_index {
				return Err(Error::Generic);
		}
	}
	return Ok(());
}

pub fn validate_edges_faces_indices(graph: &Graph) -> Result<(), Error> {
	let max_index = graph.faces_vertices.len();
	for edge_faces in graph.edges_faces.iter() {
		for &index in edge_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(Error::Generic);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_edge_orders_indices(graph: &Graph) -> Result<(), Error> {
	let mut edge_pairs: HashSet<(usize, usize)> = HashSet::new();
	let max_index = graph.edges_vertices.len();
	for edge_order in graph.edge_orders.iter() {
		if edge_pairs.contains(&(edge_order.0, edge_order.1)) {
			return Err(Error::Generic);
		}
		else if edge_order.0 == edge_order.1 {
			return Err(Error::Generic);
		}
		else if edge_order.0 >= max_index || edge_order.1 >= max_index {
			return Err(Error::Generic);
		}
		edge_pairs.insert((edge_order.0, edge_order.1));
		edge_pairs.insert((edge_order.1, edge_order.0));
	}
	return Ok(());
}

pub fn validate_faces_vertices_indices(graph: &Graph) -> Result<(), Error> {
	let max_index = graph.vertices_coordinates.len();
	for face_vertices in graph.faces_vertices.iter() {
		for &index in face_vertices.iter() {
			if index >= max_index {
				return Err(Error::Generic);
			}
		}
	}
	return Ok(());
}

pub fn validate_faces_edges_indices(graph: &Graph) -> Result<(), Error> {
	let max_index = graph.edges_vertices.len();
	for face_edges in graph.faces_edges.iter() {
		for &index in face_edges.iter() {
			if index >= max_index {
				return Err(Error::Generic);
			}
		}
	}
	return Ok(());
}

pub fn validate_faces_faces_indices(graph: &Graph) -> Result<(), Error> {
	let max_index = graph.faces_vertices.len();
	for face_faces in graph.faces_faces.iter() {
		for &index in face_faces.iter() {
			match index {
				Some(index) => if index >= max_index {
					return Err(Error::Generic);
				},
				None => continue
			}
		}
	}
	return Ok(());
}

pub fn validate_face_orders_indices(graph: &Graph) -> Result<(), Error> {
	let mut face_pairs: HashSet<(usize, usize)> = HashSet::new();
	let max_index = graph.faces_vertices.len();
	for face_order in graph.face_orders.iter() {
		if face_pairs.contains(&(face_order.0, face_order.1)) {
			return Err(Error::Generic);
		}
		else if face_order.0 == face_order.1 {
			return Err(Error::Generic);
		}
		else if face_order.0 >= max_index || face_order.1 >= max_index {
			return Err(Error::Generic);
		}
		face_pairs.insert((face_order.0, face_order.1));
		face_pairs.insert((face_order.1, face_order.0));
	}
	return Ok(());
}

pub fn validate_vertices_vertices_with_vertices_vertices(graph: &Graph) -> Result<(), Error> {
	if graph.vertices_vertices.len() == 0 {
		return Ok(());
	}

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
					return Err(Error::Generic)
				},
				None => return Err(Error::Generic)
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_edges_with_edges_vertices(graph: &Graph) -> Result<(), Error> {
	if graph.vertices_edges.len() == 0 || graph.edges_vertices.len() == 0 {
		return Ok(());
	}

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
					return Err(Error::Generic)
				},
				None => return Err(Error::Generic)
			}
		}
	}

	for (vertex_index, vertex_edges) in graph.vertices_edges.iter().enumerate() {
		for &vertex_edge_index in vertex_edges.iter() {
			let vertex_edge = graph.edges_vertices[vertex_edge_index];
			if vertex_edge.0 != vertex_index && vertex_edge.1 != vertex_index {
				return Err(Error::Generic)
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_faces_with_faces_vertices(graph: &Graph) -> Result<(), Error> {
	if graph.vertices_faces.len() == 0 || graph.faces_vertices.len() == 0 {
		return Ok(());
	}

	let mut hash_map1: HashMap<usize, HashSet<usize>> = HashMap::new();
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
		hash_map1.insert(vertex_index, hash_set);
	}
	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		for face_vertex_index in face_vertices.iter() {
			match hash_map1.get(face_vertex_index) {
				Some(hash_set) => if !hash_set.contains(&face_index) {
					return Err(Error::Generic)
				},
				None => return Err(Error::Generic)
			}
		}
	}

	let mut hash_map2: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &face_vertex_index in face_vertices.iter() {
			hash_set.insert(face_vertex_index);
		}
		hash_map2.insert(face_index, hash_set);
	}
	for (vertex_index, vertex_faces) in graph.vertices_faces.iter().enumerate() {
		for vertex_face_option in vertex_faces.iter() {
			match vertex_face_option {
				Some(vertex_face_index) => {
					match hash_map2.get(vertex_face_index) {
						Some(hash_set) => if !hash_set.contains(&vertex_index) {
							return Err(Error::Generic)
						},
						None => return Err(Error::Generic)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_edges_faces_with_faces_edges(graph: &Graph) -> Result<(), Error> {
	if graph.edges_faces.len() == 0 || graph.faces_edges.len() == 0 {
		return Ok(());
	}

	let mut hash_map1: HashMap<usize, HashSet<usize>> = HashMap::new();
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
		hash_map1.insert(edge_index, hash_set);
	}
	for (face_index, face_edges) in graph.faces_edges.iter().enumerate() {
		for face_edge_index in face_edges.iter() {
			match hash_map1.get(face_edge_index) {
				Some(hash_set) => if !hash_set.contains(&face_index) {
					return Err(Error::Generic)
				},
				None => return Err(Error::Generic)
			}
		}
	}

	let mut hash_map2: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (face_index, face_edges) in graph.faces_edges.iter().enumerate() {
		let mut hash_set = HashSet::<usize>::new();
		for &face_edge_index in face_edges.iter() {
			hash_set.insert(face_edge_index);
		}
		hash_map2.insert(face_index, hash_set);
	}
	for (edge_index, edge_faces) in graph.edges_faces.iter().enumerate() {
		for edge_face_option in edge_faces.iter() {
			match edge_face_option {
				Some(edge_face_index) => {
					match hash_map2.get(edge_face_index) {
						Some(hash_set) => if !hash_set.contains(&edge_index) {
							return Err(Error::Generic)
						},
						None => return Err(Error::Generic)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_faces_with_faces_faces(graph: &Graph) -> Result<(), Error> {
	if graph.faces_faces.len() == 0 {
		return Ok(());
	}

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
							return Err(Error::Generic)
						},
						None => return Err(Error::Generic)
					}
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_vertices_and_vertices_edges_winding(graph: &Graph) -> Result<(), Error> {
	if graph.vertices_vertices.len() == 0 { return Ok(()); }
	if graph.vertices_edges.len() == 0 { return Ok(()); }
	if graph.edges_vertices.len() == 0 { return Ok(()); }

	for (vertex_index, vertex_vertices) in graph.vertices_vertices.iter().enumerate() {
		for (index, &vertex_vertex_index) in vertex_vertices.iter().enumerate() {
			let vertex_edge_index = graph.vertices_edges[vertex_index][index];
			let vertex_edge = graph.edges_vertices[vertex_edge_index];

			if vertex_edge == (vertex_index, vertex_vertex_index) { continue; }
			else if vertex_edge == (vertex_vertex_index, vertex_index) { continue; }
			else { return Err(Error::Generic); }
		}
	}
	return Ok(());
}

pub fn validate_vertices_vertices_and_vertices_faces_winding(graph: &Graph) -> Result<(), Error> {
	if graph.vertices_vertices.len() == 0 { return Ok(()); }
	if graph.vertices_faces.len() == 0 { return Ok(()); }
	if graph.faces_vertices.len() == 0 { return Ok(()); }

	for (vertex_index, vertex_vertices) in graph.vertices_vertices.iter().enumerate() {
		let d = vertex_vertices.len();
		for index in 0..d {
			let vertex_vertex_index = vertex_vertices[index];
			let vertex_next_vertex_index = vertex_vertices[(index + 1) % d];
			let vertex_face_option = graph.vertices_faces[vertex_index][index];

			match vertex_face_option {
				Some(vertex_face_index) => {
					let vertex_face = &graph.faces_vertices[vertex_face_index];
					// TODO: properly with order!
					if !vertex_face.contains(&vertex_vertex_index) { return Err(Error::Generic); }
					if !vertex_face.contains(&vertex_next_vertex_index) { return Err(Error::Generic); }
				}
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_vertices_edges_and_vertices_faces_winding(graph: &Graph) -> Result<(), Error> {
	if graph.vertices_edges.len() == 0 { return Ok(()); }
	if graph.vertices_faces.len() == 0 { return Ok(()); }
	if graph.faces_edges.len() == 0 { return Ok(()); }

	for (vertex_index, vertex_edges) in graph.vertices_edges.iter().enumerate() {
		let d = vertex_edges.len();
		for index in 0..d {
			let vertex_edge_index = vertex_edges[index];
			let vertex_next_edge_index = vertex_edges[(index + 1) % d];
			let vertex_face_option = graph.vertices_faces[vertex_index][index];

			match vertex_face_option {
				Some(vertex_face_index) => {
					let vertex_edge = &graph.faces_edges[vertex_face_index];
					// TODO: properly with order!
					if !vertex_edge.contains(&vertex_edge_index) { return Err(Error::Generic); }
					if !vertex_edge.contains(&vertex_next_edge_index) { return Err(Error::Generic); }
				}
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_vertices_and_faces_edges_winding(graph: &Graph) -> Result<(), Error> {
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
			else { return Err(Error::Generic); }
		}
	}

	return Ok(());
}

pub fn validate_faces_vertices_and_faces_faces_winding(graph: &Graph) -> Result<(), Error> {
	if graph.faces_vertices.len() == 0 { return Ok(()); }
	if graph.faces_faces.len() == 0 { return Ok(()); }
	if graph.faces_vertices.len() == 0 { return Ok(()); }

	for (face_index, face_vertices) in graph.faces_vertices.iter().enumerate() {
		let d = face_vertices.len();
		for index in 0..d {
			let face_vertex_index = face_vertices[index];
			let face_next_vertex_index = face_vertices[(index + 1) % d];
			let face_face_option = graph.faces_faces[face_index][index];

			match face_face_option {
				Some(face_face_index) => {
					let face_face = &graph.faces_vertices[face_face_index];
					// TODO: properly with order!
					if !face_face.contains(&face_vertex_index) { return Err(Error::Generic); }
					if !face_face.contains(&face_next_vertex_index) { return Err(Error::Generic); }
				},
				None => continue
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_edges_and_faces_faces_winding(graph: &Graph) -> Result<(), Error> {
	if graph.faces_edges.len() == 0 { return Ok(()); }
	if graph.faces_faces.len() == 0 { return Ok(()); }
	if graph.edges_faces.len() == 0 { return Ok(()); }

	for (face_index, face_edges) in graph.faces_edges.iter().enumerate() {
		let d = face_edges.len();
		for index in 0..d {
			let face_edge_index = face_edges[index];
			let face_edge_faces = &graph.edges_faces[face_edge_index];
			let face_face_option = graph.faces_faces[face_index][index];

			// TODO: properly, HashSet?
			if face_face_option.is_none() { continue; }
			else if !face_edge_faces.contains(&face_face_option) {
				return Err(Error::Generic);
			}
		}
	}

	return Ok(());
}

pub fn validate_edges_assignment_with_edges_fold_angle(graph: &Graph) -> Result<(), Error> {
	if graph.edges_assignment.len() == 0 || graph.edges_fold_angle.len() == 0 {
		return Ok(());
	}

	assert!(graph.edges_assignment.len() == graph.edges_fold_angle.len());
	for (edge_index, edge_assignment) in graph.edges_assignment.iter().enumerate() {
		let fold_angle = graph.edges_fold_angle[edge_index].as_f64().unwrap();
		match edge_assignment {
			EdgeAssignment::Mountain => if !(fold_angle < 0.0) {
				return Err(Error::Generic);
			},
			EdgeAssignment::Valley => if !(fold_angle > 0.0) {
				return Err(Error::Generic);
			},
			_ => if fold_angle != 0.0 {
				return Err(Error::Generic);
			}
		}
	}

	return Ok(());
}