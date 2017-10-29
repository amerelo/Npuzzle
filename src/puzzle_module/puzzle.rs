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

impl Puzzle
{

	pub fn get_len(&self) -> (usize) {
		self.len
	}

	pub fn get_numbers(&self) -> (&Vec<i32>) {
		&self.numbers
	}

	pub fn solve_puzzle(&self)
	{
		let finalboard: Vec<Position> = self.get_last_pos(self.len as i32);

		self.get_heuristic(finalboard);
		// println!("get final pos of 8 {:?}", finalboard[8 - 1]);
		// println!("get final pos of 1 {:?}", finalboard[1 - 1]);
		// println!("get final pos of 15 {:?}", finalboard[15 - 1]);

		// println!("|vec {:?}", finalboard);
	}

	// fn a_star(&self, finalboard: Vec<Position>)
	// {
	//
	// }

	fn get_heuristic(&self, finalboard: Vec<Position>)
	{
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
	}

	//find better solution
	fn get_last_pos(&self, size: i32) -> (Vec<Position>)
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
			board.push(elem);
		}
		board
		// println!("|vec {:?}", board);
		// println!("size {} vec size {}", size, board.len());
	}
}
