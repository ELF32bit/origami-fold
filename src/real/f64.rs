use serde::{Serialize, Deserialize};
use core::cmp::{PartialEq, PartialOrd, Ordering};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Real(f64);

impl Real {
	pub fn from_str(str: &str) -> Self {
		return Real(str.parse().unwrap());
	}

	pub fn to_string(&self) -> String {
		return self.0.to_string();
	}
}

impl PartialEq<f64> for Real {
	fn eq(&self, other: &f64) -> bool {
		return f64::eq(self.0, other);
	}

	fn ne(&self, other: &f64) -> bool {
		return f64::ne(&self.0, other);
	}
}

impl PartialOrd<f64> for Real {
	fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
		return Float::partial_cmp(&self.0, other);
	}

	fn lt(&self, other: &f64) -> bool {
		return Float::lt(&self.0, other);
	}

	fn le(&self, other: &f64) -> bool {
		return Float::le(&self.0, other);
	}

	fn gt(&self, other: &f64) -> bool {
		return Float::gt(&self.0, other);
	}

	fn ge(&self, other: &f64) -> bool {
		return Float::ge(&self.0, other);
	}
}