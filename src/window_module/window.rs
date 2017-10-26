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
	let mut rotation: f64 = 0.0;
	//window.set_lazy(true);

	while let Some(e) = window.next()
	{
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);

			let text = text::Text::new_color([0.0, 0.7, 0.7, 1.0], 32);
			text.draw("12", &mut glyphs, &c.draw_state, c.transform.trans(20.0, 65.0), g);

			gameview::draw_2d(&puzzle, &c, g, &mut glyphs);
		});
	}
}
