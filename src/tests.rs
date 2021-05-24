use super::vec::Vec3d;
use super::*;

#[test]
fn get_segment_returns_correct_segment() {
	let segment = vec![
		// Start at origin, moving north at 1.0 u/s
		Pose3 {
			position: Vec3d(0.0f64, 0.0, 0.0),
			velocity: Vec3d(0.0, 1.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
		// Stop at (0,1,0)
		Pose3 {
			position: Vec3d(0.0f64, 1.0, 0.0),
			velocity: Vec3d(0.0, 0.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
		// Accelerate through (0,2,0), moving north at 1.0 u/s
		Pose3 {
			position: Vec3d(0.0f64, 2.0, 0.0),
			velocity: Vec3d(0.0, 1.0, 0.0),
			acceleration: Vec3d(0.0, 0.0, 0.0),
		},
	];

	let subdiv = 4.;

	for t in 0..=(2 * subdiv as usize) {
		let t = t as f64 / subdiv;

		if t.fract() == 0. {
			assert_eq!(
				segment.get_segment(t),
				Some(Segment3(
					t.fract(),
					&segment[t as usize],
					&segment[t as usize]
				))
			);
		} else {
			assert_eq!(
				segment.get_segment(t),
				Some(Segment3(
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
	let segment: Vec<Pose3<Vec3d<f64>>> = Vec::new();

	assert_eq!(segment.get_segment(0.5), None);
}

#[test]
fn position_correct_straight_line() {
	let mut segment = Vec::new();

	segment.push(Pose3 {
		position: Vec3d(0.0f64, 0.0, 0.0),
		velocity: Vec3d(0.0, 0.0, 0.0),
		acceleration: Vec3d(0.0, 0.0, 0.0),
	});

	segment.push(Pose3 {
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
	let segment: Vec<Pose3<Vec3d<f64>>> = Vec::new();

	assert_eq!(segment.position_at(0.0), None);
}

#[test]
fn velocity_length_zero() {
	let segment: Vec<Pose3<Vec3d<f64>>> = Vec::new();

	assert_eq!(segment.velocity_at(0.0), None);
}

#[test]
fn acceleration_length_zero() {
	let segment: Vec<Pose3<Vec3d<f64>>> = Vec::new();

	assert_eq!(segment.acceleration_at(0.0), None);
}

#[test]
fn velocity_correct_straight_line_opp_starts() {
	let mut segment = Vec::new();

	segment.push(Pose3 {
		position: Vec3d(0.0f64, 0.0, 0.0),
		velocity: Vec3d(0.0, 1.0, 0.0),
		acceleration: Vec3d(0.0, 0.0, 0.0),
	});

	segment.push(Pose3 {
		position: Vec3d(0.0f64, 1.0, 0.0),
		velocity: Vec3d(0.0, 0.0, 0.0),
		acceleration: Vec3d(0.0, 0.0, 0.0),
	});

	assert_eq!(segment.velocity_at(0.0), Some(segment[0].velocity));
	assert_eq!(segment.velocity_at(0.5), Some(Vec3d(0.0, 1.4375, 0.0)));
	assert_eq!(segment.velocity_at(1.0), Some(segment[1].velocity));
}

#[test]
fn acceleration_correct_curve() {
	let mut segment = Vec::new();

	segment.push(Pose3 {
		position: Vec3d(0.0f64, 0.0, 0.0),
		velocity: Vec3d(0.0, 1.0, 0.0),
		acceleration: Vec3d(1.0, 0.0, 0.0),
	});

	segment.push(Pose3 {
		position: Vec3d(1.0f64, 1.0, 0.0),
		velocity: Vec3d(1.0, 0.0, 0.0),
		acceleration: Vec3d(0.0, -1.0, 0.0),
	});

	assert_eq!(segment.acceleration_at(0.0), Some(segment[0].acceleration));
	assert_eq!(segment.acceleration_at(0.5), Some(Vec3d(1.25, -1.25, 0.0)));
	assert_eq!(segment.acceleration_at(1.0), Some(segment[1].acceleration));
}

#[test]
fn all_set_points_hit() {
	let mut segment = Vec::new();

	// Start at origin, moving north at 1.0 u/s
	segment.push(Pose3 {
		position: Vec3d(0.0f64, 0.0, 0.0),
		velocity: Vec3d(0.0, 1.0, 0.0),
		acceleration: Vec3d(0.0, 0.0, 0.0),
	});

	// Stop at (0,1,0)
	segment.push(Pose3 {
		position: Vec3d(0.0f64, 1.0, 0.0),
		velocity: Vec3d(0.0, 0.0, 0.0),
		acceleration: Vec3d(0.0, 0.0, 0.0),
	});

	// Accelerate through (0,2,0), moving north at 1.0 u/s
	segment.push(Pose3 {
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
