use gl::types::*;
use data::vector::Vector2;

pub trait Renderer{
	fn initiated() -> Self;
	fn render_rectangle(&self,position: Vector2<GLfloat>,size: Vector2<GLfloat>);
	fn init_projection(&self,x:GLint,y:GLint,width:GLuint,height:GLuint);
}
