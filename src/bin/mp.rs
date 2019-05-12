extern crate motion_planning;

use motion_planning::{Pose, Segment, Vec3d};

fn main() {
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

	for t in 0..1000 {
		let t: f64 = t as f64 * 0.001;
		let pos = segment.position_at(t);

		println!("{},{},{},{}", t, pos.x, pos.y, pos.z);

	}
}
