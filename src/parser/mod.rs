// **************************************************************************** //
//                                                                              //
//                                                         :::      ::::::::    //
//    parser.rs                                          :+:      :+:    :+:    //
//                                                     +:+ +:+         +:+      //
//    By: frmarinh <frmarinh@student.42.fr>          +#+  +:+       +#+         //
//                                                 +#+#+#+#+#+   +#+            //
//    Created: 2017/10/25 11:09:53 by frmarinh          #+#    #+#              //
//    Updated: 2017/10/25 11:09:56 by frmarinh         ###   ########.fr        //
//                                                                              //
// **************************************************************************** //

use std::fs::File;
use std::io::prelude::*;

use super::puzzle;
pub struct Parser {
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

	pub fn parse_puzzle(&self) -> (puzzle::Puzzle) {
		puzzle::Puzzle { len: 123, }
	}
}
