use game::gameloop::{Updatable,Renderable,EventHandler};
use std::time::Duration;

pub mod gameloop;

pub trait Game<E>: Updatable<()> + Renderable + EventHandler<E>{//TODO: R shouldn't need to be defined in Game? How to avoid?
	fn should_exit(&self) -> bool;

	//1/fps (frames per second)
	fn target_time_per_frame(&self) -> Duration;
}
