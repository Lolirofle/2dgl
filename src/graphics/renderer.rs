use data::vector2::coord_vector::Vector;
use gl::types::*;

/// A renderer that can render in two dimensional space
pub trait Renderer{
	/// Renders a axis-aligned rectangle using the specified position and dimensions in pixels
	fn render_rectangle(&self,position: Vector<GLfloat>,size: Vector<GLfloat>);
	
	/// Initializes the projection/view for the renderer
	fn init_projection(&self,x:GLint,y:GLint,width:GLuint,height:GLuint);

	/// Clears the canvas
	fn clear(&self);
}
