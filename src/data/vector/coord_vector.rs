use std::{fmt,num};

use data::vector::Vector2;

#[deriving(Clone,Zero)]
pub struct CoordVector2<T>{
	pub x: T,
	pub y: T
}

impl<T: FloatMath + Mul<T,T> + Div<T,T>> Vector2<T> for CoordVector2<T>{//TODO: Mul and Div are only required for `dot_product`, `unit` and `project`. Separate if that will be implemented in rustc
	fn from_generic<V: Vector2<T>>(v: &V) -> CoordVector2<T>{
		return CoordVector2{x: v.x(),y: v.y()};
	}

	#[inline(always)]
	fn x(&self) -> T{self.x}
	
	#[inline(always)]
	fn y(&self) -> T{self.y}

	fn magnitude(&self) -> T where T: FloatMath{
		self.x.hypot(self.y)
	}

	fn direction(&self) -> T{
		self.y.atan2(self.x)
	}

	fn dot_product<V: Vector2<T>>(&self,other: &V) -> T{
		self.x * other.x() + self.y * other.y()
	}

	fn unit(&self) -> CoordVector2<T>{
		*self / self.magnitude()
	}

	fn project<V: Vector2<T>>(&self,other: &V) -> V{
		(*other) * (self.dot_product(other) / other.dot_product(other))
	}
}

impl<T: FloatMath> CoordVector2<T>{
	pub fn limit_magnitude(&mut self,magnitude: T){
		let current_magnitude = self.x*self.x + self.y*self.y;
		if current_magnitude > magnitude*magnitude{
			let d = magnitude/current_magnitude.sqrt();
			self.x = self.x*d;
			self.y = self.y*d;
		}
	}
}

impl<T: fmt::Show> fmt::Show for CoordVector2<T>{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		return write!(f,"vec({},{})",self.x,self.y);
	}
}

impl<T: Add<T,T>> Add<CoordVector2<T>,CoordVector2<T>> for CoordVector2<T>{
	fn add(&self, other: &CoordVector2<T>) -> CoordVector2<T>{
		return CoordVector2{x: self.x + other.x,y: self.y + other.y};
	}
}

impl<T: Sub<T,T>> Sub<CoordVector2<T>,CoordVector2<T>> for CoordVector2<T>{
	fn sub(&self, other: &CoordVector2<T>) -> CoordVector2<T>{
		return CoordVector2{x: self.x - other.x,y: self.y - other.y};
	}
}

impl<T: Mul<T,T>> Mul<T,CoordVector2<T>> for CoordVector2<T>{
	fn mul(&self, other: &T) -> CoordVector2<T>{
		return CoordVector2{x: self.x * *other,y: self.y * *other};
	}
}

impl<T: Div<T,T>> Div<T,CoordVector2<T>> for CoordVector2<T>{
	fn div(&self, other: &T) -> CoordVector2<T>{
		return CoordVector2{x: self.x / *other,y: self.y / *other};
	}
}

impl<T: Neg<T>> Neg<CoordVector2<T>> for CoordVector2<T>{
	fn neg(&self) -> CoordVector2<T>{
		return CoordVector2{x: -self.x,y: -self.y};
	}
}

impl<T: num::One> /*num::One for*/ CoordVector2<T>{
	pub fn one() -> CoordVector2<T>{
		CoordVector2{x: num::One::one(),y: num::One::one()}
	}
}

impl<T: Signed> CoordVector2<T>{
	pub fn abs(&self) -> CoordVector2<T>{
		return CoordVector2{x: self.x.abs(),y: self.y.abs()};
	}
}

#[test]
fn vector_limit1(){
	let mut v = CoordVector2{x: 3.0 as f32,y: 4.0};
	v.limit_magnitude(2.5);
	let m = v.magnitude();
	println!("{}.magnitude = {} = 2.5",v,m);
	assert!(m==2.5 || (2.5-m).abs() < 1.0e-06);
}

#[test]
fn vector_limit2(){
	let mut v = CoordVector2{x: 100.0 as f32,y: 100.0};
	v.limit_magnitude(10.0);
	let m = v.magnitude();
	println!("{}.magnitude = {} = 10.0",v,m);
	assert!(m==10.0 || (10.0-m).abs() < 1.0e-06);
}
