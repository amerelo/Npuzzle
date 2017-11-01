use std::fs::File;
use std::io::prelude::*;

// use puzzle;
use puzzle_module::puzzle::Puzzle;
use puzzle_module::puzzle::Number;

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

	pub fn convert_numbers(&self, numbers: &Vec<i32>) -> (Vec<Number>)
	{
		let mut numbers_pos: Vec<Number> = vec![];
		for (_i, &item) in numbers.iter().enumerate() {
			numbers_pos.push(Number { value: item, x:0.0, y:0.0, h: 0, x_base:0, y_base:0});
		}
		return numbers_pos;
	}

	pub fn parse_puzzle(&self) -> (Puzzle) {
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
								Ok(n) => { if !numbers.iter().any(|&x| x == n) { numbers.push(n) } else {
									println!("The value {} are set multiples times in the puzzle.", n);
									puzz_len = 0;
									break;
								} },
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
		if puzz_len > 0 {
			if numbers.len() != puzz_len * puzz_len || !numbers.iter().any(|&x| x == 0) {
				puzz_len = 0;
			}
		}
		Puzzle { len: puzz_len, numbers: self.convert_numbers(&numbers), open_list: vec![], close_list: vec![], final_list: vec![] }
	}
}
