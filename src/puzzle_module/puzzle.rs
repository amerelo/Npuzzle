use gameview_module::gameview;

#[derive(Debug)]
#[derive(Clone)]
pub struct Number
{
	pub value: i32,
	pub x: f64,
	pub y: f64,
	pub h: i32,
	pub x_base: i32,
	pub y_base: i32,
}

impl Number {
	pub fn update_pos(&mut self, x: f64, y: f64) {
		self.x = x;
		self.y = y;
	}
	pub fn update_base(&mut self, x_base: i32, y_base: i32) {
		self.x_base = x_base;
		self.y_base = y_base;
	}
}

#[derive(Debug)]
pub struct Elem {
	pub list: Vec<Number>,
	pub glob_heuristic: i32,
	pub step: i32,
	pub id: i32,
	pub p_id: i32,
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
		let mut base_x: i32 = 0;
		let mut base_y: i32 = 0;
		let puzzle_len = self.get_len();

		for element in self.get_numbers().iter_mut() {
			if l == puzzle_len {
				x = square_len / 2.0;
				y += square_len;
				l = 0;
				base_y += 1;
				base_x = 0;
			} else {
				x = start[0] + l as f64 / puzzle_len as f64 * 880.0;
			}
			if element.value != 0 {
				element.update_pos(x, y);
			} else {
				element.update_pos(x - 20.0, y - 20.0);
			}
			element.update_base(base_x, base_y);
			l += 1;
			base_x += 1;
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
		// println!("{:?}", self.numbers);
		let finalboard: Vec<Number> = self.get_last_pos(self.len as i32);
		let elem: Elem = Elem {
			list: self.numbers.to_vec(),
			glob_heuristic: 0,
			step: 0,
			id: 0,
			p_id: 0,
		};

		self.open_list.push(elem);
		self.get_manhattan_heuristic(&finalboard, 0, 0);

		self.a_star(&finalboard);
		// println!("close_list {:?}", close_list);
		// println!("open_list {:?}", self.open_list);
	}

