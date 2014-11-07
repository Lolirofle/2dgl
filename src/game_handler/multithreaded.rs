use core;
use gl;
use time;
use rustrt;

use game::gameloop::{Updatable,Renderable,EventHandler};
use game::Game;
use game_handler::GameHandler as GameHandlerTrait;
use game_handler::Instance as GameHandlerInstance;
use graphics::renderer::Renderer;
use rustrt::thread::Thread;
use std::io::timer;
use std::time::Duration;
use std::mem;

enum RenderingMessage<T : Send>{
	RenderStop,
	RenderData(T),
}

pub struct GameHandler<G,R,E,RenderData>
	where G: Game<E> + Send + Clone,
	      R: Renderer + Send;

impl<G,R,E,RenderData> GameHandlerTrait<G,R,E,RenderData> for GameHandler<G,R,E,RenderData>
	where G: Game<E> + Send + Clone,
	      R: Renderer + Send,
{
	fn run(&self,mut renderer: R,instance: &GameHandlerInstance<G,E,RenderData>,game: &mut G){
		//Render
		let mut render_game = game.clone();

		let (render_send,render_receive) = channel::<RenderingMessage<G>>();
		//let instance2 = unsafe{transmute(instance)};
		let render_thread = Thread::start(proc(){
			//instance.init_render();

			loop{
				match render_receive.try_recv(){
					Ok(message) => match message{
						RenderData(new_data) => {render_game = new_data;},

						//Check if the render loop should stop.
						RenderStop => break,
					},
					_ => {}
				}

				render_game.render(&renderer);
			}
		});

		//Event and Update
		let mut previous_time = time::get_time();
		let mut next_time     = previous_time;
		loop{
			//Fetch events, processing each
			/*for e in *instance.event(){
				game.event(e);
			}*/

			//Check if the window should close
			if game.should_exit(){
				break;
			}

			//Update
			game.update((),time::get_time() - previous_time);
			render_send.send(RenderData(game.clone()));

			//Timing
			let current_time = time::get_time();
			next_time = next_time + game.target_time_per_frame();
			timer::sleep(next_time - current_time);
			previous_time = current_time;
		}

		//Tell the tasks to stop.
		render_send.send(RenderStop);

		//Wait for tasks to stop (and therefore the lifetimes of all the borrowed refs are valid. But rustc doesn't think so?)
		render_thread.join();
	}
}
