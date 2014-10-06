use lolirofle::gl::renderer::Renderer as Renderer;

pub trait Updatable{
	fn update(&mut self,delta_time: f64);
}

pub trait Renderable{
	fn render(&self,renderer: &Renderer);
}
