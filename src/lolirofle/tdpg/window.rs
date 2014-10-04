extern crate gl;
extern crate glfw;

use std::task::TaskBuilder;
use glfw::Context;
use lolirofle::gl::renderer::Renderer as Renderer;

fn glfw_loop(
	glfw:        glfw::Glfw,
	window:      &mut glfw::Window,
	events:      Receiver<(f64,glfw::WindowEvent)>,
	renderer:    Renderer,
	update_func: fn(),
	render_func: fn(Renderer),
	event_func:  fn(&mut glfw::Window,glfw::WindowEvent)
){
	//Rendering
	let rendercontext = window.render_context();

	let (sendchannel,receivechannel) = channel();
	let rendertask = TaskBuilder::new().named("Render Task");
	let mut rendertask_done = rendertask.try_future(proc(){
		rendercontext.make_current();
		loop{
			//Check if the rendering should stop.
			if receivechannel.try_recv() == Ok(()){
				break;
			}
			
			render_func(renderer);

			//Perform rendering calls
			rendercontext.swap_buffers();
		}
	});

	//Events
	while !window.should_close(){
		//Fetch events
		glfw.wait_events();

		//For each event
		for (_,event) in glfw::flush_messages(&events){
			event_func(window,event);
		}

		update_func();
	}

	//Tell the render task to exit.
	sendchannel.send(());
	//Wait for acknowledgement that the rendering was completed.
	let _ = rendertask_done.get_ref();
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
	window.make_current();
	window.set_key_polling(true);

	//Initialize GL
	gl::load_with(|s| window.get_proc_address(s));
	gl::ClearColor(0.0,0.0,0.0,1.0);
	gl::Enable(gl::TEXTURE_2D);

	gl::Enable(gl::BLEND);
	gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);
	gl::Disable(gl::DEPTH_TEST);

	///////////////////////////////////////////////
	// Prepare GLSL program

	let renderer = Renderer::initiated();

	//View/projection
	renderer.init_projection(0,0,640,480);

	//Main loop
	glfw_loop(glfw,&mut window,events,renderer,on_update,on_render,on_event);

	renderer.close();
}

fn on_event(window:&mut glfw::Window,event:glfw::WindowEvent){
	match event{
		glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {window.set_should_close(true)}
		_ => {}
	}
}

fn on_update(){}

fn on_render(renderer: Renderer){
	gl::Clear(gl::COLOR_BUFFER_BIT);
	renderer.render_rectangle(
		0.0,
		0.0,
		16.0,
		16.0
	);
}
