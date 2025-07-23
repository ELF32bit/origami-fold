use serde::{Serialize, Deserialize};

use crate::real::Real;
use super::frame::Frame;
use super::validation::Error;
use super::validation::validate_frame_parents;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Fold {
	#[serde(rename = "file_spec")]
	pub version: Real,

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
	pub key_frame: Frame,

	#[serde(rename = "file_frames")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub frames: Vec<Frame>,
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

impl Default for Fold {
	fn default() -> Self {
		return Self {
			version: Real::from_str("1.2"),
			creator: Default::default(),
			author: Default::default(),
			title: Default::default(),
			description: Default::default(),
			classes: Default::default(),
			key_frame: Default::default(),
			frames: Default::default(),
		}
	}
}

impl Fold {
	pub fn new() -> Self {
		return Self { ..Default::default() }
	}

	pub fn get_frame(&self, frame_index: usize) -> Option<&Frame> {
		if frame_index == 0 {
			return Some(&self.key_frame);
		} else if frame_index - 1 < self.frames.len() {
			return Some(&self.frames[frame_index - 1]);
		} else {
			return None;
		}
	}

	pub fn get_inherited_frame(&self, frame_index: usize) -> Result<Frame, &Frame> {
		let mut frame = self.get_frame(frame_index).unwrap();
		if !frame.inherit { return Err(frame); }

		match frame.parent {
			Some(index) => if self.get_frame(index).is_none() {
				return Err(frame);
			}
			None => return Err(frame)
		}

		let mut inherited_frame = frame.clone();
		loop {
			match frame.parent {
				Some(index) => {
					let frame_parent = self.get_frame(index).unwrap();
					inherited_frame.inherit_properties(frame_parent);
					if !frame_parent.inherit { break; }
					frame = frame_parent;
				}
				None => break
			}
		}

		return Ok(inherited_frame);
	}

	pub fn get_inherited_frames(&self) -> Vec<Result<Frame, &Frame>> {
		let mut inherited_frames: Vec<Result<Frame, &Frame>> = Vec::new();

		for frame_index in 0..(1 + self.frames.len()) {
			let inherited_frame = self.get_inherited_frame(frame_index);
			inherited_frames.push(inherited_frame);
		}

		return inherited_frames;
	}

	pub fn validate(&self) -> Result<(), Error> {
		for frame_index in 0..(1 + self.frames.len()) {
			validate_frame_parents(self, frame_index)?;
			match self.get_inherited_frame(frame_index) {
				Ok(inherited_frame) => inherited_frame.validate()?,
				Err(frame) => frame.validate()?
			}
		}
		return Ok(());
	}
}