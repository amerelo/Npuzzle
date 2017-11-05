use puzzle_module::puzzle::Puzzle;

fn format_board(size: i32, board: &Puzzle) ->(Vec<i32>)
{
	let mut tmp: Vec<i32> = Vec::new();
	let last_elem = size * size;
	let mut pos: i32 = 0;
	let mut sense: i32 = 0;
	let mut move_s: i32 = 1;
	let mut max = size - 1;
	let mut fix = 0;

	for _x in 0..last_elem
	{
		tmp.push(board.numbers[pos as usize].value);
		if fix % size == max {
			if sense == 0 {
				move_s = size;
				max -= 1;
				fix = -1;
				sense = 1;
			} else if sense == 1 {
				move_s = -1;
				fix = -1;
				sense = 2;
			} else if sense == 2 {
				move_s = -size;
				max -= 1;
				fix = -1;
				sense = 3;
			} else {
				move_s = 1;
				fix = -1;
				sense = 0;
			}
		}
		fix += 1;
		pos += move_s;
	}
	return tmp;
}

fn is_valid(tmp: Vec<i32>) ->(bool)
{
	let mut inv_count: i32 = 0;

	for w in 0..tmp.len()
	{
		for z in (w + 1)..tmp.len()
		{
			if tmp[z] != 0 && tmp[w] > tmp[z] {
				inv_count += 1;
			}
		}
	}
	return inv_count % 2 == 0;
}

pub fn solvable(board: &Puzzle) ->(bool)
{
	return is_valid(format_board(board.len as i32, board));
}
