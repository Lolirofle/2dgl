use graphics::renderer::Renderer;
use std::time::Duration;

/// Something that have the ability to update in a game loop
///
/// U: Type of the data sent through the update function
pub trait Update<U>{
	/// Updates
	///
	/// delta_time: Time since last update
	fn update(&mut self,data: U,delta_time: Duration);
}

/// Something that have the ability to render graphics in a game loop
pub trait Render<R>{
	/// Renders using the specified renderer
	///
	/// renderer: Renderer that should be used while rendering
	fn render(&self,renderer: &Renderer,data: &mut R);
}

/// Something that have the ability to handle events in a game loop
///
/// E: Type of the structure that contains an event and its own data that should be important for understanding the event
pub trait EventHandler<E>{
	/// Triggers an single event
	fn event(&mut self,event: E);
}
