use motion_planning::vec::Vec3d;
use motion_planning::{Pose3, Trajectory3};

fn main() {
	let segment: Vec<Pose3<Vec3d<f64>>> = vec![
		Pose3 {
			position: Vec3d(0.0, 0.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
		Pose3 {
			position: Vec3d(0.0, 1.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
	];

	for t in 0..=1000 {
		let t: f64 = f64::from(t) * 0.001;

		let pos = segment.position_at(t).unwrap();
		let vel = segment.velocity_at(t).unwrap();
		let acc = segment.acceleration_at(t).unwrap();

		println!("{},{},{},{}", t, pos, vel, acc);
	}
}
