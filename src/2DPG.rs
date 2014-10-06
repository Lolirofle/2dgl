#![feature(globs)]
#![feature(tuple_indexing)]

extern crate core;
extern crate gl;
extern crate glfw;

use lolirofle::player::Player as Player;
use lolirofle::gameloop::Updatable as Updatable;
use lolirofle::gameloop::Renderable as Renderable;
use lolirofle::gl::renderer::Renderer as Renderer;
use lolirofle::tdpg::window;
mod lolirofle;

#[deriving(Clone)]
struct TdpgGame{
	player: Player
}
impl Updatable for TdpgGame{
	fn update(&mut self,delta_time: f64){
		self.player.update(delta_time);
	}
}
impl Renderable for TdpgGame{
	fn render(&self,renderer: &Renderer){
		gl::Clear(gl::COLOR_BUFFER_BIT);

		self.player.render(renderer);
	}
}

fn init() -> TdpgGame{
	return TdpgGame{
		player: Player::new()
	};
}

fn on_event(window:&mut glfw::Window,event:glfw::WindowEvent){
	match event{
		glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {
			window.set_should_close(true);
		},
		glfw::KeyEvent(glfw::KeySpace,_,glfw::Press,_) => {
			window.set_should_close(true);
		},
		_ => {}
	}
}


fn main(){
	window::run(init,on_event);
}

//https://mail.mozilla.org/pipermail/rust-dev/2013-November/006714.html
//http://smallcultfollowing.com/babysteps/blog/2013/10/24/single-inheritance/
