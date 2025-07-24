use serde::{Serialize, Serializer, Deserialize, Deserializer};
use core::cmp::{PartialEq, PartialOrd, Ordering};
use serde_json::Number;
use rug::Float;

#[derive(Clone, Debug)]
pub struct Real(Float, u64);

impl Serialize for Real {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		return serializer.serialize_str(&Real::to_string(self));
	}
}

impl<'de> Deserialize<'de> for Real {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let number = Number::deserialize(deserializer)?;
		return Ok(Real::from_str(&number.to_string()));
	}
}

impl Real {
	pub fn from_str(str: &str) -> Self {
		let result = Float::parse(str).unwrap();
		return Real(Float::with_val_64(256, result), 256);
	}

	pub fn to_string(&self) -> String {
		return Float::to_string(&self.0);
	}
}

impl PartialEq<f64> for Real {
	fn eq(&self, other: &f64) -> bool {
		return Float::eq(&self.0, other);
	}

	fn ne(&self, other: &f64) -> bool {
		return Float::ne(&self.0, other);
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