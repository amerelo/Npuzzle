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
		// .then_with(|| other.step.cmp(&self.step))
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
		let mut elem: Elem = Elem {
			list: self.numbers.to_vec(),
			glob_heuristic: 0,
			step: 0,
			total_cost: 0,
			id: 0,
			p_id: 0,
		};

		self.get_manhattan_heuristic(&finalboard, &mut elem);
		self.open_l.push(elem);

		self.a_star(&finalboard);
	}

	fn a_star(&mut self, finalboard: &Vec<Number>)
	{
		let mut id : i32 = 0;
		// for x in 0..10 {
		loop {
			println!("\nstep {}", id);
			let mut board_study: Elem;

			match self.open_l.pop() {
				Some(elem) => {board_study = elem},
				None => {return},
			}
			if !self.in_close_list(&board_study.list) {
				board_study.id = id;
				let step = board_study.step + 1;
				// println!("h {}", board_study.glob_heuristic);
				// println!("steps {} \n", step);
				// self.get_manhattan_heuristic(&finalboard, &mut board_study);

				let mut y = 0;
				for elem in board_study.list.iter(){
					if y != elem.y_base {
						y += 1;
						println!("");
					}
					print!("{} ", elem.value);
				}
				println!("");

				let mut key = String::new();
				for elem in board_study.list.iter() {
					key += &elem.value.to_string();
				}

				if board_study.glob_heuristic <= 0 {
					self.close_l.insert(key, board_study);
					self.get_final_path(&id);
					return;
				}
				self.find_move(&finalboard, &mut board_study, step);
				self.close_l.insert(key, board_study);
			}
			id += 1;
		}
		// println!("Maximum number of states ever represented in memory {}", id);
		// println!("len of open_list {:?}", self.open_list.len());
		// println!("len of close_list {:?}", self.close_list.len());
	}

	fn in_close_list(&self, board: &Vec<Number>) ->(bool)
	{
		let mut key = String::new();

		for elem in board.iter() {
			key += &elem.value.to_string();
		}
		self.close_l.contains_key(&key)
	}

	fn move_elem(&mut self, finalboard: &Vec<Number>, board: &Elem, z:usize, o:usize, s: i32)
	{
		let mut newboard: Vec<Number> = board.list.to_vec();
		// let heuristic_o = ((newboard[z].x_base - finalboard[newboard[o].value as usize - 1].x_base).abs()
		// + (newboard[z].y_base - finalboard[newboard[o].value as usize - 1].y_base).abs()) - newboard[o].h;

		// println!("zero {} , other {}", newboard[z].value, newboard[o].value);
		newboard[z].value = newboard[o].value;
		newboard[o].value = 0;
		// newboard[z].h = newboard[o].h;
		// newboard[o].h = 0;
		if !self.in_close_list(&newboard) {
			let mut elem: Elem = Elem {
				list: newboard,
				glob_heuristic: 0,
				step: s,
				total_cost: 0,
				id: 0,
				p_id: board.id,
			};
			self.get_manhattan_heuristic(&finalboard, &mut elem);
			self.open_l.push(elem);
		}
	}

	fn find_move(&mut self, finalboard: &Vec<Number>, board: &Elem, step: i32)
	{
		let len = self.len;
		let board_size = len * len;
		let tmp = board.list.to_vec();

		for (i, elem) in tmp.iter().enumerate()
		{
			if elem.value == 0 {
				// Up
				if (i as i32 - len as i32) >= 0 {
					self.move_elem(&finalboard, &board, i, i - len, step)
				}
				// Down
				if i + len < board_size{
					self.move_elem(&finalboard, &board, i, i + len, step)
				}
				// Left
				if (i as i32 - 1) > 0 && i / len == (i - 1) / len {
					self.move_elem(&finalboard, &board, i, i - 1, step)
				}
				// Right
				if i / len == (i + 1) / len && i < board_size {
					self.move_elem(&finalboard, &board, i, i + 1, step)
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
		} else { return false; }
	}

	fn get_final_path(&mut self, last_id: &i32)
	{
		let mut id :i32 = *last_id;
		let mut pa_id :i32 = id;
		let mut tmp_vec = Vec::new();

		while id != 0 {
			self.close_l.iter().position(|c_id| self.insert_in_final(&c_id.1, &mut tmp_vec, &mut id, &mut pa_id));
		}
		self.final_list = tmp_vec;
		self.final_list.reverse();
		// let mut y: i32;
		// for obj in self.final_list.iter() {
		// 	y = 0;
		// 	for elem in obj.list.iter(){
		// 	// for elem in self.final_list[0].list.iter(){
		// 		if y != elem.y_base {
		// 			y += 1;
		// 			println!("");
		// 		}
		// 		print!("{} ", elem.value);
		// 	}
		// 	println!("\n");
		// }
		println!("steps {}", self.final_list.len() - 1);
		// println!("heuristic {}", heuristic);
	}

	fn get_linear_conflict(&mut self, finalboard: &Vec<Number>, val: &Number, obj: &Number, tab: &Vec<Number>, index: usize) ->(i32)
	{
		let mut lc: i32 = 0;
		let mut it: i32 = index as i32;
		let sens: i32 =  if (val.x_base - obj.x_base ) > 0 {-1} else{1};

		loop {
			if it < 0 || it >= tab.len() as i32 || tab[it as usize].x_base == obj.x_base { break;}
			it += sens;
			if it != 0 && tab[it as usize].value != 0 && finalboard[(tab[it as usize].value - 1) as usize].y_base == val.y_base {
				lc += 1;
			}
		}
		return lc;
	}

	fn get_manhattan_heuristic(&mut self, finalboard: &Vec<Number>, elem: &mut Elem)
	{
		let mut lc;
		let mut dif_x;
		let mut dif_y;
		let mut global_h = 0;

		// println!("Manhattan");
		for (i, e) in elem.list.iter().enumerate()
		{
			lc = 0;
			if e.value != 0
			{
				dif_x = (e.x_base - finalboard[e.value as usize - 1].x_base).abs();
				dif_y = (e.y_base - finalboard[e.value as usize - 1].y_base).abs();

				// Tiles out of row and column / Hamming
				// put flag condition
				// if dif_x != 0 || dif_y != 0 {
				// 	if dif_x != 0 {
				// 		lc += 1;
				// 	}
				// 	if dif_y != 0 {
				// 		lc += 1;
				// 	}
				// 	lc += 1;
				// }
				// Linear Conflict
				if e.y_base == finalboard[e.value as usize - 1].y_base{
					lc += self.get_linear_conflict(&finalboard , &e, &finalboard[e.value as usize - 1], &elem.list, i);
					// println!("lc {} for value {}", lc, e.value);
				}

				// Manhattan
				lc += dif_x + dif_y;
				global_h += lc;
				// if e.h != 0 {
				// 	println!("value of conflict {} , h {}", e.value, e.h);
				// }
			}
		}

		elem.glob_heuristic = global_h;
		elem.total_cost = global_h + (elem.step * 0);
		// println!("G_heuristic {} step {} total {}", elem.glob_heuristic , elem.step, elem.total_cost);
	}

	fn get_last_pos(&self, size: i32) -> (Vec<Number>)
	{
		let mut board: Vec<Number> = Vec::new();
		let last_elem = size * size;
		let mut max_x = size - 1;
		let mut min_y = 0;
		let mut c_x = 0;
		let mut c_y = 0;
		let mut r = true;

		for x in 1..(last_elem + 1)
		{
			let elem = if x != last_elem {
				Number {value: x, x_base: c_x, y_base: c_y, h: 0}
			} else {
				Number {value: 0, x_base: c_x, y_base: c_y, h: 0}
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
				if c_x > min_y {
					c_x -= 1;
				} else if c_y > min_y {
					c_y -= 1;
					if c_x == min_y && c_y == min_y + 1 {
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
