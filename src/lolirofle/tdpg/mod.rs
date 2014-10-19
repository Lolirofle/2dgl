use lolirofle::player::Player;
use lolirofle::gameloop::*;
use lolirofle::gl::renderer::Renderer;
use lolirofle::physics::WithPhysics;
use lolirofle::game::Game;
use lolirofle::vector::Vector2;
use gl;
use glfw;
use std::mem;

pub mod window;

#[deriving(Clone)]
pub struct TdpgGame{
	player: Player,
	pub pixels_per_meter: f32,
	pub velocity_max: f32,
}
impl Game for TdpgGame{
	fn update(&mut self,delta_time: f64){
		let f = Vector2::new(0.0,9.82*self.player.get_mass()*self.pixels_per_meter);
		self.player.add_force(f);
		unsafe{//TODO: How to fix efficiently
			let self2 = mem::transmute(&*self);
			self.player.update(self2,delta_time);
		}
	}

	fn render(&self,renderer: &Renderer){
		gl::Clear(gl::COLOR_BUFFER_BIT);

		self.player.render(renderer);
	}

	fn event(&mut self,window:&mut glfw::Window,event:glfw::WindowEvent){
		match match event{
			glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {
				window.set_should_close(true);
				None
			},
			glfw::KeyEvent(glfw::KeySpace,_,glfw::Press,_) |
			glfw::KeyEvent(glfw::KeyUp   ,_,glfw::Press,_) => Some(Jump(20.0*16.0)),
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Press,_) => Some(Move(Vector2::new(-10.0*16.0,0.0))),
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Press,_) => Some(Move(Vector2::new( 10.0*16.0,0.0))),
			_ => None
		}{
			Some(e) => {self.player.event(e);},
			None    => {}
		};
	}

	fn init() -> TdpgGame{
		return TdpgGame{
			player: Player::new(),
			pixels_per_meter: 16.0,
			velocity_max: 54.0,
		};
	}
}
