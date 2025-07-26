use crate::fold::Graph;

#[derive(Clone, Copy, Debug)]
pub enum LengthError {
	VV,
	VE,
	VF,
	EF,
	EA,
	EFA,
	EL,
	FE,
	FF,
}

macro_rules! validate {
	($method: ident, $source: ident, $destination: ident, $error: ident) => {
		pub fn $method(graph: &Graph) -> Result<(), LengthError> {
			let l1 = graph.$source.len();
			let l2 = graph.$destination.len();
			if !(l1 == 0 || l1 == l2) { return Err(LengthError::$error) }
			return Ok(());
		}
	};
}

validate!(validate_vertices_vertices_length,vertices_vertices, vertices_coordinates, VV);
validate!(validate_vertices_edges_length, vertices_edges, vertices_coordinates, VE);
validate!(validate_vertices_faces_length, vertices_faces, vertices_coordinates, VF);

validate!(validate_edges_faces_length, edges_faces, edges_vertices, EF);
validate!(validate_edges_assignment_length, edges_assignment, edges_vertices, EA);
validate!(validate_edges_fold_angle_length, edges_fold_angle, edges_vertices, EFA);
validate!(validate_edges_length_length, edges_length, edges_vertices, EL);

validate!(validate_faces_edges_length, faces_edges, faces_vertices, FE);
validate!(validate_faces_faces_length, faces_faces, faces_vertices, FF);
