use game::Game;
use graphics::renderer::Renderer;

pub trait Update<G: Game>{
	fn update(&mut self,game: &G,delta_time: f64);
}

pub trait Render{
	fn render(&self,renderer: &Renderer);
}

pub trait EventHandler<E>{
	fn event(&mut self,event: E);
}
