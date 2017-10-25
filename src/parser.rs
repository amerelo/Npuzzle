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

pub struct Parser {
	file: String,
	validity: bool,
}

impl Parser {
	pub fn parse_file(name: &String) -> (Parser) {
		let mut f = File::open(name).expect("file not found");
		Parser { file: name.clone(), validity: true, }
	}

	pub fn is_valid(&self) -> (bool) {
		self.validity
	}

	pub fn get_file(&self) -> (&String) {
		&self.file
	}
}
