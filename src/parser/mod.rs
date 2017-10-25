use std::fs::File;
use std::io::prelude::*;
use super::puzzle::Puzzle;

pub struct Parser
{
	file: String,
	validity: bool,
	content: String,
}

impl Parser {
	pub fn new_file(name: &String) -> (Parser) {
		let f = File::open(name);
		match f {
			Ok(mut file) => {
				let mut contents = String::new();
				let res = file.read_to_string(&mut contents);
				match res {
					Ok(read) => {
						if read > 0 {
							return Parser { file: name.clone(), validity: true, content: contents};
						} else {
							return Parser { file: name.clone(), validity: false, content: contents};
						}
					}
					Err(error) => {
						println!("Error while reading file: {}", error);
						return Parser { file: name.clone(), validity: false, content: String::from("")};
					}
				}
			},
			Err(error) => {
				println!("Error while opening file: {}", error);
				return Parser { file: name.clone(), validity: false, content: String::from("")};
			},
		}
	}

	pub fn is_file_valid(&self) -> (bool) {
		self.validity
	}

	pub fn get_file(&self) -> (&String) {
		&self.file
	}

	pub fn get_file_content(&self) -> (&String) {
		&self.content
	}

	pub fn parse_puzzle(&self) -> (Puzzle) {
		let mut puzz_len: i32 = 0;
		let lines = self.content.lines();
		for tuple in lines.enumerate() {
			let line = tuple.1;
			let ch = line.chars().nth(0).unwrap();
			if ch != '#' {
				if puzz_len == 0 && !line.contains(" ") && !line.contains("\t") {
					match line.parse::<i32>() {
						Ok(n) => { puzz_len = n; }
						Err(error) => { println!("Len of puzzle invalid: {}", error); break; }
					}
				} else {
					println!("{}", line);
				}
			} else {
				println!("Information: {}", line);
			}
		}
		Puzzle { len: puzz_len, }
	}
}
