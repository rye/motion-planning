/// Computes the value of a quintic Hermite basis function.
///
/// The coefficients are results from the following Mathematica code.  In
/// layman's terms, the reason these basis functions are significant is that,
/// depending on the value of `n` and `t`, they range between `0` and `1`, in a
/// manner that is smooth.  `c[t]`, in the below code, is a standard quintic
/// polynomial, so the code below simply solves for values of `b0` through `b5`
/// and then rewrites `c[t]` in that form before rearranging.
///
/// ```Mathematica
/// c[t_] := b0 + b1 t + b2 t^2 + b3 t^3 + b4 t^4 + b5 t^5;
/// Collect[
///   c[t] /.
///     Solve[
///       {
///         (c[t] /. t -> 0) == p0,
///         (D[c[t], t] /. t -> 0) == v0,
///         (D[D[c[t], t], t] /. t -> 0) == a0,
///         (D[D[c[t], t], t] /. t -> 1) == a1
///         (D[c[t], t] /. t -> 1) == v1,
///         (c[t] /. t -> 1) == p1
///       },
///       {b0, b1, b2, b3, b4, b5}
///     ],
///   {p0, v0, a0, a1, v1, p1}
/// ]
/// ```
///
/// # Examples
///
/// ```rust
/// use motion_planning::hermite::h_5;
///
/// // at t=0, all of h_n^5 are `0` except for `h_0^5`.
/// assert_eq!(h_5(0., 0), 1.);
/// assert_eq!(h_5(0., 1), 0.);
/// assert_eq!(h_5(0., 2), 0.);
/// assert_eq!(h_5(0., 3), 0.);
/// assert_eq!(h_5(0., 4), 0.);
/// assert_eq!(h_5(0., 5), 0.);
///
/// // at t=1, all of h_n^5 are `0` except for `h_5^5`.
/// assert_eq!(h_5(1., 0), 0.);
/// assert_eq!(h_5(1., 1), 0.);
/// assert_eq!(h_5(1., 2), 0.);
/// assert_eq!(h_5(1., 3), 0.);
/// assert_eq!(h_5(1., 4), 0.);
/// assert_eq!(h_5(1., 5), 1.);
/// ```
///
/// # Panics
///
/// If `n` is not one of `0`, `1`, `2`, `3`, `4`, or `5`, this function panics.
///
/// ```should_panic
/// use motion_planning::hermite::h_5;
/// # let t = 0.5_f64;
/// h_5(t, 7);
/// ```
pub fn h_5(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);
	let t4 = t.powi(4);
	let t5 = t.powi(5);

	match n {
		0 => t5.mul_add(-6., t4.mul_add(15., t3.mul_add(-10., 1.))),
		1 => t5.mul_add(-3., t4.mul_add(8., t3.mul_add(-6., t))),
		2 => t5.mul_add(-0.5, t4.mul_add(1.5, t3.mul_add(-1.5, t2 * 0.5))),
		3 => t5.mul_add(0.5, t4.mul_add(-1., t3 * 0.5)),
		4 => t5.mul_add(-3., t4.mul_add(7., t3 * -4.)),
		5 => t5.mul_add(6., t4.mul_add(-15., t3 * 10.)),
		_ => unimplemented!(),
	}
}

/// Computes the value of the first time-derivative of a quintic Hermite basis
/// function.
///
/// In other words, this function is the first time-derivative of the
/// [`h_5`](crate::hermite::h_5) function.
///
/// # Examples
///
/// ```rust
/// use motion_planning::hermite::h_5p;
///
/// // at t=0, all of h_n^5' are `0` except for `h_1^5'`.
/// assert_eq!(h_5p(0., 0), 0.);
/// assert_eq!(h_5p(0., 1), 1.);
/// assert_eq!(h_5p(0., 2), 0.);
/// assert_eq!(h_5p(0., 3), 0.);
/// assert_eq!(h_5p(0., 4), 0.);
/// assert_eq!(h_5p(0., 5), 0.);
///
/// // at t=1, all of h_n^5' are `0` except for `h_4^5'`.
/// assert_eq!(h_5p(1., 0), 0.);
/// assert_eq!(h_5p(1., 1), 0.);
/// assert_eq!(h_5p(1., 2), 0.);
/// assert_eq!(h_5p(1., 3), 0.);
/// assert_eq!(h_5p(1., 4), 1.);
/// assert_eq!(h_5p(1., 5), 0.);
/// ```
///
/// # Panics
///
/// If `n` is not one of `0`, `1`, `2`, `3`, `4`, or `5`, this function panics.
///
/// ```should_panic
/// use motion_planning::hermite::h_5p;
/// # let t = 0.5_f64;
/// h_5p(t, 7);
/// ```
pub fn h_5p(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);
	let t4 = t.powi(4);

	match n {
		0 => t4.mul_add(-30., t3.mul_add(60., t2 * -30.)),
		1 => t4.mul_add(-15., t3.mul_add(32., t2.mul_add(-18., 1.))),
		2 => t4.mul_add(-2.5, t3.mul_add(6., t2.mul_add(-4.5, t))),
		3 => t4.mul_add(2.5, t3.mul_add(-4., t2 * 1.5)),
		4 => t4.mul_add(-15., t3.mul_add(28., t2 * -12.)),
		5 => t4.mul_add(30., t3.mul_add(-60., t2 * 30.)),
		_ => unimplemented!(),
	}
}

/// Computes the value of the second time-derivative of a quintic Hermite basis
/// function.
///
/// In other words, this function is the second time-derivative of the
/// [`h_5`](crate::hermite::h_5) function, or equivalently the first
/// time-derivative of the [`h_5p`](crate::hermite::h_5p) function.
///
/// # Examples
///
/// ```rust
/// use motion_planning::hermite::h_5pp;
///
/// // at t=0, all of h_n^5'' are `0` except for `h_2^5''`.
/// assert_eq!(h_5pp(0., 0), 0.);
/// assert_eq!(h_5pp(0., 1), 0.);
/// assert_eq!(h_5pp(0., 2), 1.);
/// assert_eq!(h_5pp(0., 3), 0.);
/// assert_eq!(h_5pp(0., 4), 0.);
/// assert_eq!(h_5pp(0., 5), 0.);
///
/// // at t=1, all of h_n^5'' are `0` except for `h_3^5''`.
/// assert_eq!(h_5pp(1., 0), 0.);
/// assert_eq!(h_5pp(1., 1), 0.);
/// assert_eq!(h_5pp(1., 2), 0.);
/// assert_eq!(h_5pp(1., 3), 1.);
/// assert_eq!(h_5pp(1., 4), 0.);
/// assert_eq!(h_5pp(1., 5), 0.);
/// ```
///
/// # Panics
///
/// If `n` is not one of `0`, `1`, `2`, `3`, `4`, or `5`, this function panics.
///
/// ```should_panic
/// use motion_planning::hermite::h_5pp;
/// # let t = 0.5_f64;
/// h_5pp(t, 7);
/// ```
pub fn h_5pp(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);

	match n {
		0 => t3.mul_add(-120., t2.mul_add(180., t * -60.)),
		1 => t3.mul_add(-60., t2.mul_add(96., t * -36.)),
		2 => t3.mul_add(-10., t2.mul_add(18., t.mul_add(-9., 1.))),
		3 => t3.mul_add(10., t2.mul_add(-12., t * 3.)),
		4 => t3.mul_add(-60., t2.mul_add(84., t * -24.)),
		5 => t3.mul_add(120., t2.mul_add(-180., t * 60.)),
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
