extern crate gl;
extern crate glfw;

use glfw::Context;
use std::task::TaskBuilder;

fn glfw_loop(
	glfw:glfw::Glfw,
	window:&mut glfw::Window,
	events:Receiver<(f64,glfw::WindowEvent)>,
	renderfunc:fn(),
	eventfunc:fn(&mut glfw::Window,glfw::WindowEvent)
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
			
			renderfunc();

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
			eventfunc(window,event);
		}
	}

	//Tell the render task to exit.
	sendchannel.send(());
	//Wait for acknowledgement that the rendering was completed.
	let _ = rendertask_done.get_ref();
}

fn main(){
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

	//Initialize gl
	gl::load_with(|s| glfw.get_proc_address(s));
	gl::ClearColor(0.0,0.0,0.0,1.0);
	gl::Enable(gl::TEXTURE_2D);
	gl::Enable(gl::BLEND);
	gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);
	gl::Disable(gl::DEPTH_TEST);

	//Main loop
	glfw_loop(glfw,&mut window,events,on_render,on_event);
}

fn on_event(window:&mut glfw::Window,event:glfw::WindowEvent){
	match event{
		glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {window.set_should_close(true)}
		_ => {}
	}
}

fn on_render(){
	gl::Clear(gl::COLOR_BUFFER_BIT);
}
