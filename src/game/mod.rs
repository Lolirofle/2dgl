use graphics::renderer::Renderer;
use glfw;

pub mod gameloop;

pub trait Game{
	fn init() -> Self;
	fn update(&mut self,delta_time: f64);
	fn render(&self,renderer: &Renderer);
	fn event(&mut self,&mut glfw::Window,glfw::WindowEvent);
}
