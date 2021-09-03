use core::ops::{Add, Mul};
use std::vec::Vec;

pub mod hermite;
use hermite::{
	cubic::{h_3, h_3p},
	quintic::{h_5, h_5p, h_5pp},
};

pub mod vec;
pub mod vecn;

trait Dot {
	type Output;
	fn dot(&self, other: &Self) -> Self::Output;
}

#[derive(Debug, PartialEq)]
pub struct Pose3<V> {
	pub position: V,
	pub velocity: V,
	pub acceleration: V,
}

#[derive(Debug, PartialEq)]
pub struct Pose2<V> {
	pub position: V,
	pub velocity: V,
}

#[derive(Debug, PartialEq)]
pub struct Segment2<'a, V>(f64, &'a Pose2<V>, &'a Pose2<V>);

pub trait Trajectory2<V> {
	fn get_segment(&self, t: f64) -> Option<Segment2<V>>;
	fn position_at(&self, t: f64) -> Option<V>;
	fn velocity_at(&self, t: f64) -> Option<V>;
}

impl<V> Trajectory2<V> for Vec<Pose2<V>>
where
	V: Add<V, Output = V> + Copy + Mul<f64, Output = V>,
{
	fn get_segment(&self, t: f64) -> Option<Segment2<V>> {
		let length = self.len();

		// If our container (Vec) has length 0, we cannot find a segment!.
		if let 0 = length {
			return None;
		}

		// `t` ranges from `0.` to `length * 1.`;

		let prec_idx = t.floor() as usize;
		let succ_idx = t.ceil() as usize;

		let prec: &Pose2<V> = &self[prec_idx];
		let succ: &Pose2<V> = &self[succ_idx];

		let t = t.fract();

		assert!((0.0_f64..=1.0_f64).contains(&t), "{} not in [0., 1.]", t);

		Some(Segment2(t, prec, succ))
	}

	fn position_at(&self, t: f64) -> Option<V> {
		let anchors = self.get_segment(t);

		if let Some(Segment2(t, prec, succ)) = anchors {
			let p0 = &prec.position;
			let v0 = &prec.velocity;

			let p1 = &succ.position;
			let v1 = &succ.velocity;

			let h03 = h_3(t, 0);
			let h13 = h_3(t, 1);
			let h23 = h_3(t, 2);
			let h33 = h_3(t, 3);

			return Some((*p0 * h03) + (*v0 * h13) + (*v1 * h23) + (*p1 * h33));
		}

		None
	}

	fn velocity_at(&self, t: f64) -> Option<V> {
		let anchors = self.get_segment(t);

		if let Some(Segment2(t, prec, succ)) = anchors {
			let p0 = &prec.position;
			let v0 = &prec.velocity;

			let p1 = &succ.position;
			let v1 = &succ.velocity;

			let h03p = h_3p(t, 0);
			let h13p = h_3p(t, 1);
			let h23p = h_3p(t, 2);
			let h33p = h_3p(t, 3);

			return Some((*p0 * h03p) + (*v0 * h13p) + (*v1 * h23p) + (*p1 * h33p));
		}

		None
	}
}

#[derive(Debug, PartialEq)]
pub struct Segment3<'a, V>(f64, &'a Pose3<V>, &'a Pose3<V>);

pub trait Trajectory3<V> {
	fn get_segment(&self, t: f64) -> Option<Segment3<V>>;
	fn position_at(&self, t: f64) -> Option<V>;
	fn velocity_at(&self, t: f64) -> Option<V>;
	fn acceleration_at(&self, t: f64) -> Option<V>;
}

impl<V> Trajectory3<V> for Vec<Pose3<V>>
where
	V: Add<V, Output = V> + Copy + Mul<f64, Output = V>,
{
	fn get_segment(&self, t: f64) -> Option<Segment3<V>> {
		let length = self.len();

		// If our container (Vec) has length 0, we cannot find a segment!.
		if let 0 = length {
			return None;
		}

		// `t` ranges from `0.` to `length * 1.`;

		let prec_idx = t.floor() as usize;
		let succ_idx = t.ceil() as usize;

		let prec: &Pose3<V> = &self[prec_idx];
		let succ: &Pose3<V> = &self[succ_idx];

		let t = t.fract();

		assert!((0.0_f64..=1.0_f64).contains(&t), "{} not in [0., 1.]", t);

		Some(Segment3(t, prec, succ))
	}

	fn position_at(&self, t: f64) -> Option<V> {
		let anchors = self.get_segment(t);

		if let Some(Segment3(t, prec, succ)) = anchors {
			let p0 = &prec.position;
			let v0 = &prec.velocity;
			let a0 = &prec.acceleration;

			let p1 = &succ.position;
			let v1 = &succ.velocity;
			let a1 = &succ.acceleration;

			let h05 = h_5(t, 0);
			let h15 = h_5(t, 1);
			let h25 = h_5(t, 2);
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

		if let Some(Segment3(t, prec, succ)) = anchors {
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

		if let Some(Segment3(t, prec, succ)) = anchors {
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

#[cfg(test)]
#[macro_export]
macro_rules! assert_f64_roughly_eq {
	($left:expr, $right:expr) => {
		assert!(($right - $left).abs() < f64::EPSILON)
	};
}
