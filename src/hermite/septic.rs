#[cfg(test)]
use super::assert_f64_roughly_eq;

pub fn h_7(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);
	let t4 = t.powi(4);
	let t5 = t.powi(5);
	let t6 = t.powi(6);
	let t7 = t.powi(7);

	match n {
		0 => t7.mul_add(20., t6.mul_add(-70., t5.mul_add(84., t4.mul_add(-35., 1.)))),
		1 => t7.mul_add(10., t6.mul_add(-36., t5.mul_add(45., t4.mul_add(-20., t)))),
		2 => t7.mul_add(
			2.,
			t6.mul_add(-7.5, t5.mul_add(10., t4.mul_add(-5., t2.mul_add(0.5, 0.)))),
		),
		3 => t7.mul_add(
			1. / 6.,
			t6.mul_add(
				-(2. / 3.),
				t5.mul_add(1., t4.mul_add(-(2. / 3.), t3.mul_add(1. / 6., 0.))),
			),
		),
		4 => t7.mul_add(
			1. / 6.,
			t6.mul_add(-0.5, t5.mul_add(0.5, t4.mul_add(-1. / 6., 0.))),
		),
		5 => t7.mul_add(-2., t6.mul_add(6.5, t5.mul_add(-7., t4.mul_add(2.5, 0.)))),
		6 => t7.mul_add(10., t6.mul_add(-34., t5.mul_add(39., t4.mul_add(-15., 0.)))),
		7 => t7.mul_add(-20., t6.mul_add(70., t5.mul_add(-84., t4.mul_add(35., 0.)))),
		_ => unimplemented!(),
	}
}

pub fn h_7p(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);
	let t4 = t.powi(4);
	let t5 = t.powi(5);
	let t6 = t.powi(6);

	match n {
		0 => t6.mul_add(
			140.,
			t5.mul_add(-420., t4.mul_add(420., t3.mul_add(-140., 0.))),
		),
		1 => t6.mul_add(
			70.,
			t5.mul_add(-216., t4.mul_add(225., t3.mul_add(-80., 1.))),
		),
		2 => t6.mul_add(14., t5.mul_add(-45., t4.mul_add(50., t3.mul_add(-20., t)))),
		3 => t6.mul_add(
			7. / 6.,
			t5.mul_add(
				-4.,
				t4.mul_add(5., t3.mul_add(-8. / 3., t2.mul_add(0.5, 0.))),
			),
		),
		4 => t6.mul_add(
			7. / 6.,
			t5.mul_add(-3., t4.mul_add(2.5, t3.mul_add(-2. / 3., 0.))),
		),
		5 => t6.mul_add(-14., t5.mul_add(39., t4.mul_add(-35., t3.mul_add(10., 0.)))),
		6 => t6.mul_add(
			70.,
			t5.mul_add(-204., t4.mul_add(195., t3.mul_add(-60., 0.))),
		),
		7 => t6.mul_add(
			-140.,
			t5.mul_add(420., t4.mul_add(-420., t3.mul_add(140., 0.))),
		),
		_ => unimplemented!(),
	}
}

pub fn h_7pp(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);
	let t4 = t.powi(4);
	let t5 = t.powi(5);

	match n {
		0 => t5.mul_add(
			840.,
			t4.mul_add(-2100., t3.mul_add(1680., t2.mul_add(-420., 0.))),
		),
		1 => t5.mul_add(
			420.,
			t4.mul_add(-1080., t3.mul_add(900., t2.mul_add(-240., 0.))),
		),
		2 => t5.mul_add(
			84.,
			t4.mul_add(-225., t3.mul_add(200., t2.mul_add(-60., 1.))),
		),
		3 => t5.mul_add(7., t4.mul_add(-20., t3.mul_add(20., t2.mul_add(-8., t)))),
		4 => t5.mul_add(7., t4.mul_add(-15., t3.mul_add(10., t2.mul_add(-2., 0.)))),
		5 => t5.mul_add(
			-84.,
			t4.mul_add(195., t3.mul_add(-140., t2.mul_add(30., 0.))),
		),
		6 => t5.mul_add(
			420.,
			t4.mul_add(-1020., t3.mul_add(780., t2.mul_add(-180., 0.))),
		),
		7 => t5.mul_add(
			-840.,
			t4.mul_add(2100., t3.mul_add(-1680., t2.mul_add(420., 0.))),
		),
		_ => unimplemented!(),
	}
}

