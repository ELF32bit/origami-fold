use crate::fold::Graph;
use std::collections::{HashMap, HashSet};
use crate::fold::EdgeAssignment;

#[derive(Clone, Copy, Debug)]
pub enum ReflexiveError {
	VV(usize, usize),
	VEEV(usize, usize),
	EVVE(usize, usize),
	FVVF(usize, usize),
	VFFV(usize, usize),
	FEEF(usize, usize),
	EFFE(usize, usize),
	EFA(usize),
	FF(usize, usize),
}

fn map(vec: &Vec<Vec<usize>>) -> HashMap<usize, HashSet<usize>> {
	let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (vec_index, vec_vec) in vec.iter().enumerate() {
		let mut set = HashSet::<usize>::new();
		for &vec_vec_index in vec_vec.iter() {
			set.insert(vec_vec_index);
		}
		map.insert(vec_index, set);
	}
	return map;
}

fn map_with_null(vec: &Vec<Vec<Option<usize>>>) -> HashMap<usize, HashSet<usize>> {
	let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
	for (vec_index, vec_vec) in vec.iter().enumerate() {
		let mut set = HashSet::<usize>::new();
		for &vec_vec_option in vec_vec.iter() {
			if vec_vec_option.is_none() { continue; }
			let vec_vec_index = vec_vec_option.unwrap();
			set.insert(vec_vec_index);
		}
		map.insert(vec_index, set);
	}
	return map;
}

macro_rules! validate {
	($map: ident, $rvec: expr, $error: ident) => {
		for (rvec_index, rvec_vec) in $rvec.iter().enumerate() {
			for &rvec_vec_index in rvec_vec.iter() {
				match $map.get(&rvec_vec_index) {
					Some(set) => if !set.contains(&rvec_index) {
						return Err(ReflexiveError::$error(rvec_index, rvec_vec_index));
					}
					None => return Err(ReflexiveError::$error(rvec_index, rvec_vec_index))
				}
			}
		}
	};
}

macro_rules! validate_with_null {
	($map: ident, $rvec: expr, $error: ident) => {
		for (rvec_index, rvec_vec) in $rvec.iter().enumerate() {
			for &rvec_vec_option in rvec_vec.iter() {
				if rvec_vec_option.is_none() { continue; }
				let rvec_vec_index = rvec_vec_option.unwrap();
				match $map.get(&rvec_vec_index) {
					Some(set) => if !set.contains(&rvec_index) {
						return Err(ReflexiveError::$error(rvec_index, rvec_vec_index));
					}
					None => return Err(ReflexiveError::$error(rvec_index, rvec_vec_index))
				}
			}
		}
	};
}

pub fn validate_vertices_vertices_with_vertices_vertices(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.vertices_vertices.len() == 0 { return Ok(()); }

	let vertices_vertices_map = map(&graph.vertices_vertices);
	validate!(vertices_vertices_map, graph.vertices_vertices, VV);

	return Ok(());
}

pub fn validate_vertices_edges_with_edges_vertices(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.vertices_edges.len() == 0 { return Ok(()); }
	if graph.edges_vertices.len() == 0 { return Ok(()); }
	/*
	let edges_vertices_map = map(&graph.edges_vertices);
	validate!(edges_vertices_map, graph.vertices_edges, VEEV);

	let vertices_edges_map = map(&graph.vertices_edges);
	validate!(vertices_edges_map, graph.edges_vertices, EVVE);
	*/
	return Ok(());
}

pub fn validate_vertices_faces_with_faces_vertices(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.vertices_faces.len() == 0 { return Ok(()); }
	if graph.faces_vertices.len() == 0 { return Ok(()); }

	let faces_vertices_map = map(&graph.faces_vertices);
	validate_with_null!(faces_vertices_map, graph.vertices_faces, VFFV);

	let vertices_faces_map = map_with_null(&graph.vertices_faces);
	validate!(vertices_faces_map, graph.faces_vertices, FVVF);

	return Ok(());
}

pub fn validate_edges_faces_with_faces_edges(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.edges_faces.len() == 0 { return Ok(()); }
	if graph.faces_edges.len() == 0 { return Ok(()); }

	let faces_edges_map = map(&graph.faces_edges);
	validate_with_null!(faces_edges_map, graph.edges_faces, EFFE);

	let edges_faces_map = map_with_null(&graph.edges_faces);
	validate!(edges_faces_map, graph.faces_edges, FEEF);

	return Ok(());
}

pub fn validate_edges_assignment_with_edges_fold_angle(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.edges_assignment.len() == 0 { return Ok(()); }
	if graph.edges_fold_angle.len() == 0 { return Ok(()); }

	assert!(graph.edges_assignment.len() == graph.edges_fold_angle.len());

	for (edge_index, edge_assignment) in graph.edges_assignment.iter().enumerate() {
		let fold_angle = &graph.edges_fold_angle[edge_index];
		match edge_assignment {
			EdgeAssignment::Mountain => if *fold_angle > 0.0 {
				return Err(ReflexiveError::EFA(edge_index));
			},
			EdgeAssignment::Valley => if *fold_angle < 0.0 {
				return Err(ReflexiveError::EFA(edge_index));
			},
			_ => if *fold_angle != 0.0 {
				return Err(ReflexiveError::EFA(edge_index));
			}
		}
	}

	return Ok(());
}

pub fn validate_faces_faces_with_faces_faces(graph: &Graph) -> Result<(), ReflexiveError> {
	if graph.faces_faces.len() == 0 { return Ok(()); }

	let faces_faces_map = map_with_null(&graph.faces_faces);
	validate_with_null!(faces_faces_map, graph.faces_faces, FF);

	return Ok(());
}