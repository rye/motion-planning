trait Position {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3d<V>(pub V, pub V, pub V);

impl<T, V> std::ops::Mul<T> for Vec3d<V>
where
	T: std::ops::Mul<T, Output = T>,
	T: std::convert::From<V>,
	T: Copy,
{
	type Output = Vec3d<T>;

	fn mul(self, scalar: T) -> Vec3d<T> {
		Vec3d(
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
	type Output = Vec3d<T>;

	fn add(self, other: Vec3d<T>) -> Vec3d<T> {
		Vec3d {
			0: (self.0 + other.0).into(),
			1: (self.1 + other.1).into(),
			2: (self.2 + other.2).into(),
		}
	}
}

pub struct Pose<V> {
	pub position: V,
	pub velocity: V,
	pub acceleration: V,
}

pub trait Path<V> {
	fn position_at(&self, t: f64) -> Option<V>;
	fn velocity_at(&self, t: f64) -> Option<V>;
}

impl<V> Path<V> for std::vec::Vec<Pose<V>>
where
	V: Copy,
	V: std::ops::Mul<f64, Output = V>,
	V: std::ops::Add<V, Output = V>,
{
	fn position_at(&self, t: f64) -> Option<V> {
		let length = self.len();

		if let 0 = length {
			return None;
		}

		let prec_idx = (t / length as f64).floor() as usize;
		let succ_idx = (t / length as f64).ceil() as usize;

		let prec: &Pose<V> = &self[prec_idx];
		let succ: &Pose<V> = &self[succ_idx];

		let p0 = prec.position;
		let v0 = prec.velocity;
		let a0 = prec.acceleration;

		let p1 = succ.position;
		let v1 = succ.velocity;
		let a1 = succ.acceleration;

		let t1 = t;
		let t2 = t.powi(2);
		let t3 = t.powi(3);
		let t4 = t.powi(4);
		let t5 = t.powi(5);

		let h05 = 1. - 10. * t3 + 15. * t4 - 6. * t5;
		let h15 = t1 - 6. * t3 + 8. * t4 - 3. * t5;
		let h25 = 0.5 * t2 - 1.5 * t3 + 1.5 * t4 - 0.5 * t5;
		let h35 = 0.5 * t3 - t4 + 0.5 * t5;
		let h45 = -4. * t3 + 7. * t4 - 3. * t5;
		let h55 = 10. * t3 - 15. * t4 + 6. * t5;

		Some((p0 * h05) + (v0 * h15) + (a0 * h25) + (a1 * h35) + (v1 * h45) + (p1 * h55))
	}

	fn velocity_at(&self, t: f64) -> Option<V> {
		let length = self.len();

		if let 0 = length {
			return None;
		}

		let prec_idx = (t / length as f64).floor() as usize;
		let succ_idx = (t / length as f64).ceil() as usize;

		let prec: &Pose<V> = &self[prec_idx];
		let succ: &Pose<V> = &self[succ_idx];

		let p0 = prec.position;
		let v0 = prec.velocity;
		let a0 = prec.acceleration;

		let p1 = succ.position;
		let v1 = succ.velocity;
		let a1 = succ.acceleration;

		let t2 = t.powi(2);
		let t3 = t.powi(3);
		let t4 = t.powi(4);

		let h05p = -30. * t2 + 60. * t3 - 30. * t4;
		let h15p = 1. - 18. * t2 + 32. * t3 - 15. * t4;
		let h25p = t - 4.5 * t2 + 6. * t3 - 2.5 * t4;
		let h35p = 1.5 * t2 - 4. * t3 + 2.5 * t4;
		let h45p = -12. * t2 + 28. * t3 - 15. * t4;
		let h55p = 30. * t2 - 60. * t3 + 30. * t4;

		Some((p0 * h05p) + (v0 * h15p) + (a0 * h25p) + (a1 * h35p) + (v1 * h45p) + (p1 * h55p))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn vec_addition() {
		let a: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);
		let b: Vec3d<f32> = Vec3d(5.0, 4.0, 3.0);

		assert_eq!(a + b, Vec3d(6., 6., 6.));
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
	fn position_correct_straight_line() {
		let mut segment = Vec::new();

		segment.push(Pose {
			position: Vec3d(0.0f64, 0.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		segment.push(Pose {
			position: Vec3d(0.0f64, 1.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		assert_eq!(segment.position_at(0.0), Some(segment[0].position));
		assert_eq!(segment.position_at(0.5), Some(Vec3d(0.0, 0.5, 0.0)));
		assert_eq!(segment.position_at(1.0), Some(segment[1].position));
	}

	#[test]
	fn position_length_zero() {
		let segment: Vec<Pose<Vec3d<f64>>> = Vec::new();

		assert_eq!(segment.position_at(0.0), None);
	}

	#[test]
	fn velocity_correct_straight_line_opp_starts() {
		let mut segment = Vec::new();

		segment.push(Pose {
			position: Vec3d(0.0f64, 0.0, 0.0),
			velocity: Vec3d(0.0, 1.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		segment.push(Pose {
			position: Vec3d(0.0f64, 1.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		assert_eq!(segment.velocity_at(0.0), Some(segment[0].velocity));
		assert_eq!(segment.velocity_at(0.5), Some(Vec3d(0.0, 1.4375, 0.0)));
		assert_eq!(segment.velocity_at(1.0), Some(segment[1].velocity));
	}

	#[test]
	fn velocity_length_zero() {
		let segment: Vec<Pose<Vec3d<f64>>> = Vec::new();

		assert_eq!(segment.velocity_at(0.0), None);
	}
}
