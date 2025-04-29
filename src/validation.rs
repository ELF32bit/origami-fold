use super::file::File;
use super::frame::{Frame, EdgeAssignment};

fn edge_equals(edge: &[usize; 2], uv: &[usize; 2]) -> bool {
	return (edge[0] == uv[0] && edge[1] == uv[1]) || (edge[0] == uv[1] && edge[1] == uv[0])
}

pub fn validate_length<T>(vec: &Vec<T>, length: usize) -> bool {
	let vec_length = vec.len();
	return vec_length == 0 || vec_length == length;
}

pub fn validate_ids(vec: &Vec<Vec<usize>>, max_id: usize) -> bool {
	for vec_item in vec.iter() {
		for &id in vec_item.iter() {
			if !(id < max_id) { return false; }
		}
	}
	return true;
}

pub fn validate_edge_ids(vec: &Vec<[usize; 2]>, max_id: usize) -> bool {
	for vec_item in vec.iter() {
		for &id in vec_item.iter() {
			if !(id < max_id) { return false; }
		}
	}
	return true;
}

pub fn validate_option_ids(vec: &Vec<Vec<Option<usize>>>, max_id: usize) -> bool {
	for vec_item in vec.iter() {
		for &id_option in vec_item.iter() {
			match id_option {
				Some(id) => { if !(id < max_id) { return false; } },
				None => continue
			}
		}
	}
	return true;
}

pub fn validate_order_ids<T>(vec: &Vec<(usize, usize, T)>, max_id: usize) -> bool {
	for vec_item in vec.iter() {
		if !(vec_item.0 < max_id && vec_item.1 < max_id) { return false; }
	}
	return true;
}

pub fn validate_vertices_edges_with_vertices(frame: &Frame) -> bool {
	if frame.vertices_vertices.len() == 0 { return true; }
	if frame.edges_vertices.len() == 0 { return true; }

	for (vertex_id, vertex_edges) in frame.vertices_edges.iter().enumerate() {
		let vertex_vertices = &frame.vertices_vertices[vertex_id];
		if vertex_edges.len() != vertex_vertices.len() { return false; }

		for (index, &vertex_edge_id) in vertex_edges.iter().enumerate() {
			let vertex_vertex_id = vertex_vertices[index];
			let vertex_edge = &frame.edges_vertices[vertex_edge_id];
			if !edge_equals(vertex_edge, &[vertex_id, vertex_vertex_id]) { return false; }
		}
	}
	return true;
}

pub fn validate_vertices_faces_with_vertices(frame: &Frame) -> bool {
	if !frame.attributes.iter().any(|a| a == "manifold") { return true; }
	if frame.vertices_vertices.len() == 0 { return true; }
	if frame.faces_vertices.len() == 0 { return true; }

	for (vertex_id, vertex_faces) in frame.vertices_faces.iter().enumerate() {
		let vertex_vertices = &frame.vertices_vertices[vertex_id];
		let d = vertex_vertices.len();
		if !vertex_faces.len() == d { return false; }

		for (index, &vertex_face_id) in vertex_faces.iter().enumerate() {
			match vertex_face_id {
				Some(id) => {
					let vertex_vertex_id = vertex_vertices[index];
					let vertex_next_vertex_id = vertex_vertices[(index + 1) % d];
					let vertex_face_vertices = &frame.faces_vertices[id];
					if !vertex_face_vertices.contains(&vertex_vertex_id) { return false; }
					if !vertex_face_vertices.contains(&vertex_next_vertex_id) { return false; }
				},
				None => continue
			}
		}
	}
	return true;
}

