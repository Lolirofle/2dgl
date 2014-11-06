use gl;
use glfw;

use glfw::Context;
use graphics::renderer::Renderer;
use game::gameloop::*;
use game::Game;
use game_handler::GameHandler as GameHandlerTrait;
use std::task::TaskBuilder;
use std::io::timer;
use std::time::Duration;

enum RenderingMessage<T : Send>{
	RenderStop,
	RenderData(T),
}

fn glfw_loop<G,R>(
	glfw:        glfw::Glfw,
	window:      &mut glfw::Window,
	events:      Receiver<(f64,glfw::WindowEvent)>,
	renderer:    R,
)
	where G: Game + Send + Clone,
	      R: Renderer + Send
{
	//Init
	let mut game: G = Game::init();

	//Render
	let render_context = window.render_context();
	let mut render_game = game.clone();

	let (render_send,render_receive) = channel::<RenderingMessage<G>>();
	let render_task = TaskBuilder::new().try_future(proc(){
		render_context.make_current();

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
			render_context.swap_buffers();
		}
	});

	//Event and Update
	let mut previous_time = glfw.get_time();
	let mut next_time = glfw.get_time();
	loop{
		next_time+=1.0/60.0;

		//Fetch events, processing each
		glfw.poll_events();
		for (_,event) in glfw::flush_messages(&events){
			game.event(window,event);
		}

		//Check if the window should close
		if window.should_close(){
			break;
		}

		//Update
		game.update(glfw.get_time() - previous_time);//TODO: CHange game.update parameter to Duration
		render_send.send(RenderData(game.clone()));

		//Timing
		let current_time = glfw.get_time();
		timer::sleep(Duration::milliseconds(((next_time - current_time)*1000.0) as i64));
		previous_time = current_time;
	}

	//Tell the tasks to stop.
	render_send.send(RenderStop);

	//Wait for tasks to stop
	let _ = render_task.unwrap();
}

pub struct GameHandler<G,R>
	where G: Game + Send + Clone,
	      R: Renderer + Send;
impl<G: Game + Send + Clone,R: Renderer + Send> GameHandlerTrait<G> for GameHandler<G,R>{
	fn run(&self){
		let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		//Window
		/*glfw.window_hint(glfw::ContextVersion(3,2));
		glfw.window_hint(glfw::OpenglForwardCompat(true));
		glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));*/

		let (mut window,events) = glfw.create_window(640,480,"GLTest",glfw::Windowed)
			.expect("Failed to create GLFW window.");

		//Initialize window
		window.set_all_polling(true);
		window.make_current();

		//Initialize GL
		gl::load_with(|s| window.get_proc_address(s));
		gl::ClearColor(0.0,0.0,0.0,1.0);
		gl::Enable(gl::TEXTURE_2D);

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);
		gl::Disable(gl::DEPTH_TEST);

		{
			let renderer: R = Renderer::initiated();
			renderer.init_projection(0,0,640,480);

			//Main loop
			glfw_loop::<G,R>(glfw,&mut window,events,renderer);
		}
	}
}
