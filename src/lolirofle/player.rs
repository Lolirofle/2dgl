use lolirofle::gameloop::*;
use lolirofle::gl::renderer::Renderer;
use lolirofle::physics::Existence;
use lolirofle::physics::Mass;
use lolirofle::physics::WithPhysics;
use lolirofle::tdpg::TdpgGame;
use lolirofle::vector::Vector2;

#[deriving(Clone)]
pub struct Player{
	force: Vector2<f32>,
	position: Vector2<f32>,
	velocity: Vector2<f32>
}
impl Player{
	pub fn new() -> Player{
		return Player{
			force   : Vector2::new(0.0,0.0),
			position: Vector2::new(0.0,0.0),
			velocity: Vector2::new(0.0,0.0),
		};
	}
}
impl Existence for Player{
	fn get_position(&self) -> Vector2<f32>{
		return self.position;
	}
}
impl Updatable<TdpgGame> for Player{
	fn update(&mut self,game: &TdpgGame,delta_time: f64){
		let acceleration = self.force/self.get_mass();

		self.velocity = self.velocity + acceleration  * (delta_time as f32);
		self.velocity.limit(game.velocity_max*game.pixels_per_meter);
		self.position = self.position + self.velocity * (delta_time as f32) / 2.0;
		self.force = Vector2::new(0.0,0.0);
	}
}
impl Renderable for Player{
	fn render(&self,renderer: &Renderer){
		renderer.render_rectangle(
			self.get_position(),
			Vector2(16.0 as f32,16.0)
		);
	}
}
impl EventHandler for Player{
	fn event(&mut self,e: Event){
		match e{
			Jump(f) => {
				self.velocity = self.velocity-Vector2::new(0.0,f);
			},
			Move(v) => {
				self.velocity = self.velocity+v;
			},
		}
	}
}
impl WithPhysics for Player{
	fn get_mass(&self) -> Mass{
		return 5.0;
	}
	fn get_velocity(&self) -> Vector2<f32>{
		return self.velocity;
	}
	fn add_force(&mut self,f: Vector2<f32>){
		self.force = self.force+f;
	}
}
