use serde::{Serialize, Deserialize, Deserializer};
use serde_repr::{Serialize_repr, Deserialize_repr};
use std::collections::HashMap;
use serde_json::Value;

use super::validation::*;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Frame {
	#[serde(rename = "frame_author")]
	#[serde(skip_serializing_if = "String::is_empty")]
	pub author: String,

	#[serde(rename = "frame_title")]
	#[serde(skip_serializing_if = "String::is_empty")]
	pub title: String,

	#[serde(rename = "frame_description")]
	#[serde(skip_serializing_if = "String::is_empty")]
	pub description: String,

	#[serde(rename = "frame_classes")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub classes: Vec<String>,

	#[serde(rename = "frame_attributes")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub attributes: Vec<String>,

	#[serde(rename = "frame_unit")]
	#[serde(skip_serializing_if = "String::is_empty")]
	pub unit: String,

	#[serde(rename = "vertices_coords")]
	#[serde(deserialize_with = "deserialize_vertices_coordinates")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub vertices_coordinates: Vec<[f64; 3]>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub vertices_vertices: Vec<Vec<usize>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub vertices_edges: Vec<Vec<usize>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub vertices_faces: Vec<Vec<Option<usize>>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_vertices: Vec<[usize; 2]>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_faces: Vec<Vec<Option<usize>>>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_assignment: Vec<EdgeAssignment>,

	#[serde(alias = "edges_foldAngles")] // Version 1.0 -> Version 1.1
	#[serde(rename = "edges_foldAngle")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_fold_angle: Vec<f64>,

	#[serde(alias = "edges_lengths")] // Version 1.0 -> Version 1.1
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub edges_length: Vec<f64>,

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

	#[serde(rename = "frame_parent")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<usize>,

	#[serde(rename = "frame_inherit")]
	#[serde(skip_serializing_if = "std::ops::Not::not")]
	pub inherit: bool,

	#[serde(flatten)]
	pub custom_data: HashMap<String, Value>,
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

fn deserialize_vertices_coordinates<'de, D>(deserializer: D) -> Result<Vec<[f64; 3]>, D::Error> where D: Deserializer<'de> {
	let deserialized_coordinates = Vec::<Vec<f64>>::deserialize(deserializer)?;
	let vertices_coordinates = deserialized_coordinates.into_iter().map(|vertex_coordinates| {
		let mut coordinates = [0.0f64; 3];
		for index in 0..3 {
			match vertex_coordinates.get(index) {
				Some(&c) => coordinates[index] = c,
				None => continue
			}
		}
		return coordinates;
	}).collect();
	return Ok(vertices_coordinates);
}

impl Frame {
	pub fn new() -> Self {
		return Self { ..Default::default() }
	}

	pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
		return serde_json::from_str(s);
	}

	pub fn validate(&self) -> bool {
		let vertices_count = self.vertices_coordinates.len();
		let edges_count = self.edges_vertices.len();
		let faces_count = self.faces_vertices.len();

		let is_abstract = self.attributes.iter().any(|a| a == "abstract");

		if !(is_abstract && vertices_count == 0) {
			if !validate_length(&self.vertices_edges, vertices_count) { return false; }
			if !validate_length(&self.vertices_faces, vertices_count) { return false; }
			if !validate_length(&self.vertices_vertices, vertices_count) { return false; }
		}
		if !validate_length(&self.edges_faces, edges_count) { return false; }
		if !validate_length(&self.edges_assignment, edges_count) { return false; }
		if !validate_length(&self.edges_fold_angle, edges_count) { return false; }
		if !validate_length(&self.edges_length, edges_count) { return false; }
		if !validate_length(&self.faces_edges, faces_count) { return false; }
		if !validate_length(&self.faces_faces, faces_count) { return false; }

		if self.faces_vertices.iter().any(|f| { f.len() < 3 }) { return false; }

		if !(is_abstract && vertices_count == 0) {
			if !validate_ids(&self.vertices_vertices, vertices_count) { return false; }
			if !validate_edge_ids(&self.edges_vertices, vertices_count) { return false; }
			if !validate_ids(&self.faces_vertices, vertices_count) { return false; }
		}
		if !validate_ids(&self.vertices_edges, edges_count) { return false; }
		if !validate_option_ids(&self.vertices_faces, faces_count) { return false; }
		if !validate_option_ids(&self.edges_faces, faces_count) { return false; }
		if !validate_order_ids(&self.edge_orders, edges_count) { return false; }
		if !validate_ids(&self.faces_edges, edges_count) { return false; }
		if !validate_option_ids(&self.faces_faces, faces_count) { return false; }
		if !validate_order_ids(&self.face_orders, faces_count) { return false; }

		if self.edges_fold_angle.iter().any(|&a| a < -180.0 || a > 180.0) { return false; }
		if self.edges_length.iter().any(|&l| l < 0.0) { return false; }

		if !validate_vertices_edges_with_vertices(self) { return false; }
		if !validate_vertices_faces_with_vertices(self) { return false; }
		if !validate_vertices_faces_with_edges(self) { return false; }
		if !validate_edges_faces(self) { return false; }
		if !validate_faces_edges_with_vertices(self) { return false; }
		if !validate_faces_faces_with_edges(self) { return false; }

		return true;
	}

	pub fn inherit_properties(&mut self, frame: &Self) {
		macro_rules! inherit_property {
			($s: expr, $f: expr, $p: ident) => {
				if $s.$p.is_empty() { $s.$p = $f.$p.clone(); }
			};
		}

		inherit_property!(self, frame, author);
		inherit_property!(self, frame, title);
		inherit_property!(self, frame, description);
		inherit_property!(self, frame, classes);
		inherit_property!(self, frame, attributes);
		inherit_property!(self, frame, unit);
		inherit_property!(self, frame, vertices_coordinates);
		inherit_property!(self, frame, vertices_vertices);
		inherit_property!(self, frame, vertices_edges);
		inherit_property!(self, frame, vertices_faces);
		inherit_property!(self, frame, edges_vertices);
		inherit_property!(self, frame, edges_faces);
		inherit_property!(self, frame, edges_assignment);
		inherit_property!(self, frame, edges_fold_angle);
		inherit_property!(self, frame, edges_length);
		inherit_property!(self, frame, edge_orders);
		inherit_property!(self, frame, faces_vertices);
		inherit_property!(self, frame, faces_edges);
		inherit_property!(self, frame, faces_faces);
		inherit_property!(self, frame, face_orders);

		if self.parent.is_none() { self.parent = frame.parent; }
		self.inherit = frame.inherit;

		for (key, value) in frame.custom_data.iter() {
			if !self.custom_data.contains_key(key) {
				self.custom_data.insert(key.clone(), value.clone());
			}
		}
	}
}
