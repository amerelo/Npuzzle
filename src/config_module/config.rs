use std::collections::HashMap;

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

pub struct Config
{
	pub argv: Vec<String>,
	pub flags: HashMap<String, String>,
	pub options: Vec<char>,
}

impl Config
{
	pub fn parse_args(&mut self) -> ()
	{
		for item in self.argv.iter() {
			if item.substring(0, 2) == "--" {
				let datas = item.substring(2, item.len());
				let split: Vec<&str> = datas.split('=').collect();
				if split.len() == 2 {
					self.flags.insert(split[0].to_string(), split[1].to_string());
				} else if split.len() == 1 {
					self.flags.insert(split[0].to_string(), String::from(""));
				}
			}
		}
	}

	pub fn get_flag(&self, flag: &'static str) -> (Option<String>)
	{
		for (key, val) in &self.flags {
			if key == flag {
				return Some(val.clone());
			}
		}
		None
	}

	pub fn parse_options(&mut self) -> ()
	{
		match self.get_flag("options")
		{
			Some(value) => {
				let mut options = value.chars();
				for _i in 0..value.len() {
					match options.next()
					{
						Some(option) => {
							self.options.push(option);
						}
						None => { }
					}
				}
			},
			None => { }
		}
	}

	pub fn has_option(&self, asked_option: char) -> (bool)
	{
		for option in self.options.iter() {
			if option == &asked_option {
				return true;
			}
		}
		return false;
	}

	pub fn new(args: Vec<String>) -> (Config)
	{
		let mut object = Config { argv: args, flags: HashMap::new(), options: vec![] };
		object.parse_args();
		object.parse_options();
		return object;
	}
}
