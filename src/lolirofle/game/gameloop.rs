use lolirofle::gl::renderer::Renderer;
use lolirofle::game::Game;
use lolirofle::vector::Vector2;

pub trait Updatable<G: Game>{
	fn update(&mut self,game: &G,delta_time: f64);
}

pub trait Renderable{
	fn render(&self,renderer: &Renderer);
}

pub enum Event{
	Move(Vector2<f32>),
	Jump(f32)
}

pub trait EventHandler{
	fn event(&mut self,event: Event);
}
