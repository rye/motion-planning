trait Position {}

#[derive(Debug, PartialEq)]
struct Vec3d<V> {
	x: V,
	y: V,
	z: V,
}

impl<V> std::convert::From<(V, V, V)> for Vec3d<V> {
	fn from(xyz: (V, V, V)) -> Self {
		Vec3d {
			x: xyz.0,
			y: xyz.1,
			z: xyz.2,
		}
	}
}

impl<T: std::ops::Mul<T> + Copy> std::ops::Mul<T> for &Vec3d<T>
where
	T: std::convert::From<<T as std::ops::Mul>::Output>,
{
	type Output = Vec3d<T>;

	fn mul(self, scalar: T) -> Vec3d<T> {
		Vec3d {
			x: (self.x * scalar).into(),
			y: (self.y * scalar).into(),
			z: (self.z * scalar).into(),
		}
	}
}

impl<T: std::ops::Add<T>> std::ops::Add<Vec3d<T>> for Vec3d<T>
where
	T: std::convert::From<<T as std::ops::Add>::Output>,
{
	type Output = Vec3d<T>;

	fn add(self, other: Vec3d<T>) -> Vec3d<T> {
		Vec3d {
			x: (self.x + other.x).into(),
			y: (self.y + other.y).into(),
			z: (self.z + other.z).into(),
		}
	}
}

struct Pose<V> {
	position: V,
	velocity: V,
	acceleration: V,
}

struct Segment<V> {
	start: V,
	end: V,
}

impl Segment<Pose<Vec3d<f64>>> {
	fn position_at(&self, t: f64) -> Vec3d<f64> {
		let p0 = &self.start.position;
		let v0 = &self.start.velocity;
		let a0 = &self.start.acceleration;

		let p1 = &self.end.position;
		let v1 = &self.end.velocity;
		let a1 = &self.end.acceleration;

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

		(p0 * h05) + (v0 * h15) + (a0 * h25) + (a1 * h35) + (v1 * h45) + (p1 * h55)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hermite_works() {
		let segment: Segment<Pose<Vec3d<f64>>> = Segment {
			start: Pose {
				position: (0.0, 0.0, 0.0).into(),
				velocity: (0.0, 0.0, 0.0).into(),
				acceleration: (0.0, 0.0, 0.0).into(),
			},
			end: Pose {
				position: (0.0, 1.0, 0.0).into(),
				velocity: (0.0, 0.0, 0.0).into(),
				acceleration: (0.0, 0.0, 0.0).into(),
			},
		};

		assert_eq!(segment.position_at(0.0), segment.start.position);
		assert_eq!(segment.position_at(0.25), (0.0, 0.103515625, 0.0).into());
		assert_eq!(segment.position_at(0.5), (0.0, 0.5, 0.0).into());
		assert_eq!(segment.position_at(0.75), (0.0, 0.896484375, 0.0).into());
		assert_eq!(segment.position_at(1.0), segment.end.position);
	}
}
