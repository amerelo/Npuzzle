pub struct Puzzle {
	pub len: usize,
	pub numbers: Vec<i32>,
}

impl Puzzle {

	pub fn get_len(&self) -> (usize) {
		self.len
	}

	pub fn get_numbers(&self) -> (&Vec<i32>) {
		&self.numbers
	}
}
