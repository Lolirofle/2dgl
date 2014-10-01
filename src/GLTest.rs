extern crate gl;
extern crate glfw;

use std::task::TaskBuilder;
use std::mem;
use std::ptr;
use gl::types::*;
use glfw::Context;
use lolirofle::gl::*;
mod lolirofle;

//Vertex data
static vertex_data: [GLfloat, ..6] = [
	 0.0, 0.0,
	 1.0, 0.0,
	 0.0, 1.0
];
//Import shader sources
static vertex_shader_src:   &'static str = include_str!("vertex_shader.glsl");
static fragment_shader_src: &'static str = include_str!("fragment_shader.glsl");

fn glfw_loop(
	glfw:        glfw::Glfw,
	window:      &mut glfw::Window,
	events:      Receiver<(f64,glfw::WindowEvent)>,
	update_func: fn(),
	render_func: fn(),
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
			
			render_func();

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

	//Initialize GLSL shaders
	let vertex_shader   = shaders::compile_shader(vertex_shader_src  ,gl::VERTEX_SHADER);
	let fragment_shader = shaders::compile_shader(fragment_shader_src,gl::FRAGMENT_SHADER);
	let shader_program  = shaders::link_program(vertex_shader,fragment_shader);

	let mut vao = 0;
	let mut vbo = 0;

	unsafe {
		//Initialize Vertex Array Object
		gl::GenVertexArrays(1,&mut vao);
		gl::BindVertexArray(vao);

		//Initialize Vertex Buffer Object, copying the vertex data to it
		gl::GenBuffers(1,&mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
			mem::transmute(&vertex_data[0]),
			gl::STATIC_DRAW
		);

		//Use shader program
		gl::UseProgram(shader_program);
		"out_color".with_c_str(|ptr| gl::BindFragDataLocation(shader_program,0,ptr));

		// Specify the layout of the vertex data
		let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(shader_program, ptr));
		gl::EnableVertexAttribArray(pos_attr as GLuint);
		gl::VertexAttribPointer(pos_attr as GLuint,2,gl::FLOAT,gl::FALSE as GLboolean,0,ptr::null());
	}

	//Main loop
	glfw_loop(glfw,&mut window,events,on_update,on_render,on_event);

	//Free
	gl::DeleteProgram(shader_program);
	gl::DeleteShader(fragment_shader);
	gl::DeleteShader(vertex_shader);
	unsafe {
		gl::DeleteBuffers(1, &vbo);
		gl::DeleteVertexArrays(1, &vao);
	}
}

fn on_event(window:&mut glfw::Window,event:glfw::WindowEvent){
	match event{
		glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {window.set_should_close(true)}
		_ => {}
	}
}

fn on_update(){}

fn on_render(){
	gl::Clear(gl::COLOR_BUFFER_BIT);
	gl::DrawArrays(gl::TRIANGLES,0,3);
}
