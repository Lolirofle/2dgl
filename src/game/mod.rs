use game::gameloop::{Update,Render,EventHandler};
use std::time::Duration;

pub mod gameloop;

pub trait Game<E,R>: Update<()> + Render<R> + EventHandler<E>{
	/// Whether the execution of the game should terminate.
	/// This should be handled by the game handler.
	fn should_exit(&self) -> bool;

	/// The amount of time that should be spent on each frame
	/// This is the inverse of FPS: 1/FPS (frames per second)
	fn target_time_per_frame(&self) -> Duration;

	/// Initializes rendering
	fn init_render(&self) -> R;
}