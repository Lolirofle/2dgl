use core::num;
use std::num::FloatMath;
use std::fmt;

use data::vector2::Vector as VectorTrait;

/// Vector that uses the dimensional axes as internal storage
#[deriving(Clone)]
pub struct Vector<T>{
	/// The horizontal axis
	pub x: T,
	
	/// The vertical axis
	pub y: T
}

impl<T: num::Float + FloatMath + Mul<T,T> + Div<T,T> + num::Zero> VectorTrait<T> for Vector<T>{//TODO: Mul and Div are only required for `dot_product`, `unit` and `project`. Separate if that will be implemented in rustc
	fn from_vector2<V: VectorTrait<T>>(v: &V) -> Vector<T>{
		return Vector{x: v.x(),y: v.y()};
	}

	#[inline(always)]
	fn x(&self) -> T{self.x}
	
	#[inline(always)]
	fn y(&self) -> T{self.y}

	fn magnitude(&self) -> T where T: num::Float{
		self.x.hypot(self.y)
	}

	fn direction(&self) -> T{
		self.y.atan2(self.x)
	}

	fn dot_product<V: VectorTrait<T>>(&self,other: &V) -> T{
		self.x * other.x() + self.y * other.y()
	}

	fn unit(&self) -> Vector<T>{
		*self / self.magnitude()
	}

	fn project<V: VectorTrait<T>>(&self,other: &V) -> V{
		(*other) * (self.dot_product(other) / other.dot_product(other))
	}
}

impl<T: num::Float> Vector<T>{
	pub fn limit_magnitude(&mut self,magnitude: T){
		let current_magnitude = self.x*self.x + self.y*self.y;
		if current_magnitude > magnitude*magnitude{
			let d = magnitude/current_magnitude.sqrt();
			self.x = self.x*d;
			self.y = self.y*d;
		}
	}
}

impl<T: fmt::Show> fmt::Show for Vector<T>{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		return write!(f,"vec({},{})",self.x,self.y);
	}
}

impl<T: Add<T,T>> Add<Vector<T>,Vector<T>> for Vector<T>{
	fn add(&self, other: &Vector<T>) -> Vector<T>{
		return Vector{x: self.x + other.x,y: self.y + other.y};
	}
}

impl<T: Sub<T,T>> Sub<Vector<T>,Vector<T>> for Vector<T>{
	fn sub(&self, other: &Vector<T>) -> Vector<T>{
		return Vector{x: self.x - other.x,y: self.y - other.y};
	}
}

impl<T: Mul<T,T>> Mul<T,Vector<T>> for Vector<T>{
	fn mul(&self, other: &T) -> Vector<T>{
		return Vector{x: self.x * *other,y: self.y * *other};
	}
}

impl<T: Div<T,T>> Div<T,Vector<T>> for Vector<T>{
	fn div(&self, other: &T) -> Vector<T>{
		return Vector{x: self.x / *other,y: self.y / *other};
	}
}

impl<T: Neg<T>> Neg<Vector<T>> for Vector<T>{
	fn neg(&self) -> Vector<T>{
		return Vector{x: -self.x,y: -self.y};
	}
}

impl<T: num::Zero> num::Zero for Vector<T>{
	fn zero() -> Vector<T>{
		Vector{x: num::Zero::zero(),y: num::Zero::zero()}
	}

	fn is_zero(&self) -> bool{
		self.x.is_zero() && self.y.is_zero()
	}
}

impl<T: num::One> /*num::One for*/ Vector<T>{
	pub fn one() -> Vector<T>{
		Vector{x: num::One::one(),y: num::One::one()}
	}
}

impl<T: num::SignedInt> Vector<T>{
	pub fn abs(&self) -> Vector<T>{
		return Vector{x: self.x.abs(),y: self.y.abs()};
	}
}

#[test]
fn vector_limit1(){
	let mut v = Vector{x: 3.0 as f32,y: 4.0};
	v.limit_magnitude(2.5);
	let m = v.magnitude();
	println!("{}.magnitude = {} = 2.5",v,m);
	assert!(m==2.5 || (2.5-m).abs() < 1.0e-06);
}

#[test]
fn vector_limit2(){
	let mut v = Vector{x: 100.0 as f32,y: 100.0};
	v.limit_magnitude(10.0);
	let m = v.magnitude();
	println!("{}.magnitude = {} = 10.0",v,m);
	assert!(m==10.0 || (10.0-m).abs() < 1.0e-06);
}
