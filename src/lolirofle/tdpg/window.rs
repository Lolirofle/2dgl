extern crate core;
extern crate gl;
extern crate glfw;

use std::task::TaskBuilder;
use std::io::timer;
use std::time::Duration;
use glfw::Context;
use lolirofle::gl::renderer::Renderer as Renderer;

enum RenderingMessage<T: Send>{
	RenderStop,
	RenderData(T),
}

fn glfw_loop<T: Send + Clone>(
	glfw:        glfw::Glfw,
	window:      &mut glfw::Window,
	events:      Receiver<(f64,glfw::WindowEvent)>,
	renderer:    Renderer,
	init_func:   fn() -> T,
	update_func: fn(&mut T),
	render_func: fn(&Renderer,&T),
	event_func:  fn(&mut glfw::Window,glfw::WindowEvent)
){
	//Init
	let mut data = init_func();

	//Render
	let render_context = window.render_context();
	let mut render_data = data.clone();

	let (render_send,render_receive): (Sender<RenderingMessage<T>>,Receiver<RenderingMessage<T>>) = channel();
	let render_send2 = render_send.clone();
	let render_task = TaskBuilder::new().named("Render Loop Task").try_future(proc(){
		render_context.make_current();

		loop{
			match render_receive.try_recv(){
				Ok(message) => match message{
					RenderData(new_data) => {render_data = new_data;},

					//Check if the render loop should stop.
					RenderStop => break,
				},
				_ => {}
			}

			render_func(&renderer,&render_data);
			render_context.swap_buffers();
		}
	});

	//Events and Update
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
		update_func(&mut data);
		render_send2.send(RenderData(data.clone()));

		//Timing
		timer::sleep(Duration::milliseconds(((next_time - glfw.get_time())*1000.0) as i64));
	}

	//Tell the tasks to stop.
	render_send.send(RenderStop);

	//Wait for tasks to stop
	let _ = render_task.unwrap();
}

pub fn run(){
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
		glfw_loop(glfw,&mut window,events,renderer,on_init,on_update,on_render,on_event);
	}
}

fn on_init() -> int{
	return 5i;
}

fn on_event(window:&mut glfw::Window,event:glfw::WindowEvent){
	match event{
		glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {window.set_should_close(true);}
		_ => {}
	}
}

fn on_update(data: &mut int){
	*data+=1;
}

fn on_render(renderer: &Renderer,data: &int){
	gl::Clear(gl::COLOR_BUFFER_BIT);
	renderer.render_rectangle(
		*data as f32,
		16.0,
		32.0,
		48.0
	);
}
