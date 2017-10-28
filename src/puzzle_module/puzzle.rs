pub struct Puzzle {
	pub len: usize,
	pub numbers: Vec<i32>,
}

#[derive(Debug)]
struct  Position
{
	x: i32,
	y: i32,
}

impl Puzzle {

	pub fn get_len(&self) -> (usize) {
		self.len
	}

	pub fn get_numbers(&self) -> (&Vec<i32>) {
		&self.numbers
	}

	pub fn get_last_pos(&self, size: i32)
	{
		let mut board: Vec<Position> = Vec::new();

		let mut max_x = size - 1;
		let mut min_y = 0;
		let mut c_x = 0;
		let mut c_y = 0;
		let mut r = true;

		for x in 0..(size * size)
		{
			let elem = Position { x: c_x, y: c_y};
			if r == true {
				if c_x < max_x {
					c_x += 1;
				} else if c_x == max_x && c_y < max_x {
					c_y += 1;
					if c_x == max_x && c_y == max_x {
						r = false;
						max_x -= 1;
					}
				}
			}
			else {
			 	if c_x > 0 {
					c_x -= 1;
				} else if c_x == 0 && c_y > min_y {
					c_y -= 1;
					if c_x == 0 && c_y == min_y + 1 {
						r = true;
						min_y += 1;
					}
				}
			}
			// println!("iterator {}", x);
			board.push(elem);
		}
		println!("|vec {:?}", board);
		println!("size {} vec size {}", size, board.len());
	}
}
