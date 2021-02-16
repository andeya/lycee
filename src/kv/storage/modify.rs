// Modify is a single modification to TinyKV's underlying storage.
pub enum Modify {
	Put {
		key: Vec<u8>,
		value: Vec<u8>,
		cf: String,
	},
	Delete {
		key: Vec<u8>,
		cf: String,
	},
}


impl Modify {
	pub fn key(&self) -> &Vec<u8> {
		match self {
			Modify::Put { key, .. } => key,
			Modify::Delete { key, .. } => key,
		}
	}
	pub fn value(&self) -> Option<&Vec<u8>> {
		match self {
			Modify::Put { value, .. } => Some(value),
			_ => None,
		}
	}
	pub fn cf(&self) -> &String {
		match self {
			Modify::Put { cf, .. } => cf,
			Modify::Delete { cf, .. } => cf,
		}
	}
}
