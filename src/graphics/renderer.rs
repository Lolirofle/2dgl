use gl::types::*;
use data::vector::Vector2;

pub trait Renderer{
	unsafe fn render_rectangle(&self,position: Vector2<GLfloat>,size: Vector2<GLfloat>);
	unsafe fn init_projection(&self,x:GLint,y:GLint,width:GLuint,height:GLuint);
}