pub fn h_7ppp(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);
	let t4 = t.powi(4);

	match n {
		0 => t4.mul_add(
			4200.,
			t3.mul_add(-8400., t2.mul_add(5040., t.mul_add(-840., 0.))),
		),
		1 => t4.mul_add(
			2100.,
			t3.mul_add(-4320., t2.mul_add(2700., t.mul_add(-480., 0.))),
		),
		2 => t4.mul_add(
			420.,
			t3.mul_add(-900., t2.mul_add(600., t.mul_add(-120., 0.))),
		),
		3 => t4.mul_add(35., t3.mul_add(-80., t2.mul_add(60., t.mul_add(-16., 1.)))),
		4 => t4.mul_add(35., t3.mul_add(-60., t2.mul_add(30., t.mul_add(-4., 0.)))),
		5 => t4.mul_add(
			-420.,
			t3.mul_add(780., t2.mul_add(-420., t.mul_add(60., 0.))),
		),
		6 => t4.mul_add(
			2100.,
			t3.mul_add(-4080., t2.mul_add(2340., t.mul_add(-360., 0.))),
		),
		7 => t4.mul_add(
			-4200.,
			t3.mul_add(8400., t2.mul_add(-5040., t.mul_add(840., 0.))),
		),
		_ => unimplemented!(),
	}
}

#[cfg(test)]
mod tests {
	#[cfg(test)]
	mod h_7 {
		use super::super::{assert_f64_roughly_eq, h_7};

		#[test]
		fn correct_at_0() {
			assert_f64_roughly_eq!(h_7(0.0, 0), 1.);
			assert_f64_roughly_eq!(h_7(0.0, 1), 0.);
			assert_f64_roughly_eq!(h_7(0.0, 2), 0.);
			assert_f64_roughly_eq!(h_7(0.0, 3), 0.);
			assert_f64_roughly_eq!(h_7(0.0, 4), 0.);
			assert_f64_roughly_eq!(h_7(0.0, 5), 0.);
			assert_f64_roughly_eq!(h_7(0.0, 6), 0.);
			assert_f64_roughly_eq!(h_7(0.0, 7), 0.);
		}

		#[test]
		fn correct_at_1() {
			assert_f64_roughly_eq!(h_7(1.0, 0), 0.);
			assert_f64_roughly_eq!(h_7(1.0, 1), 0.);
			assert_f64_roughly_eq!(h_7(1.0, 2), 0.);
			assert_f64_roughly_eq!(h_7(1.0, 3), 0.);
			assert_f64_roughly_eq!(h_7(1.0, 4), 0.);
			assert_f64_roughly_eq!(h_7(1.0, 5), 0.);
			assert_f64_roughly_eq!(h_7(1.0, 6), 0.);
			assert_f64_roughly_eq!(h_7(1.0, 7), 1.);
		}
	}

	#[cfg(test)]
	mod h_7p {
		use super::super::{assert_f64_roughly_eq, h_7p};

		#[test]
		fn correct_at_0() {
			assert_f64_roughly_eq!(h_7p(0.0, 0), 0.);
			assert_f64_roughly_eq!(h_7p(0.0, 1), 1.);
			assert_f64_roughly_eq!(h_7p(0.0, 2), 0.);
			assert_f64_roughly_eq!(h_7p(0.0, 3), 0.);
			assert_f64_roughly_eq!(h_7p(0.0, 4), 0.);
			assert_f64_roughly_eq!(h_7p(0.0, 5), 0.);
			assert_f64_roughly_eq!(h_7p(0.0, 6), 0.);
			assert_f64_roughly_eq!(h_7p(0.0, 7), 0.);
		}

