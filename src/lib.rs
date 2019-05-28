mod hermite;
use hermite::{h_5, h_5p, h_5pp};

trait Position {}

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
	fn acceleration_at(&self, t: f64) -> Option<V>;
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

		assert!(0.0_f64 <= t && t <= 1.0_f64, "{} not in [0., 1.]", t);

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

	fn acceleration_at(&self, t: f64) -> Option<V> {
		let anchors = self.get_segment(t);

		if let Some(Segment(t, prec, succ)) = anchors {
			let p0 = &prec.position;
			let v0 = &prec.velocity;
			let a0 = &prec.acceleration;

			let p1 = &succ.position;
			let v1 = &succ.velocity;
			let a1 = &succ.acceleration;

			let h05pp = h_5pp(t, 0);
			let h15pp = h_5pp(t, 1);
			let h25pp = h_5pp(t, 2);
			let h35pp = h_5pp(t, 3);
			let h45pp = h_5pp(t, 4);
			let h55pp = h_5pp(t, 5);

			return Some(
				(*p0 * h05pp)
					+ (*v0 * h15pp)
					+ (*a0 * h25pp)
					+ (*a1 * h35pp)
					+ (*v1 * h45pp)
					+ (*p1 * h55pp),
			);
		}

		None
	}
}

#[cfg(test)]
mod tests;
