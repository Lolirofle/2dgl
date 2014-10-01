pub trait Game{
	fn init(&mut self);
	fn close(&mut self);
	fn update(&mut self,delta_time: uint);
	fn render(&self);
}
