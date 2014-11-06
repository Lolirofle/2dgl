use game::Game;
use graphics::renderer::Renderer;

pub trait Updatable<G: Game>{
	fn update(&mut self,game: &G,delta_time: f64);
}

pub trait Renderable{
	fn render(&self,renderer: &Renderer);
}

pub trait EventHandler<E>{
	fn event(&mut self,event: E);
}
