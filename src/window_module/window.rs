extern crate find_folder;

use piston_window::*;
use gameview;
use puzzle_module::puzzle::Puzzle;

pub fn create_window(puzzle: &Puzzle, window_size: [u32; 2])
{
	let mut window: PistonWindow = WindowSettings::new ("Npuzzle", window_size)
	.exit_on_esc(true)
	.build()
	.unwrap();

	let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	let ref font = assets.join("FiraSans-Regular.ttf");
	let factory = window.factory.clone();
	let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
	//let mut rotation: f64 = 0.0;
	window.set_lazy(true);

	while let Some(e) = window.next()
	{
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			let square_len:f64 = gameview::grid_gen(&puzzle, &c, g, &mut glyphs);

			let start = [square_len / 2.0, square_len / 2.0];
			let mut y: f64 = start[1];
			let mut x: f64 = 0.0;
			let mut l: usize = 0;
			for (i, &item) in puzzle.get_numbers().iter().enumerate()
			{
				if l == puzzle.get_len() {
					x = square_len / 2.0;
					y += square_len;
					l = 0;
				} else {
					x = start[0] + l as f64 / puzzle.get_len() as f64 * 880.0;
				}
				if item != 0 {
					let text = text::Text::new_color([0.0, 0.7, 0.7, 1.0], 16);
					text.draw(&item.to_string(), &mut glyphs, &c.draw_state, c.transform.trans(x, y), g);
				}
				l += 1;
			}
		});
	}
}
