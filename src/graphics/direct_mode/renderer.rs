use gl;
use gl::types::*;
use data::vector::Vector2;
use graphics::renderer::Renderer as RendererTrait;

pub struct Renderer;

impl Renderer{
	pub unsafe fn new() -> Renderer{
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
	}
}

impl RendererTrait for Renderer{
	unsafe fn render_rectangle(&self,Vector2(x,y): Vector2<GLfloat>,Vector2(w,h): Vector2<GLfloat>){
		let x2 = x+w;
		let y2 = y+h;
		gl::Begin(gl::LINE_LOOP);
			gl::Vertex2f(x,y);
			gl::Vertex2f(x,y2);
			gl::Vertex2f(x2,y2);
			gl::Vertex2f(x2,y);
		gl::End();
	}

	unsafe fn init_projection(&self,x:GLint,y:GLint,width:GLuint,height:GLuint){
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
	}
}
