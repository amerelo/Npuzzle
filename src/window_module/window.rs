extern crate find_folder;

use piston_window::*;
use puzzle_module::puzzle::Puzzle;
use gameview_module::gameview;
use puzzle_module::puzzle::Number;
use std::{thread, time};

pub fn get_vecs(puzzle: &Puzzle) -> (Vec<Vec<Number>>)
{
	let puzzle_nbr = &puzzle.numbers;
	let mut test: Vec<Vec<Number>> = vec![];

	let mut copy: Vec<Number> = vec![];
	for (_i, &ref item) in puzzle_nbr.iter().enumerate() {
		copy.push(Number { value: item.value, x: item.x, y: item.y, h: item.h});
	}

	let mut copy2: Vec<Number> = vec![];
	for (i, &ref item) in puzzle_nbr.iter().enumerate() {
		if i == 2 {
			copy2.push(Number { value: item.value + 666, x: item.x, y: item.y, h: item.h});
		} else {
			copy2.push(Number { value: item.value, x: item.x, y: item.y, h: item.h});
		}
	}

	let mut copy3: Vec<Number> = vec![];
	for (i, &ref item) in puzzle_nbr.iter().enumerate() {
		if i == 2 {
			copy3.push(Number { value: item.value + 667, x: item.x, y: item.y, h: item.h});
		} else {
			copy3.push(Number { value: item.value, x: item.x, y: item.y, h: item.h});
		}
	}

	test.push(copy);
	test.push(copy2);
	test.push(copy3);
	return test;
}

pub fn get_updated_number(last_value: &Vec<Number>, current: &Vec<Number>) -> (i32)
{
	let mut iter_last = last_value.iter();
	let mut iter_current = current.iter();

	for i in 0..current.len()
	{
		match iter_last.next()
		{
			Some(last_number) =>  {
				match iter_current.next()
				{
					Some(current_number) =>  {
						if current_number.value > 0 && last_number.value > 0 {
							if current_number.value != last_number.value {
								return current_number.value;
							}
						}
					}
					None => { }
				}
			}
			None => { }
		}

	}
	return -1;
}

pub fn create_window(mut puzzle: Puzzle, window_size: [u32; 2])
{
	let mut window: PistonWindow = WindowSettings::new ("Npuzzle", window_size)
	.exit_on_esc(true)
	.resizable(false)
	.decorated(false)
	.build()
	.unwrap();

	let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	let ref font = assets.join("FiraSans-Regular.ttf");
	let factory = window.factory.clone();
	let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
	// window.set_lazy(true);
	let ten_millis = time::Duration::from_millis(1000);
	let now = time::Instant::now();


	let mut test: Vec<Vec<Number>> = get_vecs(&puzzle);
	let mut iterator = test.iter();
	let mut last_value: &Vec<Number> = &vec![];
	let	mut i: i32 = 1;
	while let Some(e) = window.next()
	{
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			gameview::grid_gen(&puzzle, &c, g);

			match iterator.next() {
				Some(current_actions) => {
					let display_step = format!("Solving step #{}", i.to_string());
					text::Text::new_color([0.1, 0.4, 0.8, 1.0], 40).draw(&display_step, &mut glyphs, &c.draw_state, c.transform.trans(300.0, 930.0), g);
					if last_value.len() > 0 {
						let changed_number: i32 = get_updated_number(last_value, current_actions);
						for (_i, &ref number) in current_actions.iter().enumerate() {
							let value = number.value;
							if value > 0 {
								if value != changed_number {
									match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&number.value.to_string(), &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
									{
										Ok(text) => { }
										Err(err) => { println!("Error while trying to print a number {}", err); break; }
									}
								} else {
									match text::Text::new_color([1.0, 0.5, 0.5, 1.0], 16).draw(&number.value.to_string(), &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
									{
										Ok(text) => { }
										Err(err) => { println!("Error while trying to print a number {}", err); break; }
									}
								}
							}
						}
					} else {
						for (_i, &ref number) in current_actions.iter().enumerate() {
							if number.value > 0 {
								match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&number.value.to_string(), &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
								{
									Ok(text) => { }
									Err(err) => { println!("Error while trying to print a number {}", err); break; }
								}
							}
						}
					}
					last_value = current_actions;
					i += 1;
				}
				None => {
					for (_i, &ref number) in last_value.iter().enumerate() {
						if number.value > 0 {
							match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&number.value.to_string(), &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
							{
								Ok(text) => { }
								Err(err) => { println!("Error while trying to print a number {}", err); break; }
							}
						}
					}
					let display_step = format!("Puzzle solved at step #{}", i.to_string());
					text::Text::new_color([0.7, 0.4, 0.2, 1.0], 40).draw(&display_step, &mut glyphs, &c.draw_state, c.transform.trans(260.0, 930.0), g);
				}
			}
			thread::sleep(ten_millis);
		});

	}
}
