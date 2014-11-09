use time;

use game::gameloop::{Update,Render};
use game::Game;
use game_handler::GameHandler as GameHandlerTrait;
use graphics::renderer::Renderer;
use std::io::timer;

pub struct GameHandler<G,R,RenderData,Exit>
	where G: Game<(),RenderData,Exit>,
	      R: Renderer;

impl<G,R,RenderData,Exit> GameHandlerTrait<G,R,RenderData,Exit> for GameHandler<G,R,RenderData,Exit>
	where G: Game<(),RenderData,Exit>,
	      R: Renderer
{
	fn run(&self,renderer: R,game: &mut G) -> Exit{
		//Init
		let mut previous_time = time::get_time();
		let mut next_time     = previous_time;

		//Render
		let mut render_data = game.init_render(&renderer);

		let mut exit_data: Exit;
		loop{
			//Process events
			game.event(());

			//Check if the window should close
			if let Some(d) = game.should_exit(){
				exit_data = d;
				break;
			}

			//Update
			game.update((),time::get_time() - previous_time);

			//Timing
			let current_time = time::get_time();
			next_time = next_time + game.target_time_per_frame();
			timer::sleep(next_time - current_time);
			previous_time = current_time;

			//Render
			game.render(&renderer,&mut render_data);
		}

		return exit_data;
	}
}
