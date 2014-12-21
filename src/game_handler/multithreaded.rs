use game::Game;
use game::gameloop::{Update,Render};
use game_handler::GameHandler as GameHandlerTrait;
use graphics::renderer::Renderer;
use std::thread::Thread;
use std::io::timer;
use time;

enum RenderingMessage<T : Send>{
	Stop,
	Data(T),
}

pub struct GameHandler<G,R,RenderData,Exit>
	where G: Game<(),RenderData,Exit> + Send + Clone,
	      R: Renderer + Send;

impl<G,R,RenderData,Exit> GameHandlerTrait<G,R,RenderData,Exit> for GameHandler<G,R,RenderData,Exit>
	where G: Game<(),RenderData,Exit> + Send + Clone,
	      R: Renderer + Send,
{
	fn run(&self,renderer: R,game: &mut G) -> Exit{
		//Render
		let mut render_game = game.clone();

		let (render_send,render_receive) = channel::<RenderingMessage<G>>();
		let render_thread = Thread::spawn(move ||{
			let mut render_data = render_game.init_render(&renderer);

			loop{
				match render_receive.try_recv(){
					Ok(message) => match message{
						RenderingMessage::Data(new_data) => {render_game = new_data;},

						//Check if the render loop should stop.
						RenderingMessage::Stop => break,
					},
					_ => {}
				}

				render_game.render(&renderer,&mut render_data);
			}
		});

		//Event and Update
		let mut previous_time = time::get_time();
		let mut next_time     = previous_time;

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
			render_send.send(RenderingMessage::Data(game.clone()));

			//Timing
			let current_time = time::get_time();
			next_time = next_time + game.target_time_per_frame();
			timer::sleep(next_time - current_time);
			previous_time = current_time;
		}

		//Tell the tasks to stop.
		render_send.send(RenderingMessage::Stop);

		//Wait for tasks to stop (and therefore the lifetimes of all the borrowed refs are valid. But rustc doesn't think so?)
		render_thread.join();
		
		return exit_data;
	}
}
