pub fn h_5(t: f64, n: usize) -> f64 {
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

pub fn h_5p(t: f64, n: usize) -> f64 {
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

pub fn h_5pp(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);

	match n {
		0 => -60. * t + 180. * t2 - 120. * t3,
		1 => -36. * t + 96. * t2 - 60. * t3,
		2 => 1. - 9. * t + 18. * t2 - 10. * t3,
		3 => 3. * t - 12. * t2 + 10. * t3,
		4 => -24. * t + 84. * t2 - 60. * t3,
		5 => 60. * t - 180. * t2 + 120. * t3,
		_ => unimplemented!(),
	}
}

#[test]
fn h_5_is_correct() {
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
fn h_5p_is_correct() {
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
fn h_5pp_is_correct() {
	assert_eq!(h_5pp(0.0, 0), 0.);
	assert_eq!(h_5pp(0.0, 1), 0.);
	assert_eq!(h_5pp(0.0, 2), 1.);
	assert_eq!(h_5pp(0.0, 3), 0.);
	assert_eq!(h_5pp(0.0, 4), 0.);
	assert_eq!(h_5pp(0.0, 5), 0.);

	assert_eq!(h_5pp(1.0, 0), 0.);
	assert_eq!(h_5pp(1.0, 1), 0.);
	assert_eq!(h_5pp(1.0, 2), 0.);
	assert_eq!(h_5pp(1.0, 3), 1.);
	assert_eq!(h_5pp(1.0, 4), 0.);
	assert_eq!(h_5pp(1.0, 5), 0.);
}
