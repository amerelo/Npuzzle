use std::fs::File;
use std::io::prelude::*;

// use puzzle;
use puzzle_module::puzzle::Puzzle;

pub struct Parser
{
	file: String,
	validity: bool,
	content: String,
}

impl Parser {
	pub fn new_file(name: &String) -> (Parser) {
		let f = File::open(name);
		match f
		{
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

	pub fn parse_puzzle(&self) -> (Puzzle)
	{
		let mut puzz_len: usize = 0;
		let mut numbers: Vec<i32> = vec![];
		let mut lines_count: usize = 0;
		let lines = self.content.lines();
		for tuple in lines.enumerate() {
			let line = tuple.1;
			let ch = line.chars().nth(0).unwrap();
			if ch != '#' {
				if puzz_len == 0 && !line.contains(" ") && !line.contains("\t") {
					match line.parse::<usize>() {
						Ok(n) => { puzz_len = n; }
						Err(error) => { println!("Len of puzzle invalid: {}", error); break; }
					}
				} else if puzz_len > 0 {
					let line_numbers: Vec<&str> = line.split_whitespace().collect();
					if line_numbers.len() == puzz_len {
						for (i, &item) in line_numbers.iter().enumerate() {
							match item.parse::<i32>() {
								Ok(n) => numbers.push(n),
								Err(error) => { println!("Number on the line invalid: {}, index: {}", error, i);  }
							}
						}
						lines_count += 1;
					} else {
						println!("{:?} are invalid values for puzzle len {}", line_numbers, puzz_len);
						puzz_len = 0;
					}
				}
			} else {
				println!("Information: {}", line);
			}
			if lines_count > puzz_len {
				puzz_len = 0;
				break;
			}
		}
		Puzzle { len: puzz_len, numbers: numbers }
	}
}
