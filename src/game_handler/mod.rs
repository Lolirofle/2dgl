//! Takes care of the abstraction of a game application

use game::Game;
use graphics::renderer::Renderer;

pub mod multithreaded;
pub mod singlethreaded;

pub trait GameHandler<G,R,RenderData,Exit>
	where G: Game<(),RenderData,Exit>,
	      R: Renderer{
	fn run(&self,renderer: R,game: &mut G) -> Exit;
}
