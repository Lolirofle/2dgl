use gl::types::*;
use data::vector::coord_vector::CoordVector2 as Vector2;

/// A renderer that can render in two dimensional space
pub trait Renderer{
	/// Renders a axis-aligned rectangle using the specified position and dimensions in pixels
	unsafe fn render_rectangle(&self,position: Vector2<GLfloat>,size: Vector2<GLfloat>);
	
	/// Initializes the projection/view for the renderer
	unsafe fn init_projection(&self,x:GLint,y:GLint,width:GLuint,height:GLuint);
}
