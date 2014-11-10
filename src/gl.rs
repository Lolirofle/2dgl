#![doc(hidden)]

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
