use core::num::Float;
use num;
use std::num::FloatMath;
use std::fmt;

use data::two_dim::vector::Vector;

/// Coord that uses the dimensional axes as internal storage
#[deriving(Clone)]
pub struct Coord<T>{
	/// The horizontal axis
	pub x: T,
	
	/// The vertical axis
	pub y: T
}

impl<T: Float + FloatMath + Mul<T,T> + Div<T,T> + num::Zero> Vector<T> for Coord<T>{//TODO: Mul and Div are only required for `dot_product`, `unit` and `project`. Separate if that will be implemented in rustc
	fn from_vector<V: Vector<T>>(v: &V) -> Coord<T>{
		 Coord{x: v.x(),y: v.y()}
	}

	#[inline(always)]
	fn x(&self) -> T{self.x}
	
	#[inline(always)]
	fn y(&self) -> T{self.y}

	#[inline(always)]
	fn magnitude(&self) -> T{
		self.x.hypot(self.y)
	}

	#[inline(always)]
	fn direction(&self) -> T{
		self.y.atan2(self.x)
	}

	fn dot_product<V: Vector<T>>(&self,other: &V) -> T{
		self.x * other.x() + self.y * other.y()
	}

	#[inline(always)]
	fn unit(&self) -> Coord<T>{
		*self / self.magnitude()
	}

	fn project<V: Vector<T>>(&self,other: &V) -> V{
		(*other) * (self.dot_product(other) / other.dot_product(other))
	}

	#[inline(always)]
	fn set_x(&mut self,x: T){
		self.x = x;
	}

	#[inline(always)]
	fn set_y(&mut self,y: T){
		self.y = y;
	}

	fn set_magnitude(&mut self,magnitude: T){
		let ratio = magnitude/self.magnitude();
		self.x = self.x * ratio;
		self.y = self.y * ratio;
	}

	fn set_direction(&mut self,direction: T){
		let magnitude = self.magnitude();
		self.x = magnitude*direction.cos();
		self.y = magnitude*direction.sin();
	}
}

impl<T: Float> Coord<T>{
	pub fn limit_magnitude(&mut self,magnitude: T){
		let current_magnitude = self.x*self.x + self.y*self.y;
		if current_magnitude > magnitude*magnitude{
			let d = magnitude/current_magnitude.sqrt();
			self.x = self.x*d;
			self.y = self.y*d;
		}
	}
}

impl<T: fmt::Show> fmt::Show for Coord<T>{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		write!(f,"Coord({},{})",self.x,self.y)
	}
}

impl<T: Add<T,T>> Add<Coord<T>,Coord<T>> for Coord<T>{
	fn add(&self, other: &Coord<T>) -> Coord<T>{
		 Coord{x: self.x + other.x,y: self.y + other.y}
	}
}

impl<T: Sub<T,T>> Sub<Coord<T>,Coord<T>> for Coord<T>{
	fn sub(&self, other: &Coord<T>) -> Coord<T>{
		 Coord{x: self.x - other.x,y: self.y - other.y}
	}
}

impl<T: Mul<T,T>> Mul<T,Coord<T>> for Coord<T>{
	fn mul(&self, other: &T) -> Coord<T>{
		 Coord{x: self.x * *other,y: self.y * *other}
	}
}

impl<T: Div<T,T>> Div<T,Coord<T>> for Coord<T>{
	fn div(&self, other: &T) -> Coord<T>{
		 Coord{x: self.x / *other,y: self.y / *other}
	}
}

impl<T: Neg<T>> Neg<Coord<T>> for Coord<T>{
	fn neg(&self) -> Coord<T>{
		 Coord{x: -self.x,y: -self.y}
	}
}

impl<T: num::Zero> num::Zero for Coord<T>{
	fn zero() -> Coord<T>{
		Coord{x: num::Zero::zero(),y: num::Zero::zero()}
	}

	fn is_zero(&self) -> bool{
		self.x.is_zero() && self.y.is_zero()
	}
}

impl<T: num::Signed> Coord<T>{
	pub fn abs(&self) -> Coord<T>{
		 Coord{x: self.x.abs(),y: self.y.abs()}
	}
}

#[test]
fn vector_limit1(){
	let mut v = Coord{x: 3.0 as f32,y: 4.0};
	v.limit_magnitude(2.5);
	let m = v.magnitude();
	println!("{}.magnitude = {} = 2.5",v,m);
	assert!(m==2.5 || (2.5-m).abs() < 1.0e-06);
}

#[test]
fn vector_limit2(){
	let mut v = Coord{x: 100.0 as f32,y: 100.0};
	v.limit_magnitude(10.0);
	let m = v.magnitude();
	println!("{}.magnitude = {} = 10.0",v,m);
	assert!(m==10.0 || (10.0-m).abs() < 1.0e-06);
}
