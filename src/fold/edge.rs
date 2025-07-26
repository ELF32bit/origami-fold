use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Edge(pub usize, pub usize);

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

pub struct EdgeIterator<'a> {
	edge: &'a Edge,
	index: usize,
}

impl<'a> Iterator for EdgeIterator<'a> {
	type Item = &'a usize;
	fn next(&mut self) -> Option<Self::Item> {
		let item = match self.index {
			0 => &self.edge.0,
			1 => &self.edge.1,
			_ => return None,
		};
		self.index += 1;
		return Some(item);
	}
}

impl<'a> IntoIterator for &'a Edge {
	type Item = &'a usize;
	type IntoIter = EdgeIterator<'a>;
	fn into_iter(self) -> Self::IntoIter {
		return EdgeIterator {
			edge: self,
			index: 0,
		}
	}
}

impl Eq for Edge {}

impl PartialEq for Edge {
	fn eq(&self, other: &Self) -> bool {
		if self.0 == other.0 {
			if self.1 != other.1 { return false; }
		} else if self.0 == other.1 {
			if self.1 != other.0 { return false; }
		}
		return true;
	}
}

impl PartialEq<(usize, usize)> for Edge {
	fn eq(&self, other: &(usize, usize)) -> bool {
		let other_edge = &(other.0, other.1);
		return self == other_edge;
	}
}
