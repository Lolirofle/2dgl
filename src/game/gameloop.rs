use graphics::renderer::Renderer;
use std::time::Duration;

pub trait Updatable<U>{//TODO: Transitive verbs as trait names, also avoid suffixes. Maybe change to just Update?
	fn update(&mut self,data: U,delta_time: Duration);
}

pub trait Renderable{
	fn render(&self,renderer: &Renderer);
}

pub trait EventHandler<E>{
	fn event(&mut self,event: E);
}
