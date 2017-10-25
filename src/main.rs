mod parser_module;
mod puzzle_module;

use std::env;
use parser_module::parser::Parser;

fn main()
{
	let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		let file_path = &args[1];
		if file_path.len() > 0 {
			let parser: Parser = Parser::new_file(file_path);
			if parser.is_file_valid() {
				let puzzle = parser.parse_puzzle();
				if puzzle.get_len() > 0 {
					println!("Puzzle len: {}", puzzle.get_len())
				} else {
					println!("Invalid puzzle !");
				}
				//println!("{}", parser.get_file_content());
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
