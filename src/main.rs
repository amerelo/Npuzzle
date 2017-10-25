mod parser_module;
mod puzzle_module;

use std::env;
use parser_module::parser::Parser;

extern crate piston_window;
extern crate find_folder;
use piston_window::*;
fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() == 2
	{
		let file_path = &args[1];
		if file_path.len() > 0
		{
			let parser: Parser = Parser::new_file(file_path);

			if parser.is_file_valid()
			{
				let puzzle = parser.parse_puzzle();
				if puzzle.get_len() > 0
				{
					let mut window: PistonWindow = WindowSettings::new(
			            "piston: hello_world",
			            [200, 200]
			        )
			        .exit_on_esc(true)
			        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
			        .build()
			        .unwrap();

			    let assets = find_folder::Search::ParentsThenKids(3, 3)
			        .for_folder("assets").unwrap();
			    println!("{:?}", assets);
			    let ref font = assets.join("FiraSans-Regular.ttf");
			    let factory = window.factory.clone();
			    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

			    //window.set_lazy(true);
			    while let Some(e) = window.next() {
			        window.draw_2d(&e, |c, g| {
			            let transform = c.transform.trans(10.0, 100.0);

			            clear([0.0, 0.0, 0.0, 1.0], g);
			            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
			                "Coucou alexis !",
			                &mut glyphs,
			                &c.draw_state,
			                transform, g
			            ).unwrap();
			        });
			    }
					println!("Puzzle len: {}, vector: {:?}", puzzle.get_len(), puzzle.get_numbers());
				} else {
					println!("Invalid puzzle !");
				}
				//println!("{}", parser.get_file_content());
			} else {
				println!("The file {} is not valid.", parser.get_file());
			}
		} else {
			println!("Invalid file name.");
		}
	} else {
		println!("You must set a file as argument containing a puzzle to solve.");
	}
}
