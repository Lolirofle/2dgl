use game::Game;
use graphics::renderer::Renderer;

pub mod multithreaded;
pub mod singlethreaded;

pub trait GameHandler<G,R,E,RenderData>
	where G: Game<E>,
	      R: Renderer{
	fn run(&self,renderer: R,instance: &Instance<G,E,RenderData>,game: &mut G);
}

pub trait Instance<G,E,RenderData>
	where G: Game<E>{
	fn init(&self) -> RenderData;
	fn init_render(&self/*,data: &RenderData*/);
	fn event(&self) -> &Iterator<E>;
}
