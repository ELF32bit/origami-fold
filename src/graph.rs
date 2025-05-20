use serde::{de, Deserialize, Deserializer, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_json::Number;

use super::validation;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Graph {
	#[serde(rename = "vertices_coords")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	#[serde(deserialize_with = "deserialize_vertices_coordinates")]
	pub vertices_coordinates: Vec<Vec<Number>>,

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

	#[serde(alias = "edges_foldAngles")] // Version 1.0 -> Version 1.1
	#[serde(rename = "edges_foldAngle")]
	#[serde(deserialize_with = "deserialize_edges_fold_angle")] // Not specification-compliant!
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_fold_angle: Vec<Number>,

	#[serde(alias = "edges_lengths")] // Version 1.0 -> Version 1.1
	#[serde(skip_serializing_if = "Vec::is_empty")]
	#[serde(deserialize_with = "deserialize_edges_length")]
	pub edges_length: Vec<Number>,

	#[serde(rename = "edgeOrders")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edge_orders: Vec<(usize, usize, EdgeOrder)>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	#[serde(deserialize_with = "deserialize_faces_vertices")]
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

fn deserialize_vertices_coordinates<'de, D>(deserializer: D) -> Result<Vec<Vec<Number>>, D::Error> where D: Deserializer<'de> {
	let vertices_coordinates = Vec::<Vec<Number>>::deserialize(deserializer)?;
	let vertices_coordinates_length = vertices_coordinates.len();
	if vertices_coordinates_length > 0 {
		let coordinates_length = vertices_coordinates[0].len();
		for index in 1..vertices_coordinates_length {
			if vertices_coordinates[index].len() != coordinates_length {
				return Err(de::Error::custom("TODO: "));
			}
		}
	}
	return Ok(vertices_coordinates);
}

fn deserialize_edges_fold_angle<'de, D>(deserializer: D) -> Result<Vec<Number>, D::Error> where D: Deserializer<'de> {
	let deserialized_edges_fold_angle = Vec::<Option<Number>>::deserialize(deserializer)?;
	let edges_fold_angle = deserialized_edges_fold_angle.into_iter().map(|fold_angle| {
		match fold_angle {
			Some(angle) => return angle,
			None => return Number::from_f64(0.0).unwrap()
		}
	}).collect();
	return Ok(edges_fold_angle);
}

fn deserialize_edges_length<'de, D>(deserializer: D) -> Result<Vec<Number>, D::Error> where D: Deserializer<'de> {
	let edges_length = Vec::<Number>::deserialize(deserializer)?;
	for edge_length in edges_length.iter() {
		match edge_length.as_f64() {
			Some(length) => if length < 0.0 {
				return Err(de::Error::custom("TODO: "));
			},
			None => return Err(de::Error::custom("TODO: "))
		}
	}
	return Ok(edges_length);
}

fn deserialize_faces_vertices<'de, D>(deserializer: D) -> Result<Vec<Vec<usize>>, D::Error> where D: Deserializer<'de> {
	let faces_vertices = Vec::<Vec<usize>>::deserialize(deserializer)?;
	for face_vertices in faces_vertices.iter() {
		if face_vertices.len() < 3 {
			return Err(de::Error::custom("TODO: "));
		}
	}
	return Ok(faces_vertices);
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

	pub fn validate(&self) -> Result<(), validation::Error> {
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
		validation::validate_faces_faces_with_faces_faces(self)?;

		validation::validate_vertices_vertices_and_vertices_edges_winding(self)?;
		validation::validate_vertices_vertices_and_vertices_faces_winding(self)?;
		validation::validate_vertices_edges_and_vertices_faces_winding(self)?;
		validation::validate_faces_vertices_and_faces_edges_winding(self)?;
		validation::validate_faces_vertices_and_faces_faces_winding(self)?;
		validation::validate_faces_edges_and_faces_faces_winding(self)?;

		validation::validate_edges_assignment_with_edges_fold_angle(self)?;

		return Ok(());
	}
}