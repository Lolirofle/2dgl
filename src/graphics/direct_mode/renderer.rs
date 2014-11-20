extern crate libc;

use data::vector2::coord_vector::Vector;
use gl::types::*;
use gl;
use graphics::renderer::Renderer as RendererTrait;

pub struct Renderer;

impl Renderer{
	pub fn new(proc_address: |&str| -> *const libc::c_void) -> Renderer{unsafe{
		gl::load_with(proc_address);

		gl::Enable(gl::TEXTURE_2D);
		gl::Enable(gl::COLOR_MATERIAL);
		gl::Disable(gl::DEPTH_TEST);

		gl::Disable(gl::LIGHTING);
		gl::ShadeModel(gl::FLAT);

		gl::ClearColor(0.0,0.0,0.0,1.0);
		gl::ClearDepth(1.0);

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);

		return Renderer;
	}}
}

impl RendererTrait for Renderer{
	fn render_rectangle(&self,pos: Vector<GLfloat>,dim: Vector<GLfloat>){unsafe{
		let x2 = pos.x + dim.x;
		let y2 = pos.y + dim.y;
		gl::Begin(gl::LINE_LOOP);
			gl::Vertex2f(pos.x,pos.y);
			gl::Vertex2f(pos.x,y2);
			gl::Vertex2f(x2,y2);
			gl::Vertex2f(x2,pos.y);
		gl::End();
	}}

	fn init_projection(&self,x:GLint,y:GLint,width:GLuint,height:GLuint){unsafe{
		gl::MatrixMode(gl::PROJECTION);
		gl::LoadIdentity();

		gl::Ortho(
			x      as GLdouble,
			width  as GLdouble,
			height as GLdouble,
			y      as GLdouble,
			1.0,
			-1.0
		);
		gl::MatrixMode(gl::MODELVIEW);
		gl::LoadIdentity();

		gl::Viewport(x,y,width as GLint,height as GLint);
	}}

	fn clear(&self){unsafe{
		gl::Clear(gl::COLOR_BUFFER_BIT);
	}}
}
