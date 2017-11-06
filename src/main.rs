mod parser_module;
mod puzzle_module;
mod window_module;
mod gameview_module;
mod checker_module;
mod config_module;

extern crate piston_window;
extern crate opengl_graphics;
extern crate gfx_device_gl;

use std::env;
use parser_module::parser::Parser;
use window_module::window;
use checker_module::checker;
use puzzle_module::puzzle::Puzzle;
use std::process;

fn check_print_help(puzzle: &Puzzle)
{
	match puzzle.config.get_flag("help")
	{
		Some(_flag) => {
			println!("Npuzzle [OPTIONS]");
			println!("");
			println!("--help Print this help screen");
			println!("--time Specifies the time to wait for the graphic display");
			println!("--options Options to use on heuristics");
			println!("\t-g (Greedy)");
			println!("\t-h (Hamming)");
			println!("\t-l (Linear Conflict)");
			println!("\t-t (Tiles out of row and column)");
			process::exit(0x0100);
		}
		None => { }
	}
}

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() >= 2
	{
		let file_path = &args[1];
		if file_path.len() > 0
		{
			let parser: Parser = Parser::new_file(file_path);

			if parser.is_file_valid()
			{
				let mut puzzle = parser.parse_puzzle();
				check_print_help(&puzzle);
				if checker::solvable(&puzzle) {
					if puzzle.get_len() >= 3 && puzzle.get_len() <= 20
					{
						puzzle.solve_puzzle();
						window::create_window(puzzle, [885, 950]);
					} else {
						println!("Invalid value or invalid length. (the length must be between 3 - 20)");
					}
				} else {
					println!("The puzzle given as argument is not solvable !");
				}
			} else {
				println!("The file {} is not valid.", parser.get_file());
			}
		} else {
			println!("Invalid file name.");
		}
	} else {
		println!("You must set a file as argument containing a puzzle to solve.");
	}
}
