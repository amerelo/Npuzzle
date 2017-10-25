/*extern crate graphics;*/
extern crate find_folder;

use piston_window::*;

pub fn make_win()
{
	let mut window: PistonWindow = WindowSettings::new (
		"Npuzzle",
		[900, 900]
	)
	.exit_on_esc(true)
	.build()
	.unwrap();

	let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	let ref font = assets.join("FFF_Tusj.ttf");
	let factory = window.factory.clone();
	let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

	let mut rotation: f64 = 0.0;
	// window.set_lazy(true);
	while let Some(e) = window.next()
	{
		rotation += 0.01;

		window.draw_2d(&e, |c, g| {
			let transform = c.transform.trans(10.0, 100.0);
			clear([0.0, 0.0, 0.0, 1.0], g);

			text::Text::new_color([0.0, 0.7, 0.7, 1.0], 32).draw(
				"Coucou alexis !",
				&mut glyphs,
				&c.draw_state,
				transform, g
			).unwrap();

			let mut center = c.transform.trans(300.0, 300.0);
			let mut square = rectangle::square(0.0, 0.0, 50.0);
			let mut red = [1.0, 0.0, 0.0, 1.0];
			rectangle(red, square, center.rot_rad(rotation).trans(-25.0, -25.0), g);

			center = c.transform.trans(450.0, 450.0);
			square = rectangle::square(0.0, 0.0, 50.0);
			red = [0.0, 0.5, 0.5, 1.0];
			rectangle(red, square, center.rot_rad(rotation).trans(-25.0, -25.0), g);
			/*center.rot_rad(rotation).trans(-25.0, -25.0)*/
			// We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered
		});
	}
}
