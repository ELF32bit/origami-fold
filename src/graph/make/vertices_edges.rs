use super::pairs::make_cyclical_pairs_map;
use crate::fold::Edge;

pub fn make_vertices_edges_unsorted(edges_vertices: &Vec<Edge>) -> Vec<Vec<usize>> {
	let mut max_vertex_index: usize = 0;
	for edge_vertices in edges_vertices.iter() {
		for &edge_vertex_index in [edge_vertices.0, edge_vertices.1].iter() {
			if edge_vertex_index > max_vertex_index {
				max_vertex_index = edge_vertex_index;
			}
		}
	}

	let mut vertices_edges = Vec::new();
	vertices_edges.resize(max_vertex_index, Vec::new());
	for (edge_index, edge_vertices) in edges_vertices.iter().enumerate() {
		for &edge_vertex_index in [edge_vertices.0, edge_vertices.1].iter() {
			vertices_edges[edge_vertex_index].push(edge_index);
		}
	}

	return vertices_edges;
}

pub fn make_vertices_edges(edges_vertices: &Vec<Edge>, vertices_vertices: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
	let mut vertices_edges = Vec::new();
	vertices_edges.resize(vertices_vertices.len(), Vec::new());
	let edges_vertices = make_cyclical_pairs_map(edges_vertices);

	for (vertex_index, vertex_vertices) in vertices_vertices.iter().enumerate() {
		for &vertex_vertex_index in vertex_vertices.iter() {
			let edge = (vertex_index, vertex_vertex_index);
			match edges_vertices.get(&edge) {
				Some(&edge_index) => vertices_edges[vertex_index].push(edge_index),
				None => return Vec::new()
			}
		}
	}

	return vertices_edges;
}