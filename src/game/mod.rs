//! Abstractions for a game application

use game::gameloop::{Update,Render,EventHandler};
use graphics::renderer::Renderer;
use std::time::Duration;

pub mod gameloop;

/// An abstract game that is handled by GameHandler
pub trait Game<E,R,Exit>: Update<()> + Render<R> + EventHandler<E>{
	/// Whether the execution of the game should terminate.
	/// This should be handled by the game handler.
	fn should_exit(&self) -> Option<Exit>;

	/// The amount of time that should be spent on each frame
	/// This is the inverse of FPS: 1/FPS (frames per second)
	fn target_time_per_frame(&self) -> Duration;

	/// Initializes rendering
	fn init_render(&self,renderer: &Renderer) -> R;
}
