use gameview_module::gameview;

#[derive(Debug)]
#[derive(Clone)]
pub struct Number
{
	pub value: i32,
	pub x: f64,
	pub y: f64,
	// pub posx: i32,
	// pub posy: i32,
	pub h: i32,
}

impl Number {
	pub fn update_pos(&mut self, x: f64, y: f64) {
		self.x = x;
		self.y = y;
	}
}

#[derive(Debug)]
pub struct Elem {
	pub list: Vec<Number>,
	pub glob_heuristic: i32,
	pub step: i32,
}

pub struct Puzzle {
	pub len: usize,
	pub numbers: Vec<Number>,

	pub open_list: Vec<Elem>,
	pub close_list: Vec<Elem>,
	pub final_list: Vec<Elem>,
}

impl Puzzle
{
	pub fn init_pos(&mut self) -> ()
	{
		let square_len: f64 = gameview::get_square_len(&self, [0.0; 2], 880.0);
		let start = [square_len / 2.0, square_len / 2.0];
		let mut y: f64 = start[1];
		let mut x: f64;
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
		// self.close_list.push(self.numbers.to_vec());

		let mut elem: Elem = Elem {
			list: self.numbers.to_vec(),
			glob_heuristic: 0,
			step: 0,
		};
		self.open_list.push(elem);
		self.get_manhattan_heuristic(&finalboard, 0);

		self.a_star(&finalboard);
		// println!("close_list {:?}", close_list);
		// println!("open_list {:?}", self.open_list);
	}

	fn move_elem(&mut self, finalboard: &Vec<Number>, board: &Vec<Number>, a:usize, b:usize, s: i32)
	{
		let mut newboard: Vec<Number> = board.to_vec();
		let tmp: i32 = board[a].value;
		let index: usize = self.open_list.len();

		newboard[a].value = newboard[b].value;
		newboard[b].value = tmp;

		let mut elem: Elem = Elem {
			list: newboard.to_vec(),
			glob_heuristic: 0,
			step: s,
		};
		// need to just modify M heuristic not all of the list
		self.open_list.push(elem);
		self.get_manhattan_heuristic(&finalboard, index);
	}


	// get zero pos previous this fn
	fn find_move(&mut self, finalboard: &Vec<Number>, board: &Vec<Number>, step: i32)
	{
		let len = self.len;
		let board_size = len * len;

		for (i, elem) in board.iter().enumerate()
		{
			if elem.value == 0 {
				// println!("Up {}", i - self.len);
				if (i as i32 - len as i32) >= 0 {
					self.move_elem(&finalboard, &board, i, i - len, step)
				}
				// println!("Down {}", i + len);
				if i + len < board_size{
					self.move_elem(&finalboard, &board, i, i + len, step)
				}
				// println!("Lefty {} - L-1y {}", i / len, (i - 1) / len);
				if (i as i32 - 1) > 0 && i / len == (i - 1) / len {
					self.move_elem(&finalboard, &board, i, i - 1, step)
				}
				// println!("Right {} - R+1y {}", i / len, (i + 1) / len);
				if i / len == (i + 1) / len && i < board_size {
					self.move_elem(&finalboard, &board, i, i + 1, step)
				}
				break;
			}
		}
	}

	fn a_star(&mut self, finalboard: &Vec<Number>) -> (bool)
	{
		let index = self.index_to_study();
		let step = self.open_list[index].step + 1;
		let mut board_study: Elem = Elem {
			list: self.open_list[index].list.to_vec(),
			glob_heuristic: self.open_list[index].glob_heuristic,
			step: self.open_list[index].step,
		};


		self.get_manhattan_heuristic(&finalboard, index);
		self.find_move(&finalboard, &board_study.list, step);

		println!("len of open_list {:?}", self.open_list.len());
		self.close_list.push(board_study);
		self.open_list.iter().enumerate().position(|t| t.0 == index).map(|e| self.open_list.remove(e));
		println!("len of open_list {:?}", self.open_list.len());
		return true;
	}

	fn get_manhattan_heuristic(&mut self, finalboard: &Vec<Number>, index: usize)
	{
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		let mut global_h = 0;

		// return abs(a.x - b.x) + abs(a.y - b.y)
	// need real x an y in the list to do less op
		for e in self.open_list[index].list.iter_mut()
		{
			if e.value != 0
			{
				// println!("final pos of {} is  {:?} ", self.open_list[index].value, finalboard[self.open_list[index].value as usize - 1]);
				// println!("self.open_list[index] pos of x {} pos y is {} ", x, y);
				e.h = (x - finalboard[e.value as usize - 1].x as i32).abs() + (y - finalboard[e.value as usize - 1].y as i32).abs();
				// println!("self.open_list[index] heuristic {}  ", self.open_list[index].h);
				global_h += e.h;
			}
			x += 1;
			if x == self.len as i32
			{
				y += 1;
				x = 0;
			}
		}
		self.open_list[index].glob_heuristic = global_h;
	}

	fn index_to_study(&self) -> (usize)
	{
		let mut index: usize = 0;
		let heuristic: i32 = self.open_list[index].glob_heuristic;

		for (i, elem) in self.open_list.iter().enumerate()
		{
			if heuristic > elem.glob_heuristic {
				index = i;
			}
		}
		return index;
	}

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
				Number {value: x, x: c_x, y: c_y, h: 0}
			} else {
				Number {value: 0, x: c_x, y: c_y, h: 0}
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
