extern crate gl;

use std::str;
use std::ptr;
use gl::types::*;

/**
 * @param shader_type `gl::VERTEX_SHADER` or `gl::FRAGMENT_SHADER`
 */
pub fn compile_shader(src: &str,shader_type: GLenum) -> GLuint{
	let shader = gl::CreateShader(shader_type);
	unsafe {
		//Attempt to compile the shader
		src.with_c_str(|ptr| gl::ShaderSource(shader,1,&ptr,ptr::null()));
		gl::CompileShader(shader);

		// Get the compile status
		let mut status = gl::FALSE as GLint;
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

		// Fail on error
		if status != (gl::TRUE as GLint) {
			let mut len = 0;
			gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
			let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
			gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
			fail!("{}", str::from_utf8(buf.as_slice()).expect("Shader Info Log not valid utf8"));
		}
	}
	shader
}

pub fn link_program(vertex_shader: GLuint,fragment_shader: GLuint) -> GLuint{
	let program = gl::CreateProgram();
	gl::AttachShader(program,vertex_shader);
	gl::AttachShader(program,fragment_shader);
	gl::LinkProgram(program);

	//Check for errors
	unsafe {
		//Get link status
		let mut status = gl::FALSE as GLint;
		gl::GetProgramiv(program,gl::LINK_STATUS,&mut status);

		//Fail on error
		if status != (gl::TRUE as GLint) {
			let mut len: GLint = 0;
			gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
			let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
			gl::GetProgramInfoLog(program,len,ptr::null_mut(),buf.as_mut_ptr() as *mut GLchar);
			fail!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
		}
	}
	program
}
