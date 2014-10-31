extern crate core;
extern crate gl;
extern crate glfw;

use glfw::Context;
use lolirofle::game::gameloop::*;
use lolirofle::game::Game;
use lolirofle::game_handler::GameHandler as GameHandlerTrait;
use lolirofle::gl::renderer::Renderer;
use std::io::timer;
use std::time::Duration;

fn glfw_loop<G,R>(
	glfw:        glfw::Glfw,
	window:      &mut glfw::Window,
	events:      Receiver<(f64,glfw::WindowEvent)>,
	renderer:    R,
)
	where G: Game,
	      R: Renderer{
	//Init
	let mut game: G       = Game::init();
	let mut previous_time = glfw.get_time();
	let mut next_time     = glfw.get_time();

	//Render
	let render_context = window.render_context();

	render_context.make_current();
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
		game.update(glfw.get_time() - previous_time);

		//Timing
		let current_time = glfw.get_time();
		timer::sleep(Duration::milliseconds(((next_time - current_time)*1000.0) as i64));
		previous_time = current_time;

		//Render
		game.render(&renderer);
		render_context.swap_buffers();
	}
}

pub struct GameHandler<G,R>
	where G: Game,
	      R: Renderer;
impl<G: Game,R: Renderer> GameHandlerTrait<G> for GameHandler<G,R>{
	fn run(&self){
		let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		//Window
		glfw.window_hint(glfw::ContextVersion(3,2));//TODO: Not always
		glfw.window_hint(glfw::OpenglForwardCompat(true));
		glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

		let (mut window,events) = glfw.create_window(640,480,"GLTest",glfw::Windowed)
			.expect("Failed to create GLFW window.");

		//Initialize window
		window.set_all_polling(true);
		window.make_current();
		glfw.set_swap_interval(0);

		//Initialize GL
		gl::load_with(|s| window.get_proc_address(s));

		gl::Enable(gl::TEXTURE_2D);
		gl::Disable(gl::DEPTH_TEST);
		
		gl::Disable(gl::LIGHTING);//TODO: Deprecated OpenGL functions?
		gl::ShadeModel(gl::FLAT);

		gl::ClearColor(0.0,0.0,0.0,1.0);
		gl::ClearDepth(1.0);

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);

		{
			let renderer: R = Renderer::initiated();
			renderer.init_projection(0,0,640,480);

			//Main loop
			glfw_loop::<G,R>(glfw,&mut window,events,renderer);
		}
	}
}
