extern crate motion_planning;

use motion_planning::{Pose, Segment, Vec3d};

fn main() {
	let segment: Segment<Pose<Vec3d<f64>>> = Segment {
		start: Pose {
			position: Vec3d(0.0, 0.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
		end: Pose {
			position: Vec3d(0.0, 1.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
	};

	for t in 0..1000 {
		let t: f64 = f64::from(t) * 0.001;
		let pos = segment.position_at(t);

		println!("{},{},{},{}", t, pos.0, pos.1, pos.2);
	}
}
