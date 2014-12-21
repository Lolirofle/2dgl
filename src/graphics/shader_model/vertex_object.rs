use gl::types::*;

pub struct VertexObject{
	pub array: GLuint,
	pub buffer: GLuint,
	pub size: GLsizeiptr
}

impl Copy for VertexObject{}
