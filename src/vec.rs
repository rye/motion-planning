use core::ops::{Add, Mul, Neg};
use core::{convert::From, fmt};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3d<V>(pub V, pub V, pub V);

impl<V> fmt::Display for Vec3d<V>
where
	V: fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({},{},{})", self.0, self.1, self.2)
	}
}

impl<V> Neg for Vec3d<V>
where
	V: From<V> + Neg<Output = V>,
{
	type Output = Self;

	fn neg(self) -> Self::Output {
		Vec3d(-self.0, -self.1, -self.2)
	}
}

impl<T, V> Mul<T> for Vec3d<V>
where
	T: Copy + From<V> + Mul<T, Output = T>,
{
	type Output = Vec3d<T>;

	fn mul(self, scalar: T) -> Self::Output {
		Vec3d(
			scalar * self.0.into(),
			scalar * self.1.into(),
			scalar * self.2.into(),
		)
	}
}

impl<T: Add<T>> Add<Vec3d<T>> for Vec3d<T>
where
	T: From<<T as Add>::Output>,
{
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Vec3d(
			(self.0 + other.0).into(),
			(self.1 + other.1).into(),
			(self.2 + other.2).into(),
		)
	}
}

impl<V> crate::Dot for Vec3d<V>
where
	V: Add<V, Output = V> + Copy + Mul<V, Output = V>,
{
	type Output = V;
	fn dot(&self, other: &Self) -> Self::Output {
		(self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
	}
}

#[cfg(test)]
mod add {
	use super::Vec3d;

	#[test]
	fn it_works() {
		let a: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);
		let b: Vec3d<f32> = Vec3d(5.0, 4.0, 3.0);

		assert_eq!(a + b, Vec3d(6., 6., 6.));
	}
}

#[cfg(test)]
mod mul {
	use super::Vec3d;

	#[cfg(test)]
	mod scalar {
		use super::Vec3d;

		#[test]
		fn smul_f64() {
			let a: Vec3d<f64> = Vec3d(1.0, 2.0, 3.0);
			let b: f64 = 2.0;

			assert_eq!(a * b, Vec3d(2.0f64, 4.0f64, 6.0f64));
		}

		#[test]
		fn smul_f32() {
			let a: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);
			let b: f32 = 2.0;

			assert_eq!(a * b, Vec3d(2.0f32, 4.0f32, 6.0f32));
		}

		#[test]
		fn smul_v_f32_s_f64() {
			let a: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);
			let b: f64 = 2.0;

			assert_eq!(a * b, Vec3d(2.0f64, 4.0f64, 6.0f64));
		}
	}
}

#[cfg(test)]
mod neg {
	use super::Vec3d;

	#[test]
	fn it_works() {
		let vec: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);

		assert_eq!(-vec, Vec3d(-1.0, -2.0, -3.0));
	}
}

#[cfg(test)]
mod display {
	use super::Vec3d;

	#[test]
	fn it_works() {
		let v: Vec3d<f32> = Vec3d(0., 1.25, 4.);

		assert_eq!(format!("{}", v), "(0,1.25,4)");
	}
}

#[cfg(test)]
mod dot {
	use super::Vec3d;
	use crate::{assert_f64_roughly_eq, Dot};

	#[test]
	fn it_works() {
		let a: Vec3d<f64> = Vec3d(1., 2., 3.);
		let b: Vec3d<f64> = Vec3d(5., 4., 3.);

		assert_f64_roughly_eq!(a.dot(&b), 22.0_f64);
	}
}
