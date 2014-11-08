//! Library for 2D games

#![crate_name = "2dgl"]
#![comment = "Personal library for building games"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs)]
#![feature(phase)]

extern crate core;
extern crate collections;
extern crate rustrt;
extern crate time;

pub mod data;
pub mod game_handler;
pub mod game;
pub mod graphics;

#[doc(hidden)]
mod gl{

	#[phase(plugin)]
	extern crate gl_generator;


	generate_gl_bindings! {
		api: "gl",
		profile: "core",
		version: "3.0",
		generator: "global",
		extensions: [
			"GL_EXT_texture_filter_anisotropic",
		],
	}
}
