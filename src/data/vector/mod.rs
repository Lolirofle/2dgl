use std::num::Zero;
use std::ops::Neg;

pub mod coord_vector;

pub trait Vector2<T>
	: Add<Self,Self>
	+ Sub<Self,Self>
	+ Mul<T,Self>
	+ Div<T,Self>
	+ Neg<Self>
	+ Zero
{
	fn from_generic<V: Vector2<T>>(v: &V) -> Self;

	fn x(&self) -> T;

	fn y(&self) -> T;
	
	fn magnitude(&self) -> T;

	fn direction(&self) -> T;

	fn dot_product(&self,other: &Vector2<T>) -> T;
}
