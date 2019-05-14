trait Position {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3d<V>(pub V, pub V, pub V);

impl<V> std::ops::Neg for Vec3d<V>
where
	V: std::convert::From<V>,
	V: std::ops::Neg<Output = V>,
{
	type Output = Vec3d<V>;

	fn neg(self) -> Vec3d<V> {
		Vec3d(-self.0, -self.1, -self.2)
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

fn h_5(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);
	let t4 = t.powi(4);
	let t5 = t.powi(5);

	match n {
		0 => 1. - 10. * t3 + 15. * t4 - 6. * t5,
		1 => t - 6. * t3 + 8. * t4 - 3. * t5,
		2 => 0.5 * t2 - 1.5 * t3 + 1.5 * t4 - 0.5 * t5,
		3 => 0.5 * t3 - t4 + 0.5 * t5,
		4 => -4. * t3 + 7. * t4 - 3. * t5,
		5 => 10. * t3 - 15. * t4 + 6. * t5,
		_ => unimplemented!(),
	}
}

fn h_5p(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);
	let t4 = t.powi(4);

	match n {
		0 => -30. * t2 + 60. * t3 - 30. * t4,
		1 => 1. - 18. * t2 + 32. * t3 - 15. * t4,
		2 => t - 4.5 * t2 + 6. * t3 - 2.5 * t4,
		3 => 1.5 * t2 - 4. * t3 + 2.5 * t4,
		4 => -12. * t2 + 28. * t3 - 15. * t4,
		5 => 30. * t2 - 60. * t3 + 30. * t4,
		_ => unimplemented!(),
	}
}

#[derive(Debug, PartialEq)]
pub struct Pose<V> {
	pub position: V,
	pub velocity: V,
	pub acceleration: V,
}

#[derive(Debug, PartialEq)]
pub struct Segment<'a, V>(f64, &'a Pose<V>, &'a Pose<V>);

pub trait Trajectory<V> {
	fn get_segment(&self, t: f64) -> Option<Segment<V>>;
	fn position_at(&self, t: f64) -> Option<V>;
	fn velocity_at(&self, t: f64) -> Option<V>;
}

impl<V> Trajectory<V> for std::vec::Vec<Pose<V>>
where
	V: Copy,
	V: std::ops::Mul<f64, Output = V>,
	V: std::ops::Add<V, Output = V>,
{
	fn get_segment(&self, t: f64) -> Option<Segment<V>> {
		let length = self.len();

		// If our container (Vec) has length 0, we cannot find a segment!.
		if let 0 = length {
			return None;
		}

		// `t` ranges from `0.` to `length * 1.`;

		let prec_idx = t.floor() as usize;
		let succ_idx = t.ceil() as usize;

		let prec: &Pose<V> = &self[prec_idx];
		let succ: &Pose<V> = &self[succ_idx];

		let t = t.fract();

		assert!(0.0f64 <= t && t <= 1.0f64, "{} not in [0., 1.]", t);

		Some(Segment(t, prec, succ))
	}

	fn position_at(&self, t: f64) -> Option<V> {
		let anchors = self.get_segment(t);

		if let Some(Segment(t, prec, succ)) = anchors {
			let p0 = &prec.position;
			let v0 = &prec.velocity;
			let a0 = &prec.acceleration;

			let p1 = &succ.position;
			let v1 = &succ.velocity;
			let a1 = &succ.acceleration;

			let h05 = h_5(t, 0);
			let h15 = h_5(t, 1);
			let h25 = h_5(t, 1);
			let h35 = h_5(t, 3);
			let h45 = h_5(t, 4);
			let h55 = h_5(t, 5);

			return Some(
				(*p0 * h05) + (*v0 * h15) + (*a0 * h25) + (*a1 * h35) + (*v1 * h45) + (*p1 * h55),
			);
		}

		None
	}

	fn velocity_at(&self, t: f64) -> Option<V> {
		let anchors = self.get_segment(t);

		if let Some(Segment(t, prec, succ)) = anchors {
			let p0 = &prec.position;
			let v0 = &prec.velocity;
			let a0 = &prec.acceleration;

			let p1 = &succ.position;
			let v1 = &succ.velocity;
			let a1 = &succ.acceleration;

			let h05p = h_5p(t, 0);
			let h15p = h_5p(t, 1);
			let h25p = h_5p(t, 2);
			let h35p = h_5p(t, 3);
			let h45p = h_5p(t, 4);
			let h55p = h_5p(t, 5);

			return Some(
				(*p0 * h05p) + (*v0 * h15p) + (*a0 * h25p) + (*a1 * h35p) + (*v1 * h45p) + (*p1 * h55p),
			);
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_segment_returns_correct_segment() {
		let mut segment = Vec::new();

		// Start at origin, moving north at 1.0 u/s
		segment.push(Pose {
			position: Vec3d(0.0f64, 0.0, 0.0),
			velocity: Vec3d(0.0, 1.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		// Stop at (0,1,0)
		segment.push(Pose {
			position: Vec3d(0.0f64, 1.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		// Accelerate through (0,2,0), moving north at 1.0 u/s
		segment.push(Pose {
			position: Vec3d(0.0f64, 2.0, 0.0),
			velocity: Vec3d(0.0, 1.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		let subdiv = 4.;

		for t in 0..=(2 * subdiv as usize) {
			let t = t as f64 / subdiv;

			if t.fract() == 0. {
				assert_eq!(
					segment.get_segment(t),
					Some(Segment(
						t.fract(),
						&segment[t as usize],
						&segment[t as usize]
					))
				);
			} else {
				assert_eq!(
					segment.get_segment(t),
					Some(Segment(
						t.fract(),
						&segment[t as usize],
						&segment[(t + 1.) as usize]
					))
				);
			}
		}
	}

	#[test]
	fn get_segment_returns_none_on_zero_length_vec() {
		let segment: Vec<Pose<Vec3d<f64>>> = Vec::new();

		assert_eq!(segment.get_segment(0.5), None);
	}

	#[test]
	fn vec_addition() {
		let a: Vec3d<f32> = Vec3d(1.0, 2.0, 3.0);
		let b: Vec3d<f32> = Vec3d(5.0, 4.0, 3.0);

		assert_eq!(a + b, Vec3d(6., 6., 6.));
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
	fn velocity_length_zero() {
		let segment: Vec<Pose<Vec3d<f64>>> = Vec::new();

		assert_eq!(segment.velocity_at(0.0), None);
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
	fn h_5_correctness() {
		assert_eq!(h_5(0.0, 0), 1.);
		assert_eq!(h_5(0.0, 1), 0.);
		assert_eq!(h_5(0.0, 2), 0.);
		assert_eq!(h_5(0.0, 3), 0.);
		assert_eq!(h_5(0.0, 4), 0.);
		assert_eq!(h_5(0.0, 5), 0.);

		assert_eq!(h_5(1.0, 0), 0.);
		assert_eq!(h_5(1.0, 1), 0.);
		assert_eq!(h_5(1.0, 2), 0.);
		assert_eq!(h_5(1.0, 3), 0.);
		assert_eq!(h_5(1.0, 4), 0.);
		assert_eq!(h_5(1.0, 5), 1.);
	}

	#[test]
	fn h_5p_correctness() {
		assert_eq!(h_5p(0.0, 0), 0.);
		assert_eq!(h_5p(0.0, 1), 1.);
		assert_eq!(h_5p(0.0, 2), 0.);
		assert_eq!(h_5p(0.0, 3), 0.);
		assert_eq!(h_5p(0.0, 4), 0.);
		assert_eq!(h_5p(0.0, 5), 0.);

		assert_eq!(h_5p(1.0, 0), 0.);
		assert_eq!(h_5p(1.0, 1), 0.);
		assert_eq!(h_5p(1.0, 2), 0.);
		assert_eq!(h_5p(1.0, 3), 0.);
		assert_eq!(h_5p(1.0, 4), 1.);
		assert_eq!(h_5p(1.0, 5), 0.);
	}

	#[test]
	fn all_set_points_hit() {
		let mut segment = Vec::new();

		// Start at origin, moving north at 1.0 u/s
		segment.push(Pose {
			position: Vec3d(0.0f64, 0.0, 0.0),
			velocity: Vec3d(0.0, 1.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		// Stop at (0,1,0)
		segment.push(Pose {
			position: Vec3d(0.0f64, 1.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		// Accelerate through (0,2,0), moving north at 1.0 u/s
		segment.push(Pose {
			position: Vec3d(0.0f64, 2.0, 0.0),
			velocity: Vec3d(0.0, 1.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		});

		assert_eq!(segment.position_at(0.0), Some(segment[0].position));
		assert_eq!(segment.velocity_at(0.0), Some(segment[0].velocity));
		assert_eq!(segment.position_at(1.0), Some(segment[1].position));
		assert_eq!(segment.velocity_at(1.0), Some(segment[1].velocity));
		assert_eq!(segment.position_at(2.0), Some(segment[2].position));
		assert_eq!(segment.velocity_at(2.0), Some(segment[2].velocity));
	}
}
