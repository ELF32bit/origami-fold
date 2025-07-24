use serde::{Serialize, Deserialize};
use core::cmp::{PartialEq, PartialOrd, Ordering};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Real(f32);

impl Real {
	pub fn from_str(str: &str) -> Self {
		return Real(str.parse().unwrap());
	}

	pub fn to_string(&self) -> String {
		return self.0.to_string();
	}
}

impl PartialEq<f32> for Real {
	fn eq(&self, other: &f32) -> bool {
		return f32::eq(&self.0, other);
	}

	fn ne(&self, other: &f32) -> bool {
		return f32::ne(&self.0, other);
	}
}

impl PartialOrd<f32> for Real {
	fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
		return f32::partial_cmp(&self.0, other);
	}

	fn lt(&self, other: &f32) -> bool {
		return f32::lt(&self.0, other);
	}

	fn le(&self, other: &f32) -> bool {
		return f32::le(&self.0, other);
	}

	fn gt(&self, other: &f32) -> bool {
		return f32::gt(&self.0, other);
	}

	fn ge(&self, other: &f32) -> bool {
		return f32::ge(&self.0, other);
	}
}