use lolirofle::gl::renderer::Renderer;
use glfw;

pub trait Game{
	fn init() -> Self;
	fn update(&mut self,delta_time: f64);
	fn render(&self,renderer: &Renderer);
	fn event(&mut self,&mut glfw::Window,glfw::WindowEvent);
}
