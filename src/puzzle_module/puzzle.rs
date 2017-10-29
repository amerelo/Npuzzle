use gameview_module::gameview;


#[derive(Debug)]
#[derive(Clone)]
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

/////
pub struct Puzzle {
	pub len: usize,
	pub numbers: Vec<Number>,
}

impl Puzzle
{

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
		let finalboard: Vec<Number> = self.get_last_pos(self.len as i32);

		let mut testv: Vec<i32> = Vec::new();

		self.test_recursive(&mut testv, 0);
		println!("last vec = {:?}", testv);
		// self.a_star(finalboard);
		// self.get_manhattan_heuristic(finalboard);
	}

	fn test_recursive(&self, vec: &mut Vec<i32>, i: i32) -> (bool)
	{
		println!("vec = {:?}", vec);
		vec.push(i);
		if i < 10 {
			self.test_recursive(vec, i + 1);
		}
		return true;
	}


	// fn a_star(&self, finalboard: Vec<Number>)
	// {
	// 	let mut test: Vec<Number> = self.numbers.to_vec();
	//
	// 	println!("|vec1 {:?}", finalboard);
	//
	// 	println!("\n|test {:?}", test);
	// }

	// fn get_manhattan_heuristic(&self, finalboard: Vec<Number>)
	// {
		// let mut h: i32 = 0;

		// return abs(a.x - b.x) + abs(a.y - b.y)

		// for (i, elem) in self.numbers.iter().enumerate()
		// {
		// 	if *elem != 0
		// 	{
		// 		println!("final pos of {} is  {:?} ", elem, finalboard[*elem as usize - 1]);
		// 	}
		// }

		// println!("heuristic {} ", h);
	// }

	//find better solution
	fn get_last_pos(&self, size: i32) -> (Vec<Number>)
	{
		let mut board: Vec<Number> = Vec::new();

		let last_elem = (size * size) + 1;
		let mut max_x = (size - 1) as f64;
		let mut min_y = 0.0;
		let mut c_x = 0.0;
		let mut c_y = 0.0;
		let mut r = true;

		for x in 1..last_elem
		{
			let elem = if x != last_elem {
				Number {value: x, x: c_x, y: c_y}
			} else {
				Number {value: 0, x: c_x, y: c_y}
			};

			if r == true {
				if c_x < max_x {
					c_x += 1.0;
				} else if c_x == max_x && c_y < max_x {
					c_y += 1.0;
					if c_x == max_x && c_y == max_x {
						r = false;
						max_x -= 1.0;
					}
				}
			}
			else {
			 	if c_x > 0.0 {
					c_x -= 1.0;
				} else if c_x == 0.0 && c_y > min_y {
					c_y -= 1.0;
					if c_x == 0.0 && c_y == min_y + 1.0 {
						r = true;
						min_y += 1.0;
					}
				}
			}
			board.push(elem);
		}
		return board;
	}
}
