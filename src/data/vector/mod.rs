use std::num::Zero;
use std::ops::Neg;

pub mod coord_vector;

//TODO: More generic, do not use Self for the "other" paramters. Will it be inefficient?
pub trait Vector2<T>
	: Add<Self,Self>
	+ Sub<Self,Self>
	+ Mul<T,Self>
	+ Div<T,Self>
	+ Neg<Self>
	+ Zero
{
	fn from_generic<V: Vector2<T>>(v: &V) -> Self;

	/// The x coordinate that this vector represents
	fn x(&self) -> T;

	/// The y coordinate that this vector represents
	fn y(&self) -> T;
	
	/// The magnitude/length of this vector
	fn magnitude(&self) -> T;

	/// The direction/angle of this vector
	fn direction(&self) -> T;

	fn dot_product<V: Vector2<T>>(&self,other: &V) -> T;

	/// Projection of self on other
	fn project<V: Vector2<T>>(&self,other: &V) -> V;

	/// The unit vector (Length set to 1 but still having the same direction) of this vector
	fn unit(&self) -> Self;

	//TODO: Mutable methods
}
