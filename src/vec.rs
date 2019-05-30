#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3d<V>(pub V, pub V, pub V);

impl<V> std::fmt::Display for Vec3d<V>
where
	V: std::fmt::Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({},{},{})", self.0, self.1, self.2)
	}
}

impl<V> std::ops::Neg for Vec3d<V>
where
	V: std::convert::From<V>,
	V: std::ops::Neg<Output = V>,
{
	type Output = Self;

	fn neg(self) -> Self {
		Self(-self.0, -self.1, -self.2)
	}
}

impl<T, V> std::ops::Mul<T> for Vec3d<V>
where
	T: std::ops::Mul<T, Output = T>,
	T: std::convert::From<V>,
	T: Copy,
{
	type Output = Vec3d<T>;

	fn mul(self, scalar: T) -> Vec3d<T> {
		Self(
			scalar * self.0.into(),
			scalar * self.1.into(),
			scalar * self.2.into(),
		)
	}
}

impl<T: std::ops::Add<T>> std::ops::Add<Vec3d<T>> for Vec3d<T>
where
	T: std::convert::From<<T as std::ops::Add>::Output>,
{
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self(
			(self.0 + other.0).into(),
			(self.1 + other.1).into(),
			(self.2 + other.2).into(),
		)
	}
}

impl<V> Vec3d<V>
where
	V: std::ops::Mul<V, Output = V>,
	V: std::ops::Add<V, Output = V>,
	V: Copy,
{
	pub fn dot(&self, other: &Vec3d<V>) -> V {
		(self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
	}
}

#[test]
fn vec_addition() {
	let a: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);
	let b: Vec3d<f32> = Vec3d(5.0, 4.0, 3.0);

	assert_eq!(a + b, Vec3d(6., 6., 6.));
}

#[test]
fn vec_display() {
	let v: Vec3d<f32> = Vec3d(0., 1.25, 4.);

	assert_eq!(format!("{}", v), "(0,1.25,4)");
}

#[test]
fn vec_negation() {
	let vec: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);

	assert_eq!(-vec, Vec3d(-1.0, -2.0, -3.0));
}

#[test]
fn vec_scalar_multiplication_f64() {
	let a: Vec3d<f64> = Vec3d(1.0, 2.0, 3.0);
	let b: f64 = 2.0;

	assert_eq!(a * b, Vec3d(2.0f64, 4.0f64, 6.0f64));
}

#[test]
fn vec_scalar_multiplication_f32() {
	let a: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);
	let b: f32 = 2.0;

	assert_eq!(a * b, Vec3d(2.0f32, 4.0f32, 6.0f32));
}

#[test]
fn vec_scalar_multiplication_vf32_sf64() {
	let a: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);
	let b: f64 = 2.0;

	assert_eq!(a * b, Vec3d(2.0f64, 4.0f64, 6.0f64));
}

#[test]
fn vec_dot() {
	let a: Vec3d<f64> = Vec3d(1., 2., 3.);
	let b: Vec3d<f64> = Vec3d(5., 4., 3.);

	assert_eq!(a.dot(&b), 22.0_f64);
}
