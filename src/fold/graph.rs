use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::real::Real;
use super::validation::Error;
use crate::graph::validation;
use crate::graph::make;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Graph {
	#[serde(rename = "vertices_coords")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub vertices_coordinates: Vec<Vec<Real>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub vertices_vertices: Vec<Vec<usize>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub vertices_edges: Vec<Vec<usize>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub vertices_faces: Vec<Vec<Option<usize>>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_vertices: Vec<(usize, usize)>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_faces: Vec<Vec<Option<usize>>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_assignment: Vec<EdgeAssignment>,

	#[serde(rename = "edges_foldAngle")]
	#[serde(alias = "edges_foldAngles")] // Version 1.0 -> Version 1.1
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_fold_angle: Vec<Real>,

	#[serde(alias = "edges_lengths")] // Version 1.0 -> Version 1.1
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_length: Vec<Real>,

	#[serde(rename = "edgeOrders")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edge_orders: Vec<(usize, usize, EdgeOrder)>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub faces_vertices: Vec<Vec<usize>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub faces_edges: Vec<Vec<usize>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub faces_faces: Vec<Vec<Option<usize>>>,

	#[serde(rename = "faceOrders")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub face_orders: Vec<(usize, usize, FaceOrder)>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum EdgeAssignment {
	#[serde(rename = "B")]
	Boundary,
	#[serde(rename = "M")]
	Mountain,
	#[serde(rename = "V")]
	Valley,
	#[serde(rename = "F")]
	Flat,
	#[serde(rename = "U")]
	Unknown,
	#[serde(rename = "C")]
	Cut,
	#[serde(rename = "J")]
	Join,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug)]
#[repr(i8)]
pub enum EdgeOrder {
	Left = 1,
	Right = -1,
	Unknown = 0,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug)]
#[repr(i8)]
pub enum FaceOrder {
	Above = 1,
	Below = -1,
	Unknown = 0,
}

impl Graph {
	pub fn new() -> Self {
		return Self { ..Default::default() }
	}

	pub fn inherit_properties(&mut self, graph: &Self) {
		macro_rules! inherit_property {
			($s: expr, $g: expr, $p: ident) => {
				if $s.$p.is_empty() { $s.$p = $g.$p.clone(); }
			};
		}

		inherit_property!(self, graph, vertices_coordinates);
		inherit_property!(self, graph, vertices_vertices);
		inherit_property!(self, graph, vertices_edges);
		inherit_property!(self, graph, vertices_faces);
		inherit_property!(self, graph, edges_vertices);
		inherit_property!(self, graph, edges_faces);
		inherit_property!(self, graph, edges_assignment);
		inherit_property!(self, graph, edges_fold_angle);
		inherit_property!(self, graph, edges_length);
		inherit_property!(self, graph, edge_orders);
		inherit_property!(self, graph, faces_vertices);
		inherit_property!(self, graph, faces_edges);
		inherit_property!(self, graph, faces_faces);
		inherit_property!(self, graph, face_orders);
	}

	pub fn validate(&self) -> Result<(), Error> {
		validation::validate_vertices_coordinates(self)?;
		validation::validate_edges_length(self)?;
		validation::validate_edge_orders(self)?;
		validation::validate_faces_vertices(self)?;
		validation::validate_face_orders(self)?;

		validation::validate_vertices_vertices_length(self)?;
		validation::validate_vertices_edges_length(self)?;
		validation::validate_vertices_faces_length(self)?;
		validation::validate_edges_faces_length(self)?;
		validation::validate_edges_assignment_length(self)?;
		validation::validate_edges_fold_angle_length(self)?;
		validation::validate_edges_length_length(self)?;
		validation::validate_faces_edges_length(self)?;
		validation::validate_faces_faces_length(self)?;

		validation::validate_vertices_vertices_indices(self)?;
		validation::validate_vertices_edges_indices(self)?;
		validation::validate_vertices_faces_indices(self)?;
		validation::validate_edges_vertices_indices(self)?;
		validation::validate_edges_faces_indices(self)?;
		validation::validate_edge_orders_indices(self)?;
		validation::validate_faces_vertices_indices(self)?;
		validation::validate_faces_edges_indices(self)?;
		validation::validate_faces_faces_indices(self)?;
		validation::validate_face_orders_indices(self)?;

		validation::validate_vertices_vertices_with_vertices_vertices(self)?;
		validation::validate_vertices_edges_with_edges_vertices(self)?;
		validation::validate_vertices_faces_with_faces_vertices(self)?;
		validation::validate_edges_faces_with_faces_edges(self)?;
		validation::validate_edges_assignment_with_edges_fold_angle(self)?;
		validation::validate_faces_faces_with_faces_faces(self)?;

		validation::validate_vertices_vertices_and_vertices_edges_winding(self)?;
		validation::validate_vertices_vertices_and_vertices_faces_winding(self)?;
		validation::validate_vertices_edges_and_vertices_faces_winding(self)?;
		validation::validate_faces_vertices_and_faces_edges_winding(self)?;
		validation::validate_faces_vertices_and_faces_faces_winding(self)?;
		validation::validate_faces_edges_and_faces_faces_winding(self)?;

		return Ok(());
	}

	pub fn make_vertices_edges_unsorted(&mut self) {
		self.vertices_edges = make::make_vertices_edges_unsorted(&self.edges_vertices);
	}

	pub fn make_vertices_edges(&mut self) {
		self.vertices_edges = make::make_vertices_edges(&self.edges_vertices, &self.vertices_vertices);
	}
}