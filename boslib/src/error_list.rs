use std::fmt::{Display, Formatter};

pub struct BosError {
	pub name: String,
	pub detail: String,
	pub line: u32,
	pub column: usize,
}

impl Display for BosError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}\nLine {}, column {}", self.name, self.detail, self.line, self.column)
	}
}


pub struct BosErrorList {
	list: Vec<BosError>,
}

impl BosErrorList {
	pub fn new() -> Self {
		Self {
			list: Vec::new(),
		}
	}
	
	pub fn push_error(&mut self, error: BosError) {
		self.list.push(error);
	}
}

impl Display for BosErrorList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut s = "Errors:\n".to_string();
		for e in &self.list {
			s.push_str((e.to_string() + "\n\n").as_str());
		}
		write!(f, "{}", s)
	}
}