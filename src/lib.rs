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
mod tests;
