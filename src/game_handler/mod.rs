use game::Game;
use graphics::renderer::Renderer;

pub mod multithreaded;
pub mod singlethreaded;

pub trait GameHandler<G,R,RenderData>
	where G: Game<(),RenderData>,
	      R: Renderer{
	fn run(&self,renderer: R,game: &mut G);
}
