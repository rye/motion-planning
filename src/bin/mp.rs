extern crate motion_planning;

use motion_planning::{Pose, Trajectory, Vec3d};

fn main() {
	let segment: Vec<Pose<Vec3d<f64>>> = vec![
		Pose {
			position: Vec3d(0.0, 0.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
		Pose {
			position: Vec3d(0.0, 1.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
	];

	for t in 0..=1000 {
		let t: f64 = f64::from(t) * 0.001;

		let pos = segment.position_at(t).unwrap();
		let vel = segment.velocity_at(t).unwrap();

		println!("{},{:?},{:?}", t, pos, vel);
	}
}

#[test]
fn main_runs() {
	assert_eq!(main(), ());
}
