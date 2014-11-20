//! Mathematical vector data structures

use core::num;

pub mod coord_vector;

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
	fn from_vector2<V: Vector<T>>(v: &V) -> Self;

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
	fn project<V: Vector<T>>(&self,other: &V) -> V;

	/// The unit vector (Length set to 1 but still having the same direction) of this vector
	fn unit(&self) -> Self;

	//TODO: Mutable methods
}
