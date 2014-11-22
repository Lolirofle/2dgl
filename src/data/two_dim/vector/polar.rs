use core::num::Float;
use num;
use std::num::FloatMath;

use data::two_dim::vector::{Coord,Vector};

#[deriving(Clone)]
pub struct Polar<T>{
	direction: T,
	magnitude: T
}

impl<T: Float + FloatMath + Mul<T,T> + Div<T,T> + num::One + num::Zero> Vector<T> for Polar<T>{//TODO: Mul and Div are only required for `dot_product`, `unit` and `project`. Separate if that will be implemented in rustc
	fn from_vector<V: Vector<T>>(v: &V) -> Polar<T>{
		Polar{direction: v.direction(),magnitude: v.magnitude()}
	}

	fn x(&self) -> T{
		self.magnitude*self.direction.cos()
	}
	
	fn y(&self) -> T{
		self.magnitude * self.direction.sin()
	}

	#[inline(always)]
	fn magnitude(&self) -> T{self.magnitude}

	#[inline(always)]
	fn direction(&self) -> T{self.direction}

	fn dot_product<V: Vector<T>>(&self,other: &V) -> T{
		self.magnitude * other.magnitude() * (self.direction-other.direction()).cos()
	}

	fn unit(&self) -> Polar<T>{
		Polar{direction: self.direction,magnitude: num::One::one()}
	}

	fn project<V: Vector<T>>(&self,other: &V) -> V{
		(*other) * (self.dot_product(other) / other.dot_product(other))
	}

	fn set_x(&mut self,x: T){
		*self = Vector::from_vector(&Coord{x: self.x() + x,y: self.y()})
	}

	fn set_y(&mut self,y: T){
		*self = Vector::from_vector(&Coord{x: self.x(),y: self.y() + y})
	}

	#[inline(always)]
	fn set_magnitude(&mut self,magnitude: T){
		self.magnitude = magnitude;
	}

	#[inline(always)]
	fn set_direction(&mut self,direction: T){
		self.direction = direction;
	}
}

impl<T: Add<T,T> + Float + FloatMath + Mul<T,T> + Div<T,T> + num::One + num::Zero> Add<Polar<T>,Polar<T>> for Polar<T>{
	fn add(&self, other: &Polar<T>) -> Polar<T>{
		Vector::from_vector(&Coord{x: self.x() + other.x(),y: self.y() + other.y()})
	}
}

impl<T: Sub<T,T> + Float + FloatMath + Mul<T,T> + Div<T,T> + num::One + num::Zero> Sub<Polar<T>,Polar<T>> for Polar<T>{
	fn sub(&self, other: &Polar<T>) -> Polar<T>{
		Vector::from_vector(&Coord{x: self.x() - other.x(),y: self.y() - other.y()})
	}
}

impl<T: Mul<T,T> + num::One> Mul<T,Polar<T>> for Polar<T>{
	fn mul(&self, x: &T) -> Polar<T>{
		Polar{direction: self.direction * num::One::one(),magnitude: self.magnitude * *x}
	}
}

impl<T: Div<T,T> + num::One> Div<T,Polar<T>> for Polar<T>{
	fn div(&self, x: &T) -> Polar<T>{
		Polar{direction: self.direction * num::One::one(),magnitude: self.magnitude / *x}
	}
}

impl<T: Neg<T> + Float> Neg<Polar<T>> for Polar<T>{
	fn neg(&self) -> Polar<T>{
		Polar{direction: self.direction + Float::pi(),magnitude: self.magnitude}
	}
}

impl<T: num::Zero + Float + FloatMath + Mul<T,T> + Div<T,T> + num::One> num::Zero for Polar<T>{
	fn zero() -> Polar<T>{
		Polar{direction: num::Zero::zero(),magnitude: num::Zero::zero()}
	}

	fn is_zero(&self) -> bool{
		self.magnitude.is_zero()
	}
}

impl<T: num::Signed> Polar<T>{
	pub fn abs(&self) -> Polar<T>{
		Polar{direction: self.direction.abs(),magnitude: self.magnitude.abs()}
	}
}
