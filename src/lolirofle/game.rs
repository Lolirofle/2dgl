use lolirofle::gameloop::Updatable as Updatable;
use lolirofle::gameloop::Renderable as Renderable;

pub trait Game : Updatable + Renderable{
	fn init(&mut self);
	fn close(&mut self);
}
