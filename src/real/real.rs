use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::Number;

use rug::Float;

#[derive(Clone, Debug)]
pub struct Real(Float);

impl Real {
	pub fn from_str(str: &str) -> Self {
		let result = Float::parse(str).unwrap();
		return Real(Float::with_val(256, result));
	}

	pub fn to_string(&self) -> String {
		return Float::to_string_radix(&self.0, 10, None);
	}
}

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