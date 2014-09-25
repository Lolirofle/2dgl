extern crate std;

trait Game{
	fn init(&mut self);
	fn close(&mut self);
	fn update(&mut self,delta_time: uint);
	fn render(&self);
}

struct Point2<T: std::fmt::Show>(T,T);
struct Mass(f32);

impl Add<Mass,Mass> for Mass{
    fn add(&self, other: &Mass) -> Mass{
        return Mass(self.0 + other.0);
    }
}

/*TODO: Not yet implemented in rustc
https://www.mail-archive.com/rust-dev@mozilla.org/msg10971.html
https://github.com/rust-lang/rfcs/blob/master/active/0024-traits.md
http://smallcultfollowing.com/babysteps/blog/2012/10/04/refining-traits-slash-impls/
impl Add<f32,Mass> for Mass{
    fn add(&self, other: &f32) -> Mass{
        return Mass(self.0 + other);
    }
}
*/

impl<T: std::fmt::Show> std::fmt::Show for Point2<T>{
	fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
		return write!(f,"P2({},{})",self.0,self.1);
	}
}

impl std::fmt::Show for Mass{
	fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
		return write!(f,"{} kg",self.0);
	}
}

trait WithPhysics: Existence{
	fn get_mass(&self) -> Mass;
	fn get_velocity(&self) -> Point2<f32>;
}

trait Existence{
	fn get_position(&self) -> Point2<f32>;
}

trait Updatable{
	fn update(&mut self,delta_time: uint);
}

trait Renderable{
	fn render(&self);
}

struct Player{
	position: Point2<f32>,
	velocity: Point2<f32>
}
impl Existence for Player{
	fn get_position(&self) -> Point2<f32>{
		return self.position;
	}
}
impl Updatable for Player{
	fn update(&mut self,delta_time: uint){
		let acceleration = (
			0.0,
			self.get_mass().0 * 9.82
		);

		self.velocity.0+=acceleration.0 * (delta_time as f32);
		self.velocity.1+=acceleration.1 * (delta_time as f32);
		
		self.position.0+=self.velocity.0 * (delta_time as f32);
		self.position.1+=self.velocity.1 * (delta_time as f32);
	}
}
impl Renderable for Player{
	fn render(&self){}
}
impl WithPhysics for Player{
	fn get_mass(&self) -> Mass{
		return Mass(5.0);
	}
	fn get_velocity(&self) -> Point2<f32>{
		return self.velocity;
	}
}

struct TdpgGame{
	player: Player
}
impl Game for TdpgGame{
	fn init(&mut self){}
	fn close(&mut self){}
	fn update(&mut self,delta_time: uint){
		self.player.update(delta_time);
	}
	fn render(&self){}	
}
