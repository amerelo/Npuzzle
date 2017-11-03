use gameview_module::gameview;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Number
{
	pub value: i32,
	// pub x: f64,
	// pub y: f64,
	pub x_base: i32,
	pub y_base: i32,
	pub h: i32,
}

impl Number {
	// pub fn update_pos(&mut self, x: f64, y: f64) {
	// 	self.x = x;
	// 	self.y = y;
	// }
	pub fn update_base(&mut self, x_base: i32, y_base: i32) {
		self.x_base = x_base;
		self.y_base = y_base;
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Elem {
	pub list: Vec<Number>,
	pub glob_heuristic: i32,
	pub total_cost: i32,
	pub step: i32,
	pub id: i32,
	pub p_id: i32,
}

impl Ord for Elem {
	fn cmp(&self, other: &Elem) -> Ordering {
		// other.glob_heuristic.cmp(&self.glob_heuristic)
		other.total_cost.cmp(&self.total_cost)
		.then_with(|| other.glob_heuristic.cmp(&self.glob_heuristic))
	}
}

impl PartialOrd for Elem {
	fn partial_cmp(&self, other: &Elem) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub struct Puzzle {
	pub len: usize,
	pub numbers: Vec<Number>,

	pub final_list: Vec<Elem>,
	pub open_l: BinaryHeap<Elem>,
	pub close_l: HashMap<String, Elem>,

	pub max_steps: i32,
}

impl Puzzle
{
	pub fn init_pos(&mut self) -> ()
	{
		// let square_len: f64 = gameview::get_square_len(&self, [0.0; 2], 880.0);
		// let start = [square_len / 2.0, square_len / 2.0];
		// let mut y: f64 = start[1];
		// let mut x: f64;
		let mut l: usize = 0;
		let mut base_x: i32 = 0;
		let mut base_y: i32 = 0;
		let puzzle_len = self.get_len();

		for element in self.get_numbers().iter_mut() {
			if l == puzzle_len {
				// x = square_len / 2.0;
				// y += square_len;
				l = 0;
				base_y += 1;
				base_x = 0;
			}
			//  else {
			// 	x = start[0] + l as f64 / puzzle_len as f64 * 880.0;
			// }
			// if element.value != 0 {
			// 	element.update_pos(x, y);
			// } else {
				// element.update_pos(x - 20.0, y - 20.0);
			// }
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
			total_cost: 0,
			id: 0,
			p_id: 0,
		};

		// self.get_manhattan_heuristic(&finalboard, &mut elem);
		self.open_l.push(elem);

		self.a_star(&finalboard);
		// println!("close_list {:?}", close_list);
		// println!("open_list {:?}", self.open_list);
	}

	fn in_close_list(&self, board: &Vec<Number>) ->(bool)
	{
		let mut key = String::new();

		for elem in board.iter() {
			key += &elem.value.to_string();
		}
		self.close_l.contains_key(&key)
	}

	fn move_elem(&mut self, finalboard: &Vec<Number>, board: &Vec<Number>, z:usize, o:usize, s: i32, id: i32, h: i32)
	{
		let mut newboard: Vec<Number> = board.to_vec();
		let tmp: i32 = board[z].value;

		let heuristic_o = ((newboard[z].x_base - finalboard[newboard[o].value as usize - 1].x_base).abs()
		 + (newboard[z].y_base - finalboard[newboard[o].value as usize - 1].y_base).abs()) - newboard[o].h;

		newboard[z].value = newboard[o].value;
		newboard[o].value = tmp;
		if !self.in_close_list(&newboard) {
			let elem: Elem = Elem {
				list: newboard.to_vec(),
				glob_heuristic: h + heuristic_o,
				step: s,
				total_cost: 0,
				id: 0,
				p_id: id,
			};
			self.open_l.push(elem);
		}
	}

	// get zero pos previous this fn
	fn find_move(&mut self, finalboard: &Vec<Number>, board: &Vec<Number>, step: i32, id: i32, h: i32)
	{
		let len = self.len;
		let board_size = len * len;

		for (i, elem) in board.iter().enumerate()
		{
			if elem.value == 0 {
				// Up
				if (i as i32 - len as i32) >= 0 {
					self.move_elem(&finalboard, &board, i, i - len, step, id, h)
				}
				// Down
				if i + len < board_size{
					self.move_elem(&finalboard, &board, i, i + len, step, id, h)
				}
				// Left
				if (i as i32 - 1) > 0 && i / len == (i - 1) / len {
					self.move_elem(&finalboard, &board, i, i - 1, step, id, h)
				}
				// Right
				if i / len == (i + 1) / len && i < board_size {
					self.move_elem(&finalboard, &board, i, i + 1, step, id, h)
				}
				break;
			}
		}
	}

	fn insert_in_final(&self, elem: &Elem, tmp_vec: &mut Vec<Elem>, id: &mut i32, elem_id: &mut i32) -> (bool)
	{
		if elem.id == *elem_id {
			let board: Elem = Elem {
				list: elem.list.to_vec(),
				glob_heuristic: elem.glob_heuristic,
				step: elem.step,
				total_cost: elem.total_cost,
				id: elem.id,
				p_id: elem.p_id,
			};
			*id = elem.id;
			*elem_id = elem.p_id;
			tmp_vec.push(board);
			return true;
		} else {
			return false;
		}
	}

	fn get_final_path(&mut self, last_id: &i32)
	{
		let mut id :i32 = *last_id;
		let mut pa_id :i32 = id;
		let mut tmp_vec = Vec::new();

		loop {
			self.close_l.iter().position(|c_id| self.insert_in_final(&c_id.1, &mut tmp_vec, &mut id, &mut pa_id));
			// println!("debug id {} , p_id {}", id, pa_id);
			// println!("418 is in list {:?}", self.close_l.contains_key("1403211245101386119157"));
			// println!("417 is in list {:?}", self.close_l.contains_key("1430211245101386119157"));


			// if pa_id == 417 {
			// 	println!(" elem {:?}", self.close_l.get("1430211245101386119157"));
			// // 	for val in self.close_l.values() {
			// // 			println!("id {} p_id {}", val.id, val.p_id);
			// // 	}
			// 	break;
			// }

			if id == 0{
				break;
			}
		}

		// self.final_list = tmp_vec;
		// // println!("step {}", self.final_list[0].step);
		// // println!("heuristic {}", heuristic);
		// self.final_list.reverse();
		// let mut y: i32;
		// for obj in self.final_list.iter() {
		// 	y = 0;
		// 	for elem in obj.list.iter(){
		// 		if y != elem.y_base {
		// 			y += 1;
		// 			println!("");
		// 		}
		// 		print!("{} ", elem.value);
		// 	}
		// 	println!("\n");
		// }
	}

	fn a_star(&mut self, finalboard: &Vec<Number>)
	{
		let mut id : i32 = 0;
		loop {
			println!("step {}", id);
			let mut board_study: Elem;

			match self.open_l.pop() {
				Some(elem) => board_study = elem,
				None => {break},
			}

			board_study.id = id;
			let step = board_study.step + 1;
			self.get_manhattan_heuristic(&finalboard, &mut board_study);

			// let mut y = 0;
			// for elem in board_study.list.iter(){
			// 	if y != elem.y_base {
			// 		y += 1;
			// 		println!("");
			// 	}
			// 	print!("{} ", elem.value);
			// }
			// println!("");

			println!("id {} p_id {}", board_study.id, board_study.p_id);

			let mut key = String::new();
			for elem in board_study.list.iter() {
				key += &elem.value.to_string();
			}

			if board_study.glob_heuristic == 0 {
				// println!("fin");
				self.close_l.insert(key, board_study);
				self.get_final_path(&id);
				break;
			}

			// if board_study.id == 417 {
			// 	println!("key {}", key);
			// 	// 1403211245101386119157
			// 	// 1430211245101386119157
			// 	break;
			// }
			self.find_move(&finalboard, &board_study.list, step, id, board_study.glob_heuristic);
			self.close_l.insert(key, board_study);
			id += 1;
		}
		// println!("Maximum number of states ever represented in memory {}", id);
		// println!("len of open_list {:?}", self.open_list.len());
		// println!("len of close_list {:?}", self.close_list.len());
	}

	fn get_manhattan_heuristic(&mut self, finalboard: &Vec<Number>, elem: &mut Elem)
	{
		let mut global_h = 0;

		// return abs(a.x - b.x) + abs(a.y - b.y)
		for e in elem.list.iter_mut()
		{
			if e.value != 0
			{
				e.h = (e.x_base - finalboard[e.value as usize - 1].x_base).abs() + (e.y_base - finalboard[e.value as usize - 1].y_base).abs();
				global_h += e.h;
			}
			// println!(" value {}, posx {}, posy {}, heuristic {}", e.value, e.x_base, e.y_base, e.h);
		}

		if self.max_steps == 0 {
			self.max_steps = global_h;
		}
		// println!("G_heuristic {} step {}", global_h, elem.step);
		elem.glob_heuristic = global_h;
		elem.total_cost = (global_h + elem.step - self.max_steps).abs() ;
		// println!("G_heuristic {} step {} total {}", elem.glob_heuristic , elem.step, elem.total_cost);
	}

	//find better solution
	fn get_last_pos(&self, size: i32) -> (Vec<Number>)
	{
		let mut board: Vec<Number> = Vec::new();
		let last_elem = (size * size) + 1;
		let mut max_x = size - 1;
		let mut min_y = 0;
		let mut c_x = 0;
		let mut c_y = 0;
		let mut r = true;

		for x in 1..last_elem
		{
			let elem = if x != last_elem {
				// Number {value: x, x: c_x, y: c_y, h: 0, x_base: 0, y_base: 0}
				Number {value: x, x_base: c_x, y_base: c_y, h: 0}
			} else {
				Number {value: 0, x_base: c_x, y_base: c_y, h: 0}
				// Number {value: 0, x: c_x, y: c_y, h: 0, x_base: 0, y_base: 0}
			};

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
			board.push(elem);
		}
		return board;
	}
}
