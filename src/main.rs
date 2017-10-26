mod parser_module;
mod puzzle_module;
mod window_module;
mod gameview_module;

extern crate piston_window;
extern crate opengl_graphics;

use std::env;
use parser_module::parser::Parser;
use window_module::window;
use gameview_module::gameview;

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() == 2
	{
		let file_path = &args[1];
		if file_path.len() > 0
		{
			let parser: Parser = Parser::new_file(file_path);

			if parser.is_file_valid()
			{
				let puzzle = parser.parse_puzzle();
				if puzzle.get_len() > 0
				{
					window::create_window(&puzzle, [885; 2]);
				} else {
					println!("Invalid puzzle !");
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
