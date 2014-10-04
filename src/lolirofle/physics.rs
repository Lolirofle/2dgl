extern crate std;

use super::vector::Vector2 as Vector2;
use super::gameloop::Updatable as Updatable;

/*
 * Mass in kg
 * Length in meter where 1 m = 16 px resulting average 1.75 m to 28 px
 */

pub type Mass = f32;

/*TODO: Multiple implementations of the same type withNot yet implemented in rustc
https://www.mail-archive.com/rust-dev@mozilla.org/msg10971.html
https://github.com/rust-lang/rfcs/blob/master/active/0024-traits.md
http://smallcultfollowing.com/babysteps/blog/2012/10/04/refining-traits-slash-impls/
impl Add<f32,Mass> for Mass{
	fn add(&self, other: &f32) -> Mass{
		return Mass(self.0 + other);
	}
}
*/

pub trait Existence{
	fn get_position(&self) -> Vector2<f32>;
}

pub trait WithPhysics: Existence + Updatable{
	fn get_mass(&self) -> Mass;
	fn get_velocity(&self) -> Vector2<f32>;
}
