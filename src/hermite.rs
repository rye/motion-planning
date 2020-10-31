#[cfg(test)]
use super::assert_f64_roughly_eq;

mod cubic;
pub use cubic::*;

mod quintic;
pub use quintic::*;

mod septic;
pub use septic::*;
