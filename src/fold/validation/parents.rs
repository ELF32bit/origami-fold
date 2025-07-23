use crate::fold::Fold;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum ParentError {
	MissingParent(usize, usize),
	InfiniteParents(usize),
}

pub fn validate_frame_parents(fold: &Fold, frame_index: usize) -> Result<(), ParentError> {
	let mut frame = fold.get_frame(frame_index).unwrap();
	let mut frame_parents = HashSet::from([frame_index]);
	loop {
		match frame.parent {
			Some(id) => {
				match fold.get_frame(id) {
					Some(frame_parent) => {
						if frame_parents.contains(&id) {
							return Err(ParentError::InfiniteParents(frame_index));
						}
						frame_parents.insert(id);
						frame = frame_parent;
					}
					None => return Err(ParentError::MissingParent(frame_index, id))
				}
			}
			None => break
		}
	}
	return Ok(());
}