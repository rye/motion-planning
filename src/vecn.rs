#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vecn<V, const N: usize>([V; N]);
