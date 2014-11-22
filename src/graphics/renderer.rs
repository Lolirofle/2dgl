use data::two_dim::vector;
use gl::types::*;

/// A renderer that can render in two dimensional space
pub trait Renderer{
	/// Renders a axis-aligned rectangle using the specified position and dimensions in pixels
	fn render_rectangle(&self,position: vector::Coord<GLfloat>,size: vector::Coord<GLfloat>);
	
	/// Initializes the projection/view for the renderer
	fn init_projection(&self,x:GLint,y:GLint,width:GLuint,height:GLuint);

	/// Clears the canvas
	fn clear(&self);
}
