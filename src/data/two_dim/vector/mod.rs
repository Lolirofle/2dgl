//! Mathematical vector data structures

use num;

//Re-exports
pub use self::coord::Coord;
pub use self::polar::Polar;

mod coord;
mod polar;

//TODO: More generalized, do not use Self for the "other" paramters. Will it be inefficient?
/// Generalized two dimensional vector
pub trait Vector<T>
	: Add<Self,Self>
	+ Sub<Self,Self>
	+ Mul<T,Self>
	+ Div<T,Self>
	+ Neg<Self>
	+ num::Zero
{
	/// Constructor for constructing from a generalized vector
	fn from_vector<V: Vector<T>>(v: &V) -> Self;

	/// The x coordinate that this vector represents
	fn x(&self) -> T;

	/// The y coordinate that this vector represents
	fn y(&self) -> T;
	
	/// The magnitude/length of this vector
	fn magnitude(&self) -> T;

	/// The direction/angle of this vector
	fn direction(&self) -> T;

	fn dot_product<V: Vector<T>>(&self,other: &V) -> T;

	/// Projection of self on other
	fn project<V: Vector<T>>(&self,other: &V) -> V;//TODO: Return type should be Self or use a type parameter. But then it becomes really verbose

	/// The unit vector (Length set to 1 but still having the same direction) of this vector
	fn unit(&self) -> Self;

	fn set_x(&mut self,x: T);
	fn set_y(&mut self,y: T);
	fn set_magnitude(&mut self,magnitude: T);
	fn set_direction(&mut self,direction: T);
}
