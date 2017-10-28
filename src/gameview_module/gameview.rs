use piston_window::*;
use puzzle_module::puzzle::Puzzle;
// use gfx_device_gl::*;

pub fn grid_gen<G: Graphics>(puzzle: &Puzzle, c: &Context, g: &mut G, glyphs: &mut Glyphs) -> (f64)
{
	let draw_size = 880.0;
	let start_position = [0.0; 2];
	let colors_draw = [0.0, 0.5, 0.5, 1.0];

	draw_cell_borders(&puzzle, &c, g, draw_size, start_position, colors_draw);
	draw_section_borders(&puzzle, &c, g, draw_size, start_position, colors_draw);
	Rectangle::new_border(colors_draw, 3.0).draw([start_position[0], start_position[1], draw_size, draw_size], &c.draw_state, c.transform, g);
	return get_square_len(&puzzle, start_position, draw_size);
}

pub fn draw_cell_borders<G: Graphics>(puzzle: &Puzzle, c: &Context, g: &mut G, size: f64, position: [f64; 2], colors: [f32; 4]) -> ()
{
	let cell_edge = Line::new(colors, 1.0);

	for i in 0..9 {
		if (i % 3) == 0 {continue;}

		let x = position[0] + i as f64 / puzzle.get_len() as f64 * size;
		let y = position[1] + i as f64 / puzzle.get_len() as f64 * size;
		let x2 = position[0] + size;
		let y2 = position[1] + size;

		let vline = [x, position[1], x, y2];
		cell_edge.draw(vline, &c.draw_state, c.transform, g);

		let hline = [position[0], y, x2, y];
		cell_edge.draw(hline, &c.draw_state, c.transform, g);
	}
}

pub fn get_square_len(puzzle: &Puzzle, position: [f64; 2], size: f64) -> f64
{
	(position[0] + 1.0 / puzzle.get_len() as f64 * size) as f64
}

pub fn draw_section_borders<G: Graphics>(puzzle: &Puzzle, c: &Context, g: &mut G, size: f64, position: [f64; 2], colors: [f32; 4]) -> ()
{
	let section_edge = Line::new(colors, 2.0);

	for i in 0..puzzle.get_len() {
		let x = position[0] + i as f64 / puzzle.get_len() as f64 * size;
		let y = position[1] + i as f64 / puzzle.get_len() as f64 * size;

		let x2 = position[0] + size;
		let y2 = position[1] + size;

		let vline = [x, position[1], x, y2];
		section_edge.draw(vline, &c.draw_state, c.transform, g);

		let hline = [position[0], y, x2, y];
		section_edge.draw(hline, &c.draw_state, c.transform, g);
	}
}
