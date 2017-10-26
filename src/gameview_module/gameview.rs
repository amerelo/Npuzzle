use piston_window::*;
use puzzle_module::puzzle::Puzzle;

pub fn draw_2d<G: Graphics>(puzzle: &Puzzle, c: &Context, g: &mut G, glyphs: &mut Glyphs) -> ()
{
	let draw_size = 880.0;
	let start_position = [2.5; 2];
	let colors_draw = [0.0, 0.5, 0.5, 1.0];

	draw_cell_borders(&puzzle, &c, g, draw_size, start_position, colors_draw);
	draw_section_borders(&puzzle, &c, g, draw_size, start_position, colors_draw);
	Rectangle::new_border(colors_draw, 3.0).draw([start_position[0], start_position[1], draw_size, draw_size], &c.draw_state, c.transform, g);
	/*let text_image = Image::new_color([0.5, 0.0, 0.0, 1.0]);
        let cell_size = draw_size / 9.0;
        for j in 0..9 {
            for i in 0..9 {
				let obj = glyphs.opt_character(32, 'b');
				let pos = [
                    start_position[0] + i as f64 * cell_size + 15.0,
                    start_position[1] + j as f64 * cell_size + 34.0
                ];
				/*
				let ch_x = pos[0] + obj.Character.texture.left();
                let ch_y = pos[1] - obj.Character.texture.top();
                text_image.draw(obj.Character.texture,
                                &c.draw_state,
                                c.transform.trans(ch_x, ch_y),
                                g);*/
            }
        }
	*/
}

pub fn draw_cell_borders<G: Graphics>(puzzle: &Puzzle, c: &Context, g: &mut G, size: f64, position: [f64; 2], colors: [f32; 4]) -> ()
{
	let cell_edge = Line::new(colors, 1.0);

	for i in 0..9 {
		if (i % 3) == 0 {continue;}

		let x = position[0] + i as f64 / 9.0 * size;
		let y = position[1] + i as f64 / 9.0 * size;
		let x2 = position[0] + size;
		let y2 = position[1] + size;

		let vline = [x, position[1], x, y2];
		cell_edge.draw(vline, &c.draw_state, c.transform, g);

		let hline = [position[0], y, x2, y];
		cell_edge.draw(hline, &c.draw_state, c.transform, g);
	}
}

pub fn draw_section_borders<G: Graphics>(puzzle: &Puzzle, c: &Context, g: &mut G, size: f64, position: [f64; 2], colors: [f32; 4]) -> ()
{
	let section_edge = Line::new(colors, 2.0);

	for i in 0..3 {
		let x = position[0] + i as f64 / 3.0 * size;
		let y = position[1] + i as f64 / 3.0 * size;
		let x2 = position[0] + size;
		let y2 = position[1] + size;

		let vline = [x, position[1], x, y2];
		section_edge.draw(vline, &c.draw_state, c.transform, g);

		let hline = [position[0], y, x2, y];
		section_edge.draw(hline, &c.draw_state, c.transform, g);
	}
}
