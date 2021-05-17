//! The cubic Hermite basis functions.

/// Computes the value of a cubic Hermite basis function.
///
/// The coefficients are results from the following Mathematica code.  In
/// layman's terms, the reason these basis functions are significant is that,
/// depending on the value of `n` and `t`, they range between `0` and `1`, in a
/// manner that is smooth.  `c[t]`, in the below code, is a standard cubic
/// polynomial, so the code below simply solves for values of `b0` through `b3`
/// and then rewrites `c[t]` in that form before rearranging.
///
/// ```Mathematica
/// c[t_] := b0 + b1 t + b2 t^2 + b3 t^3;
/// Collect[
///   c[t] /.
///     Solve[
///       {
///         (c[t] /. t -> 0) == p0,
///         (D[c[t], t] /. t -> 0) == v0,
///         (D[c[t], t] /. t -> 1) == v1,
///         (c[t] /. t -> 1) == p1
///       },
///       {b0, b1, b2, b3}
///     ],
///   {p0, v0, v1, p1}
/// ]
/// ```
///
/// # Examples
///
/// ```rust
/// use motion_planning::hermite::cubic::h_3;
///
/// // at t=0, all of h_n^3 are `0` except for `h_0^3`.
/// assert_eq!(h_3(0., 0), 1.);
/// assert_eq!(h_3(0., 1), 0.);
/// assert_eq!(h_3(0., 2), 0.);
/// assert_eq!(h_3(0., 3), 0.);
///
/// // at t=1, all of h_n^3 are `0` except for `h_3^3`.
/// assert_eq!(h_3(1., 0), 0.);
/// assert_eq!(h_3(1., 1), 0.);
/// assert_eq!(h_3(1., 2), 0.);
/// assert_eq!(h_3(1., 3), 1.);
/// ```
///
/// # Panics
///
/// If `n` is not one of `0`, `1`, `2`, or `3`, this function panics.
///
/// ```should_panic
/// use motion_planning::hermite::cubic::h_3;
/// # let t = 0.5_f64;
/// h_3(t, 7);
/// ```
pub fn h_3(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);
	let t3 = t.powi(3);

	match n {
		0 => t3.mul_add(2., t2.mul_add(-3., 1.)),
		1 => t3.mul_add(1., t2.mul_add(-2., t)),
		2 => t3.mul_add(1., t2.mul_add(-1., 0.)),
		3 => t3.mul_add(-2., t2.mul_add(3., 0.)),
		_ => unimplemented!(),
	}
}

/// Computes the value of the first time-derivative of the corresponding cubic
/// Hermite basis function.
///
/// In other words, this function is the first time-derivative of the
/// [`h_3`](crate::hermite::h_3) function.
///
/// # Examples
///
/// ```rust
/// use motion_planning::hermite::cubic::h_3p;
///
/// // at t=0, all of h_n^3' are `0` except for `h_1^3'`.
/// assert_eq!(h_3p(0., 0), 0.);
/// assert_eq!(h_3p(0., 1), 1.);
/// assert_eq!(h_3p(0., 2), 0.);
/// assert_eq!(h_3p(0., 3), 0.);
///
/// // at t=1, all of h_n^3' are `0` except for `h_2^3'`.
/// assert_eq!(h_3p(1., 0), 0.);
/// assert_eq!(h_3p(1., 1), 0.);
/// assert_eq!(h_3p(1., 2), 1.);
/// assert_eq!(h_3p(1., 3), 0.);
/// ```
///
/// # Panics
///
/// If `n` is not one of `0`, `1`, `2`, or `3` this function panics.
///
/// ```should_panic
/// use motion_planning::hermite::cubic::h_3p;
/// # let t = 0.5_f64;
/// h_3p(t, 7);
/// ```
pub fn h_3p(t: f64, n: usize) -> f64 {
	let t2 = t.powi(2);

	match n {
		0 => t2.mul_add(6., t.mul_add(-6., 0.)),
		1 => t2.mul_add(3., t.mul_add(-4., 1.)),
		2 => t2.mul_add(3., t.mul_add(-2., 0.)),
		3 => t2.mul_add(-6., t.mul_add(6., 0.)),
		_ => unimplemented!(),
	}
}

#[cfg(test)]
use super::assert_f64_roughly_eq;

#[cfg(test)]
mod tests {
	#[cfg(test)]
	mod h_3 {
		use super::super::{assert_f64_roughly_eq, h_3};

		#[test]
		fn correct_at_0() {
			assert_f64_roughly_eq!(h_3(0.0, 0), 1.);
			assert_f64_roughly_eq!(h_3(0.0, 1), 0.);
			assert_f64_roughly_eq!(h_3(0.0, 2), 0.);
			assert_f64_roughly_eq!(h_3(0.0, 3), 0.);
		}

		#[test]
		fn correct_at_1() {
			assert_f64_roughly_eq!(h_3(1.0, 0), 0.);
			assert_f64_roughly_eq!(h_3(1.0, 1), 0.);
			assert_f64_roughly_eq!(h_3(1.0, 2), 0.);
			assert_f64_roughly_eq!(h_3(1.0, 3), 1.);
		}
	}

	#[cfg(test)]
	mod h_3p {
		use super::super::{assert_f64_roughly_eq, h_3p};

		#[test]
		fn correct_at_0() {
			assert_f64_roughly_eq!(h_3p(0.0, 0), 0.);
			assert_f64_roughly_eq!(h_3p(0.0, 1), 1.);
			assert_f64_roughly_eq!(h_3p(0.0, 2), 0.);
			assert_f64_roughly_eq!(h_3p(0.0, 3), 0.);
		}

		#[test]
		fn correct_at_1() {
			assert_f64_roughly_eq!(h_3p(1.0, 0), 0.);
			assert_f64_roughly_eq!(h_3p(1.0, 1), 0.);
			assert_f64_roughly_eq!(h_3p(1.0, 2), 1.);
			assert_f64_roughly_eq!(h_3p(1.0, 3), 0.);
		}
	}
}
