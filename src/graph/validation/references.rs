use crate::fold::Graph;

#[derive(Clone, Copy, Debug)]
pub enum ReferencesError {
	VV(usize, usize),
	VE(usize, usize),
	VF(usize, usize),
	EV(usize, usize),
	EF(usize, usize),
	EO(usize, usize),
	FV(usize, usize),
	FE(usize, usize),
	FF(usize, usize),
	FO(usize, usize),
}

macro_rules! validate {
	($method: ident, $source: ident, $destination: ident, $error: ident) => {
		pub fn $method(graph: &Graph) -> Result<(), ReferencesError> {
			let max_index = graph.$destination.len();
			for &index in graph.$source.iter().flatten() {
				if index >= max_index {
					return Err(ReferencesError::$error(index, max_index));
				}
			}
			return Ok(());
		}
	};
}

macro_rules! validate_with_null {
	($method: ident, $source: ident, $destination: ident, $error: ident) => {
		pub fn $method(graph: &Graph) -> Result<(), ReferencesError> {
			let max_index = graph.$destination.len();
			for &index_option in graph.$source.iter().flatten() {
				if index_option.is_none() { continue; }
				let index = index_option.unwrap();
				if index >= max_index {
					return Err(ReferencesError::$error(index, max_index));
				}
			}
			return Ok(());
		}
	};
}

macro_rules! validate_orders {
	($method: ident, $source: ident, $destination: ident, $error: ident) => {
		pub fn $method(graph: &Graph) -> Result<(), ReferencesError> {
			let max_index = graph.$destination.len();
			for &order in graph.$source.iter() {
				if order.0 >= max_index {
					return Err(ReferencesError::$error(order.0, max_index));
				}
				if order.1 >= max_index {
					return Err(ReferencesError::$error(order.1, max_index));
				}
			}
			return Ok(());
		}
	};
}

validate!(validate_vertices_vertices_indices, vertices_vertices, vertices_coordinates, VV);
validate!(validate_vertices_edges_indices, vertices_edges, edges_vertices, VE);
validate_with_null!(validate_vertices_faces_indices, vertices_faces, faces_vertices, VF);

validate!(validate_edges_vertices_indices, edges_vertices, vertices_coordinates, EV);
validate_with_null!(validate_edges_faces_indices, edges_faces, faces_vertices, EF);
validate_orders!(validate_edge_orders_indices, edge_orders, edges_vertices, EO);

validate!(validate_faces_vertices_indices, faces_vertices, vertices_coordinates, FV);
validate!(validate_faces_edges_indices, faces_edges, edges_vertices, FE);
validate_with_null!(validate_faces_faces_indices, faces_faces, faces_vertices, FF);
validate_orders!(validate_face_orders_indices, face_orders, faces_vertices, FO);