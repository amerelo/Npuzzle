use puzzle_module::puzzle::Puzzle;

fn puzzle_3_size(board: &Puzzle) ->(bool)
{
	let mut inv_count: i32 = 0;

	for w in 0..board.numbers.len()
	{
		for z in (w + 1)..board.numbers.len()
		{
			if board.numbers[w].value > board.numbers[z].value {
				inv_count += 1;
			}
		}
	}
	return inv_count % 2 == 0;
}

pub fn not_solvable(board: &Puzzle) ->(bool)
{
	if board.len == 3 {
		return puzzle_3_size(board);
	}
	return false;
}
