use core;
use gl;
use time;

use game::gameloop::{Updatable,Renderable,EventHandler};
use game::Game;   
use game_handler::GameHandler as GameHandlerTrait;
use game_handler::Instance as GameHandlerInstance;
use graphics::renderer::Renderer;
use std::io::timer;
use std::time::Duration;

pub struct GameHandler<G,R,E,RenderData>
	where G: Game<E>,
	      R: Renderer;

impl<G,R,E,RenderData> GameHandlerTrait<G,R,E,RenderData> for GameHandler<G,R,E,RenderData>
	where G: Game<E>,
	      R: Renderer
{
	fn run(&self,renderer: R,instance: &GameHandlerInstance<G,E,RenderData>,game: &mut G){
		//Init
		let mut previous_time = time::get_time();
		let mut next_time     = previous_time;

		//Render
		instance.init_render();

		loop{
			//Fetch events, processing each
			/*for e in instance.event(){
				game.event(e);
			}*/

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
			game.render(&renderer);
		}
	}
}
