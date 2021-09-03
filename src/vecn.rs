use core::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vecn<V, const N: usize>([V; N]);

impl<V, const N: usize> Add<Self> for Vecn<V, N>
where
	V: Add<V, Output = V>,
	V: Copy,
{
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		use core::convert::TryInto;

		// TODO: Once rust-lang/rust#80094 (array_zip) is stabilized as is array_map (rust-lang/rust#75243), use this one
		//
		//     Self(self.0.zip(other.0.map(|x| x.0 + x.1)))

		let self_coords = self.0.iter();
		let other_coords = other.0.iter();
		let new_coords: Vec<V> = self_coords
			.zip(other_coords)
			.map(|(s, o)| *s + *o)
			.collect();

		assert!(new_coords.len() == N, "zip called on two equal-length vectors (compile-time check) did not return iter with expected length; cannot produce fixed-size ary");
		let new_coords: [V; N] = new_coords.try_into().ok().unwrap();

		Self(new_coords)
	}
}

#[cfg(test)]
mod add {
	use super::Vecn;

	#[test]
	fn add_1() {
		let x_bar: Vecn<_, 1> = Vecn([1.0]);
		let y_bar: Vecn<_, 1> = Vecn([2.0]);
		assert_eq!(x_bar + y_bar, Vecn([3.0]));
	}

	#[test]
	fn add_2() {
		let x_bar: Vecn<_, 2> = Vecn([1.0, 2.0]);
		let y_bar: Vecn<_, 2> = Vecn([2.0, 4.0]);
		assert_eq!(x_bar + y_bar, Vecn([3.0, 6.0]));
	}
}