	fn in_close_list(&self, board: &Vec<Number>) ->(bool)
	{
		let mut diff = true;

		'outer: for elem in self.close_list.iter() {
			diff = false;
			'inner: for (i, section) in elem.list.iter().enumerate() {
				// print!("v {} vc {} ", board[i].value, section.value);
				if board[i].value != section.value {
					diff = true;
					break 'inner;
				}
			}
			if diff == false{
				// println!("\nfalse");
				return false;
			}
		}
		// println!("\ntrue");
		return diff;
	}

	fn move_elem(&mut self, finalboard: &Vec<Number>, board: &Vec<Number>, a:usize, b:usize, s: i32, id: i32)
	{
		let mut newboard: Vec<Number> = board.to_vec();
		let tmp: i32 = board[a].value;
		let index: usize = self.open_list.len();

		newboard[a].value = newboard[b].value;
		newboard[b].value = tmp;
		if self.in_close_list(&newboard) {
			let elem: Elem = Elem {
				list: newboard.to_vec(),
				glob_heuristic: 0,
				step: s,
				id: 0,
				p_id: id,
			};

			// need to just modify M heuristic not all of the list
			self.open_list.push(elem);
			self.get_manhattan_heuristic(&finalboard, index, s);
		}
	}

	// get zero pos previous this fn
	fn find_move(&mut self, finalboard: &Vec<Number>, board: &Vec<Number>, step: i32, id: i32)
	{
		let len = self.len;
		let board_size = len * len;

		for (i, elem) in board.iter().enumerate()
		{
			if elem.value == 0 {
				// Up
				if (i as i32 - len as i32) >= 0 {
					self.move_elem(&finalboard, &board, i, i - len, step, id)
				}
				// Down
				if i + len < board_size{
					self.move_elem(&finalboard, &board, i, i + len, step, id)
				}
				// Left
				if (i as i32 - 1) > 0 && i / len == (i - 1) / len {
					self.move_elem(&finalboard, &board, i, i - 1, step, id)
				}
				// Right
				if i / len == (i + 1) / len && i < board_size {
					self.move_elem(&finalboard, &board, i, i + 1, step, id)
				}
				break;
			}
		}
	}

	fn get_final_path(&mut self, last_id: &i32)
	{
		let mut id :i32 = *last_id;
		let mut elem_id :i32 = id;

		loop {
			for elem in self.close_list.iter() {
				if elem.id == id {
					let board: Elem = Elem {
						list: elem.list.to_vec(),
						glob_heuristic: elem.glob_heuristic,
						step: elem.step,
						id: elem.id,
						p_id: elem.p_id,
					};
					id = elem.p_id;
					elem_id = elem.id;
					self.final_list.push(board);
					break;
				}
			}
			if elem_id == 0 {
				break;
			}
		}

		self.final_list.reverse();
	}

	fn a_star(&mut self, finalboard: &Vec<Number>)
	{
		let mut id : i32 = 0;
		loop {
			println!("loop {}", id);
			let index = self.index_to_study();
			let step = self.open_list[index].step + 1;

			let board_study: Elem = Elem {
				list: self.open_list[index].list.to_vec(),
				glob_heuristic: self.open_list[index].glob_heuristic,
				step: self.open_list[index].step,
				id: id,
				p_id: self.open_list[index].p_id,
			};

			if self.open_list[index].glob_heuristic == 0 {
				self.close_list.push(board_study);
				self.open_list.iter().enumerate().position(|t| t.0 == index).map(|e| self.open_list.remove(e));

				self.get_final_path(&id);
				break;
			}

			self.find_move(&finalboard, &board_study.list, step, id);
			self.close_list.push(board_study);
			self.open_list.iter().enumerate().position(|t| t.0 == index).map(|e| self.open_list.remove(e));
			id += 1;
		}
		println!("Maximum number of states ever represented in memory {}", id);
		// println!("len of open_list {:?}", self.open_list.len());
		// println!("len of close_list {:?}", self.close_list.len());
	}

	fn get_manhattan_heuristic(&mut self, finalboard: &Vec<Number>, index: usize, step: i32)
	{
		let mut global_h = 0;

		// return abs(a.x - b.x) + abs(a.y - b.y)
		for e in self.open_list[index].list.iter_mut()
		{
			if e.value != 0
			{
				e.h = (e.x_base - finalboard[e.value as usize - 1].x as i32).abs() + (e.y_base - finalboard[e.value as usize - 1].y as i32).abs();
				// e.h += (e.x_base - finalboard[e.value as usize - 1].x_base as i32).abs() + (e.y_base - finalboard[e.value as usize - 1].y_base as i32).abs();
				global_h += e.h;
			}
			// println!(" value {}, posx {}, posy {}, heuristic {}", e.value, e.x_base, e.y_base, e.h);
		}
		// println!("G_heuristic {} step {}", global_h, step);
		self.open_list[index].glob_heuristic = global_h;
	}

	fn index_to_study(&self) -> (usize)
	{
		let mut index: usize = 0;
		// let mut step: i32 = self.open_list[index].step;
		let mut heuristic: i32 = self.open_list[index].glob_heuristic;

		for (i, elem) in self.open_list.iter().enumerate()
		{
			// if heuristic + step > elem.glob_heuristic + elem.step{
			if heuristic >= elem.glob_heuristic {
				// step = elem.step;
				heuristic = elem.glob_heuristic;
				index = i;
			}
		}

		// println!("step {}", self.open_list[index].step);
		// println!("heuristic {}", heuristic);
		// let mut y: i32 = 0;
		// for elem in self.open_list[index].list.iter() {
		// 	if y != elem.y_base {
		// 		y += 1;
		// 		println!("");
		// 	}
		// 	print!("{} ", elem.value);
		// }
		// println!("\n");

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
				Number {value: x, x: c_x, y: c_y, h: 0, x_base: 0, y_base: 0}
			} else {
				Number {value: 0, x: c_x, y: c_y, h: 0, x_base: 0, y_base: 0}
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

		for elem in self.numbers.iter()
		{
			if elem.value != 0 {
				board[elem.value as usize -1].x_base = elem.x_base;
				board[elem.value as usize -1].y_base = elem.y_base;
			}
		}

		return board;
	}
}
