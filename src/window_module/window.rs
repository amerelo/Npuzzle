extern crate find_folder;

use puzzle_module::puzzle::Puzzle;
use gameview_module::gameview;
use puzzle_module::puzzle::Number;
use puzzle_module::puzzle::Elem;
use std::{thread, time};
use piston_window::*;

#[derive(Debug)]
pub struct DisplayNumber
{
	pub x_display: f64,
	pub y_display: f64,
	pub value: i32,
}

pub fn get_step_display(numbers: &Vec<Number>, puzzle: &Puzzle) -> (Vec<DisplayNumber>)
{
	let mut displays: Vec<DisplayNumber> = vec![];
	let base_pos = &puzzle.base_pos;
	let len = puzzle.get_len() * puzzle.get_len();
	let mut number_iterator = numbers.iter();
	let mut pos_iterator = base_pos.iter();

	for _i in 0..len
	{
		match number_iterator.next() {
			Some (number) => {
				match pos_iterator.next()
				{
					Some (pos) => {
						displays.push(DisplayNumber { x_display: pos.x, y_display: pos.y, value: number.value });
					}
					None => { println!("An error occured while tring to iterate"); }
				}
			},
			None => { println!("An error occured while tring to iterate"); }
		}
	}
	return displays;
}

pub fn get_time_to_wait(puzzle: &Puzzle, default_value: u64) -> (u64)
{
	match puzzle.config.get_flag("time")
	{
		Some (value) => {
			match value.parse::<u64>() {
				Ok (number) => {
					if number > 0 {
						return number;
					} else {
						return default_value;
					}
				}
				Err(_error) => {
					return default_value;
				}
			}
		},
		None => {
			return default_value;
		}
	}
}

pub fn create_window(puzzle: Puzzle, window_size: [u32; 2])
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
	let ten_millis = time::Duration::from_millis(get_time_to_wait(&puzzle, 200));

	let elems: &Vec<Elem> = &puzzle.final_list;
	let mut last_value: Vec<DisplayNumber> = vec![];
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
					let display = get_step_display(&numbers, &puzzle);
					let text_step = format!("Solving step #{}", current_actions.step.to_string());
					match text::Text::new_color([0.1, 0.4, 0.8, 1.0], 40).draw(&text_step, &mut glyphs, &c.draw_state, c.transform.trans(300.0, 930.0), g)
					{
						Ok(_text) => { },
						Err(err) => {  println!("Error while trying to print steps {}", err); }
					}
					for (_i, &ref number) in display.iter().enumerate() {
						let value = number.value;
						let display = if value != 0 { value.to_string() } else { String::from("X") } ;
							if display != "X" {
								match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x_display, number.y_display), g)
								{
									Ok(_text) => { }
									Err(err) => { println!("Error while trying to print a number {}", err); break; }
								}
							} else {
								match text::Text::new_color([1.0, 0.5, 0.5, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x_display, number.y_display), g)
								{
									Ok(_text) => { }
									Err(err) => { println!("Error while trying to print a number {}", err); break; }
								}
							}
						}
					last_value = display;
					last_step = current_actions.step;
				},
				None => {
					for (_i, &ref number) in last_value.iter().enumerate() {
						let display = if number.value != 0 { number.value.to_string() } else { String::from("X") } ;
						if display != "X" {
							match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x_display, number.y_display), g)
							{
								Ok(_text) => { }
								Err(err) => { println!("Error while trying to print a number {}", err); break; }
							}
						} else {
							match text::Text::new_color([1.0, 0.5, 0.5, 1.0], 16).draw(&display, &mut glyphs, &c.draw_state, c.transform.trans(number.x_display, number.y_display), g)
							{
								Ok(_text) => { }
								Err(err) => { println!("Error while trying to print a number {}", err); break; }
							}
						}
					}
					let display_step = format!("Puzzle solved at step #{}", last_step.to_string());
					match text::Text::new_color([0.7, 0.4, 0.2, 1.0], 40).draw(&display_step, &mut glyphs, &c.draw_state, c.transform.trans(260.0, 930.0), g)
					{
						Ok(_text) => { }
						Err(err) => { println!("Error while trying to print steps {}", err); }
					}
				}
			}
			thread::sleep(ten_millis);
		});
	}
}
