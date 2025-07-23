use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Real(f64);

impl Real {
	pub fn from_str(str: &str) -> Self {
		return Real(str.parse().unwrap());
	}

	pub fn to_string(&self) -> String {
		return f64::to_string(&self.0);
	}
}