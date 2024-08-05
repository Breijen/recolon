pub struct Scanner {}

impl Scanner {
	pub fn new(src: &str) -> Self {
		Self {}
	}

	pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, String> {
		todo!()
	}
}

#[derive(Debug)]
pub struct Token {}