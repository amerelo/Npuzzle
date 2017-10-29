extern crate find_folder;

use piston_window::*;
use puzzle_module::puzzle::Puzzle;
use gameview_module::gameview;

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
	window.set_lazy(true);

	println!("{:?}", puzzle.get_numbers());
	while let Some(e) = window.next()
	{
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			gameview::grid_gen(&puzzle, &c, g);
			for (_i, &ref item) in puzzle.get_numbers().iter().enumerate()
			{
				if item.value != 0 {
					match text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16).draw(&item.value.to_string(), &mut glyphs, &c.draw_state, c.transform.trans(item.x, item.y), g)
					{
						Ok(text) => { }
						Err(err) => { println!("Error while trying to print a number {}", err); break; }
					}
				}
			}
		});
	}
}
