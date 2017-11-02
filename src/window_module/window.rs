extern crate find_folder;

use piston_window::*;
use puzzle_module::puzzle::Puzzle;
use gameview_module::gameview;
use puzzle_module::puzzle::Number;
use puzzle_module::puzzle::Elem;
use std::{thread, time};

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
	let ten_millis = time::Duration::from_millis(200);
	let now = time::Instant::now();

	let mut elems: &Vec<Elem> = &puzzle.final_list;
	let mut last_value: Vec<Number> = vec![];
	let mut iterator = elems.iter();
	let mut last_step: i32 = 0;
	while let Some(e) = window.next()
	{
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			gameview::grid_gen(&puzzle, &c, g);

			match iterator.next() {
				Some(current_actions) => {
					let numbers = &current_actions.list;
					let display_step = format!("Solving step #{}", current_actions.step.to_string());
					text::Text::new_color([0.1, 0.4, 0.8, 1.0], 40).draw(&display_step, &mut glyphs, &c.draw_state, c.transform.trans(300.0, 930.0), g);

					if last_value.len() > 0 {
						for (_i, &ref number) in numbers.iter().enumerate() {
							let value = number.value;
							let mut display = if value != 0 { value.to_string() } else { String::from("X") } ;
								if display != "X" {
									match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
									{
										Ok(text) => { }
										Err(err) => { println!("Error while trying to print a number {}", err); break; }
									}
								} else {
									match text::Text::new_color([1.0, 0.5, 0.5, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
									{
										Ok(text) => { }
										Err(err) => { println!("Error while trying to print a number {}", err); break; }
									}
								}
							}
					} else {
						for (_i, &ref number) in numbers.iter().enumerate() {
							let mut display = if number.value != 0 { number.value.to_string() } else { String::from("X") } ;
							if display != "X" {
								match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
								{
									Ok(text) => { }
									Err(err) => { println!("Error while trying to print a number {}", err); break; }
								}
							} else {
								match text::Text::new_color([1.0, 0.5, 0.5, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
								{
									Ok(text) => { }
									Err(err) => { println!("Error while trying to print a number {}", err); break; }
								}
							}
						}
					}
					last_value = numbers.to_vec();
					last_step = current_actions.step;
				}

				None => {
					for (_i, &ref number) in last_value.iter().enumerate() {
						let mut display = if number.value != 0 { number.value.to_string() } else { String::from("X") } ;
						if display != "X" {
							match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
							{
								Ok(text) => { }
								Err(err) => { println!("Error while trying to print a number {}", err); break; }
							}
						} else {
							match text::Text::new_color([1.0, 0.5, 0.5, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x, number.y), g)
							{
								Ok(text) => { }
								Err(err) => { println!("Error while trying to print a number {}", err); break; }
							}
						}
					}
					let display_step = format!("Puzzle solved at step #{}", last_step.to_string());
					text::Text::new_color([0.7, 0.4, 0.2, 1.0], 40).draw(&display_step, &mut glyphs, &c.draw_state, c.transform.trans(260.0, 930.0), g);
				}
			}
			thread::sleep(ten_millis);
		});
	}
}
