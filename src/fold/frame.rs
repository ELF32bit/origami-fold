use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

use super::graph::Graph;
use super::validation::Error;

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

	#[serde(flatten)]
	pub graph: Graph,

	#[serde(rename = "frame_parent")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<usize>,

	#[serde(rename = "frame_inherit")]
	#[serde(skip_serializing_if = "std::ops::Not::not")]
	pub inherit: bool,

	#[serde(flatten)]
	pub custom_data: Map<String, Value>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum FrameClass {
	#[serde(rename = "creasePattern")]
	CreasePattern,
	#[serde(rename = "foldedForm")]
	FoldedForm,
	#[serde(rename = "graph")]
	Graph,
	#[serde(rename = "linkage")]
	Linkage,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum FrameAttribute {
	#[serde(rename = "2D")]
	TwoDimensional,
	#[serde(rename = "3D")]
	ThreeDimensional,
	#[serde(rename = "abstract")]
	Abstract,
	#[serde(rename = "manifold")]
	Manifold,
	#[serde(rename = "nonManifold")]
	NonManifold,
	#[serde(rename = "orientable")]
	Orientable,
	#[serde(rename = "nonOrientable")]
	NonOrientable,
	#[serde(rename = "selfTouching")]
	SelfTouching,
	#[serde(rename = "nonSelfTouching")]
	NonSelfTouching,
	#[serde(rename = "selfIntersecting")]
	SelfIntersecting,
	#[serde(rename = "nonSelfIntersecting")]
	NonSelfIntersecting,
	#[serde(rename = "cuts")]
	Cuts,
	#[serde(rename = "noCuts")]
	NoCuts,
	#[serde(rename = "joins")]
	Joins,
	#[serde(rename = "noJoins")]
	NoJoins,
	#[serde(rename = "convexFaces")]
	ConvexFaces,
	#[serde(rename = "nonConvexFaces")]
	NonConvexFaces,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum FrameUnit {
	#[serde(rename = "unit")]
	Unit,
	#[serde(rename = "in")]
	Inch,
	#[serde(rename = "pt")]
	PostScriptPoints,
	#[serde(rename = "m")]
	Meters,
	#[serde(rename = "cm")]
	Centimeters,
	#[serde(rename = "mm")]
	Millimeters,
	#[serde(rename = "um")]
	Microns,
	#[serde(rename = "nm")]
	Nanometers,
}

impl Frame {
	pub fn new() -> Self {
		return Self { ..Default::default() }
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

		self.graph.inherit_properties(&frame.graph);

		if self.parent.is_none() { self.parent = frame.parent; }
		self.inherit = frame.inherit;

		for (key, value) in frame.custom_data.iter() {
			if !self.custom_data.contains_key(key) {
				self.custom_data.insert(key.clone(), value.clone());
			}
		}
	}

	pub fn validate(&self) -> Result<(), Error> {
		self.graph.validate()?;
		return Ok(());
	}
}
