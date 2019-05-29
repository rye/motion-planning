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
		0 => 1. - 10. * t3 + 15. * t4 - 6. * t5,
		1 => t - 6. * t3 + 8. * t4 - 3. * t5,
		2 => 0.5 * t2 - 1.5 * t3 + 1.5 * t4 - 0.5 * t5,
		3 => 0.5 * t3 - t4 + 0.5 * t5,
		4 => -4. * t3 + 7. * t4 - 3. * t5,
		5 => 10. * t3 - 15. * t4 + 6. * t5,
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
		0 => -30. * t2 + 60. * t3 - 30. * t4,
		1 => 1. - 18. * t2 + 32. * t3 - 15. * t4,
		2 => t - 4.5 * t2 + 6. * t3 - 2.5 * t4,
		3 => 1.5 * t2 - 4. * t3 + 2.5 * t4,
		4 => -12. * t2 + 28. * t3 - 15. * t4,
		5 => 30. * t2 - 60. * t3 + 30. * t4,
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
