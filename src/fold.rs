use serde::{Serialize, Deserialize};
use serde_aux::field_attributes::deserialize_string_from_number;
use super::fold_frame::FoldFrame;

use super::validation::validate_frame_parents;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Fold {
	#[serde(rename = "file_spec")]
	#[serde(skip_serializing_if = "String::is_empty")]
	#[serde(deserialize_with = "deserialize_string_from_number")]
	pub version: String,

	#[serde(rename = "file_creator")]
	#[serde(skip_serializing_if = "String::is_empty")]
	pub creator: String,

	#[serde(rename = "file_author")]
	#[serde(skip_serializing_if = "String::is_empty")]
	pub author: String,

	#[serde(rename = "file_title")]
	#[serde(skip_serializing_if = "String::is_empty")]
	pub title: String,

	#[serde(rename = "file_description")]
	#[serde(skip_serializing_if = "String::is_empty")]
	pub description: String,

	#[serde(rename = "file_classes")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub classes: Vec<String>,

	#[serde(flatten)]
	pub key_frame: FoldFrame,

	#[serde(rename = "file_frames")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub frames: Vec<FoldFrame>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum FoldClass {
	#[serde(rename = "singleModel")]
	SingleModel,
	#[serde(rename = "multiModel")]
	MultiModel,
	#[serde(rename = "animation")]
	Animation,
	#[serde(rename = "diagrams")]
	Diagrams,
}

impl Fold {
	pub fn new() -> Self {
		return Self { ..Default::default() }
	}

	pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
		return serde_json::from_str(s);
	}

	pub fn validate(&self) -> bool {
		if !self.validate_frame(0) { return false; }
		for (frame_id, _) in self.frames.iter().enumerate() {
			if !self.validate_frame(frame_id + 1) { return false; }
		}
		return true;
	}

	pub fn validate_frame(&self, frame_id: usize) -> bool {
		if !validate_frame_parents(self, frame_id) { return false; }
		match self.get_inherited_frame(frame_id) {
			Some((frame, inherited_frame)) => {
				match inherited_frame {
					Some(f) => { if !f.validate() { return false; } },
					None => { if !frame.validate() { return false; } }
				}
				return true;
			},
			None => return false
		}
	}

	pub fn get_frame(&self, frame_id: usize) -> Option<&FoldFrame> {
		let frames_count = self.frames.len() + 1;
		if !(frame_id < frames_count) {
			return None;
		} else if frame_id == 0 {
			return Some(&self.key_frame);
		} else {
			return Some(&self.frames[frame_id - 1]);
		}
	}

	pub fn get_frame_parent(&self, frame_id: usize) -> Option<&FoldFrame> {
		let frame = self.get_frame(frame_id)?;
		match frame.parent {
			Some(id) => return self.get_frame(id),
			None => return None
		}
	}

	pub fn get_frame_parents(&self, frame_id: usize) -> Option<Vec<&FoldFrame>> {
		let mut frame = self.get_frame(frame_id)?;
		let mut frame_parents: Vec<&FoldFrame> = Vec::new();
		loop {
			match frame.parent {
				Some(id) => {
					let frame_parent = self.get_frame(id)?;
					frame_parents.push(frame_parent);
					frame = frame_parent;
				},
				None => break
			}
		}
		return Some(frame_parents);
	}

	pub fn get_inherited_frame(&self, frame_id: usize) -> Option<(&FoldFrame, Option<FoldFrame>)> {
		let mut frame = self.get_frame(frame_id)?;
		if !frame.inherit || self.get_frame_parent(frame_id).is_none() {
			return Some((frame, None));
		}
		let mut inherited_frame = frame.clone();
		loop {
			match frame.parent {
				Some(id) => {
					let frame_parent = self.get_frame(id)?;
					inherited_frame.inherit_properties(frame_parent);
					if !frame_parent.inherit { break; }
					frame = frame_parent;
				}
				None => break
			}
		}
		return Some((frame, Some(inherited_frame)));
	}
}