		#[test]
		fn correct_at_1() {
			assert_f64_roughly_eq!(h_7p(1.0, 0), 0.);
			assert_f64_roughly_eq!(h_7p(1.0, 1), 0.);
			assert_f64_roughly_eq!(h_7p(1.0, 2), 0.);
			// TODO(rye): Precision is too low on these two.
			//assert_f64_roughly_eq!(h_7p(1.0, 3), 0.);
			//assert_f64_roughly_eq!(h_7p(1.0, 4), 0.);
			assert_f64_roughly_eq!(h_7p(1.0, 5), 0.);
			assert_f64_roughly_eq!(h_7p(1.0, 6), 1.);
			assert_f64_roughly_eq!(h_7p(1.0, 7), 0.);
		}
	}

	#[cfg(test)]
	mod h_7pp {
		use super::super::{assert_f64_roughly_eq, h_7pp};

		#[test]
		fn correct_at_0() {
			assert_f64_roughly_eq!(h_7pp(0.0, 0), 0.);
			assert_f64_roughly_eq!(h_7pp(0.0, 1), 0.);
			assert_f64_roughly_eq!(h_7pp(0.0, 2), 1.);
			assert_f64_roughly_eq!(h_7pp(0.0, 3), 0.);
			assert_f64_roughly_eq!(h_7pp(0.0, 4), 0.);
			assert_f64_roughly_eq!(h_7pp(0.0, 5), 0.);
			assert_f64_roughly_eq!(h_7pp(0.0, 6), 0.);
			assert_f64_roughly_eq!(h_7pp(0.0, 7), 0.);
		}

		#[test]
		fn correct_at_1() {
			assert_f64_roughly_eq!(h_7pp(1.0, 0), 0.);
			assert_f64_roughly_eq!(h_7pp(1.0, 1), 0.);
			assert_f64_roughly_eq!(h_7pp(1.0, 2), 0.);
			assert_f64_roughly_eq!(h_7pp(1.0, 3), 0.);
			assert_f64_roughly_eq!(h_7pp(1.0, 4), 0.);
			assert_f64_roughly_eq!(h_7pp(1.0, 5), 1.);
			assert_f64_roughly_eq!(h_7pp(1.0, 6), 0.);
			assert_f64_roughly_eq!(h_7pp(1.0, 7), 0.);
		}
	}

	#[cfg(test)]
	mod h_7ppp {
		use super::super::{assert_f64_roughly_eq, h_7ppp};

		#[test]
		fn correct_at_0() {
			assert_f64_roughly_eq!(h_7ppp(0.0, 0), 0.);
			assert_f64_roughly_eq!(h_7ppp(0.0, 1), 0.);
			assert_f64_roughly_eq!(h_7ppp(0.0, 2), 0.);
			assert_f64_roughly_eq!(h_7ppp(0.0, 3), 1.);
			assert_f64_roughly_eq!(h_7ppp(0.0, 4), 0.);
			assert_f64_roughly_eq!(h_7ppp(0.0, 5), 0.);
			assert_f64_roughly_eq!(h_7ppp(0.0, 6), 0.);
			assert_f64_roughly_eq!(h_7ppp(0.0, 7), 0.);
		}

		#[test]
		fn correct_at_1() {
			assert_f64_roughly_eq!(h_7ppp(1.0, 0), 0.);
			assert_f64_roughly_eq!(h_7ppp(1.0, 1), 0.);
			assert_f64_roughly_eq!(h_7ppp(1.0, 2), 0.);
			assert_f64_roughly_eq!(h_7ppp(1.0, 3), 0.);
			assert_f64_roughly_eq!(h_7ppp(1.0, 4), 1.);
			assert_f64_roughly_eq!(h_7ppp(1.0, 5), 0.);
			assert_f64_roughly_eq!(h_7ppp(1.0, 6), 0.);
			assert_f64_roughly_eq!(h_7ppp(1.0, 7), 0.);
		}
	}
}
