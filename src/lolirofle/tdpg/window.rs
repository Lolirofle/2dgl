extern crate core;
extern crate gl;
extern crate glfw;

use std::task::TaskBuilder;
use std::io::timer;
use std::time::Duration;
use glfw::Context;
use lolirofle::gl::renderer::Renderer;
use lolirofle::gameloop::Updatable;
use lolirofle::gameloop::Renderable;

enum RenderingMessage<T : Send>{
	RenderStop,
	RenderData(T),
}

fn glfw_loop<G: Updatable + Renderable + Send + Clone>(
	glfw:        glfw::Glfw,
	window:      &mut glfw::Window,
	events:      Receiver<(f64,glfw::WindowEvent)>,
	renderer:    Renderer,
	init_func:   fn() -> G,
	event_func:  fn(&mut glfw::Window,glfw::WindowEvent)
){
	//Init
	let mut game = init_func();

	//Render
	let render_context = window.render_context();
	let mut render_game = game.clone();

	let (render_send,render_receive): (Sender<RenderingMessage<G>>,Receiver<RenderingMessage<G>>) = channel();
	let render_task = TaskBuilder::new().named("Render Loop Task").try_future(proc(){
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

	//Events and Update
	let mut previous_time = glfw.get_time();
	let mut next_time = glfw.get_time();
	loop{
		next_time+=1.0/60.0;

		//Fetch events, processing each
		glfw.poll_events();
		for (_,event) in glfw::flush_messages(&events){
			event_func(window,event);
		}

		//Check if the window should close
		if window.should_close(){
			break;
		}

		//Update
		game.update(glfw.get_time() - previous_time);
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

pub fn run<G: Updatable + Renderable + Send + Clone>(
	init_func: fn() -> G,
	event_func: fn(window:&mut glfw::Window,event:glfw::WindowEvent)
){
	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	//Window
	glfw.window_hint(glfw::ContextVersion(3,2));
	glfw.window_hint(glfw::OpenglForwardCompat(true));
	glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

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
		let renderer = Renderer::initiated();
		renderer.init_projection(0,0,640,480);

		//Main loop
		glfw_loop(glfw,&mut window,events,renderer,init_func,event_func);
	}
}
