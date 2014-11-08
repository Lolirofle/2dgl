use time;

use game::gameloop::{Update,Render};
use game::Game;
use game_handler::GameHandler as GameHandlerTrait;
use graphics::renderer::Renderer;
use std::io::timer;

pub struct GameHandler<G,R,RenderData>
	where G: Game<(),RenderData>,
	      R: Renderer;

impl<G,R,RenderData> GameHandlerTrait<G,R,RenderData> for GameHandler<G,R,RenderData>
	where G: Game<(),RenderData>,
	      R: Renderer
{
	fn run(&self,renderer: R,game: &mut G){
		//Init
		let mut previous_time = time::get_time();
		let mut next_time     = previous_time;

		//Render
		let mut render_data = game.init_render();

		loop{
			//Process events
			game.event(());

			//Check if the window should close
			if game.should_exit(){
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
	}
}
