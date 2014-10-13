extern crate std;
extern crate core;

#[deriving(Clone,Zero)]
pub struct Vector2<T>(pub T,pub T);

impl<T> Vector2<T>{
	pub fn new(x: T,y: T) -> Vector2<T>{
		return Vector2(x,y);
	}
}

impl<T: std::fmt::Show> std::fmt::Show for Vector2<T>{
	fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
		return write!(f,"vec({},{})",self.0,self.1);
	}
}

impl<T: Add<T,T>> Add<Vector2<T>,Vector2<T>> for Vector2<T>{
	fn add(&self, other: &Vector2<T>) -> Vector2<T>{
		return Vector2(self.0 + other.0,self.1 + other.1);
	}
}

impl<T: Sub<T,T>> Sub<Vector2<T>,Vector2<T>> for Vector2<T>{
	fn sub(&self, other: &Vector2<T>) -> Vector2<T>{
		return Vector2(self.0 - other.0,self.1 - other.1);
	}
}

impl<T: Mul<T,T>> Mul<T,Vector2<T>> for Vector2<T>{
	fn mul(&self, other: &T) -> Vector2<T>{
		return Vector2(self.0 * *other,self.1 * *other);
	}
}
/*
impl<T: Mul<T,T> + Add<T,T>> Mul<Vector2<T>,T> for Vector2<T>{
	fn mul(&self, other: &Vector2<T>) -> T{
		return self.0 * other.0 + self.1 * other.1;
	}
}
*/
impl<T: Div<T,T>> Div<T,Vector2<T>> for Vector2<T>{
	fn div(&self, other: &T) -> Vector2<T>{
		return Vector2(self.0 / *other,self.1 / *other);
	}
}
