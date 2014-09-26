pub trait Updatable{
	fn update(&mut self,delta_time: uint);
}

pub trait Renderable{
	fn render(&self);
}