pub fn validate_vertices_faces_with_edges(frame: &Frame) -> bool {
	if !frame.attributes.iter().any(|a| a == "manifold") { return true; }
	if frame.vertices_edges.len() == 0 { return true; }
	if frame.faces_vertices.len() == 0 { return true; }

	for (vertex_id, vertex_faces) in frame.vertices_faces.iter().enumerate() {
		let vertex_edges = &frame.vertices_edges[vertex_id];
		let d = vertex_edges.len();
		if !vertex_faces.len() == d { return false; }

		for (index, &vertex_face_id) in vertex_faces.iter().enumerate() {
			match vertex_face_id {
				Some(id) => {
					let vertex_face_vertices = &frame.faces_vertices[id];
					let vertex_edge_id = vertex_edges[index];
					let vertex_next_edge_id = vertex_edges[(index + 1) % d];

					if frame.faces_edges.len() > 0 {
						if !frame.faces_edges[id].contains(&vertex_edge_id) { return false; }
						if !frame.faces_edges[id].contains(&vertex_next_edge_id) { return false; }
					} else {
						if frame.edges_vertices.len() == 0 { return true; }
						let vertex_edge = &frame.edges_vertices[vertex_edge_id];
						let vertex_next_edge = &frame.edges_vertices[vertex_next_edge_id];
						let d2 = vertex_face_vertices.len();

						let mut is_vertex_edge = false;
						let mut is_vertex_next_edge = false;
						for index in 0..d2 {
							let uv = [vertex_face_vertices[index], vertex_face_vertices[(index + 1) % d2]];
							if edge_equals(vertex_edge, &uv) { is_vertex_edge = true; }
							if edge_equals(vertex_next_edge, &uv) { is_vertex_next_edge = true; }
							if is_vertex_edge && is_vertex_next_edge { break; }
						}
						if !(is_vertex_edge && is_vertex_next_edge) { return false; }
					}
				},
				None => continue
			}
		}
	}
	return true;
}

pub fn validate_edges_faces(frame: &Frame) -> bool {
	let is_manifold = frame.attributes.iter().any(|a| a == "manifold");
	for (edge_id, edge_faces) in frame.edges_faces.iter().enumerate() {
		let mut is_boundary_edge = false;
		match frame.edges_assignment.get(edge_id) {
			Some(edge_assignment) => {
				match edge_assignment {
					EdgeAssignment::Boundary => { is_boundary_edge = true; }
					_ => ()
				}
			},
			None => ()
		}
		if is_manifold {
			let edge_faces_length = edge_faces.len();
			if is_boundary_edge {
				if !(edge_faces_length == 1 || edge_faces_length == 2) { return false; }
			} else if edge_faces_length != 2 { return false; }
		}
	}
	return true;
}

pub fn validate_faces_edges_with_vertices(frame: &Frame) -> bool {
	if frame.edges_vertices.len() == 0 { return true; }
	if frame.faces_vertices.len() == 0 { return true; }

	for (face_id, face_edges) in frame.faces_edges.iter().enumerate() {
		let face_vertices = &frame.faces_vertices[face_id];
		let d = face_vertices.len();
		if !(face_edges.len() == d) { return false; }

		for (index, &face_edge_id) in face_edges.iter().enumerate() {
			let face_edge = &frame.edges_vertices[face_edge_id];
			let face_vertices_edge = [face_vertices[index], face_vertices[(index + 1) % d]];
			if !edge_equals(face_edge, &face_vertices_edge) { return false; }
		}
	}
	return true;
}

pub fn validate_faces_faces_with_edges(frame: &Frame) -> bool {
	if !frame.attributes.iter().any(|a| a == "manifold") { return true; }
	if frame.faces_edges.len() == 0 { return true; }

	for (face_id, face_faces) in frame.faces_faces.iter().enumerate() {
		let face_edges = &frame.faces_edges[face_id];
		if face_faces.len() != face_edges.len() { return false; }

		for (index, &face_face_id) in face_faces.iter().enumerate() {
			match face_face_id {
				Some(id) => {
					let face_edge = face_edges[index];
					let face_face_edges = &frame.faces_edges[id];
					if !face_face_edges.contains(&face_edge) { return false; }
				},
				None => continue
			}
		}
	}
	return true;
}

pub fn validate_frame_parents(file: &File, frame_id: usize) -> bool {
	match file.get_frame(frame_id) {
		Some(frame) => {
			let mut ids: Vec<usize> = vec![frame_id];
			let mut current_frame = frame;
			loop {
				match current_frame.parent {
					Some(id) => {
						match file.get_frame(id) {
							Some(parent_frame) => {
								if ids.contains(&id) {
									return false;
								} else {
									ids.push(id);
								}
								current_frame = parent_frame;
							},
							None => return false
						}
					},
					None => return true
				}
			}
		},
		None => return false
	}
}
