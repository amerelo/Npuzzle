use gameview_module::gameview;

#[derive(Debug)]
pub struct Number
{
	pub value: i32,
	pub x: f64,
	pub y: f64,
}

impl Number {
	pub fn update_pos(&mut self, x: f64, y: f64) {
		self.x = x;
		self.y = y;
	}
}

pub struct Puzzle {
	pub len: usize,
	pub numbers: Vec<Number>,
}

impl Puzzle {

	pub fn init_pos(&mut self) -> ()
	{
		let square_len:f64 = gameview::get_square_len(&self, [0.0; 2], 880.0);
		let start = [square_len / 2.0, square_len / 2.0];
		let mut y: f64 = start[1];
		let mut x: f64 = 0.0;
		let mut l: usize = 0;
		let puzzle_len = self.get_len();

		for element in self.get_numbers().iter_mut() {
			if l == puzzle_len {
				x = square_len / 2.0;
				y += square_len;
				l = 0;
			} else {
				x = start[0] + l as f64 / puzzle_len as f64 * 880.0;
			}
			if element.value != 0 {
				element.update_pos(x, y);
			} else {
				element.update_pos(x - 20.0, y - 20.0);
			}
			l += 1;
		}
	}

	pub fn get_len(&self) -> (usize) {
		self.len
	}

	pub fn get_numbers(&mut self) -> (&mut Vec<Number>) {
		&mut self.numbers
	}

	pub fn solve_puzzle(&mut self)
	{
		println!("Solving the puzzle...");
		self.init_pos(); // init Vec<Number> with value and graphics positions
		// let finalboard: Vec<Position> = self.get_last_pos(self.len as i32);
		//
		// println!("get final pos of 8 {:?}", finalboard[8 - 1]);
		//
		// println!("get final pos of 1 {:?}", finalboard[1 - 1]);
		//
		// println!("get final pos of 15 {:?}", finalboard[15 - 1]);
		//
		// println!("|vec {:?}", finalboard);
	}


	//find better solution
	// fn get_last_pos(&self, size: i32) -> (Vec<Position>)
	// {
	// 	let mut board: Vec<Position> = Vec::new();
	//
	// 	let mut max_x = size - 1;
	// 	let mut min_y = 0;
	// 	let mut c_x = 0;
	// 	let mut c_y = 0;
	// 	let mut r = true;
	//
	// 	for x in 0..(size * size)
	// 	{
	// 		let elem = Position { x: c_x, y: c_y};
	// 		if r == true {
	// 			if c_x < max_x {
	// 				c_x += 1;
	// 			} else if c_x == max_x && c_y < max_x {
	// 				c_y += 1;
	// 				if c_x == max_x && c_y == max_x {
	// 					r = false;
	// 					max_x -= 1;
	// 				}
	// 			}
	// 		}
	// 		else {
	// 		 	if c_x > 0 {
	// 				c_x -= 1;
	// 			} else if c_x == 0 && c_y > min_y {
	// 				c_y -= 1;
	// 				if c_x == 0 && c_y == min_y + 1 {
	// 					r = true;
	// 					min_y += 1;
	// 				}
	// 			}
	// 		}
	// 		board.push(elem);
	// 	}
	// 	board
	// 	println!("|vec {:?}", board);
	// 	println!("size {} vec size {}", size, board.len());
	// }
}
