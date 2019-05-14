# motion-planning

This repository is a proof-of-concept implementation of motion planning in Rust.

## How it Works

We implement one struct, `Pose`, which contains information about the position,
velocity, and acceleration of the actor at certain key moments.  We implemented
a trait, `Path`, on all types matching the following key values:

```rust
impl<V> Path<V> for std::vec::Vec<Pose<V>>
where
    V: Copy,
    V: std::ops::Mul<f64, Output = V>,
    V: std::ops::Add<V, Output = V>,
{
    // snip
}
```

The advantage of doing it this way is that `V` can be any
`Copy, Mul<f64, Output=V>, Add<V, Output=V>` type; this means that linear
actuation is supported with no additional code, as is _n_-dimensional vectors
provided that they have implementations to support it.

We also added a `Vec3d(V, V, V)` tuple struct and a couple of other things that
are all transparently supported.

### Road Map

- [ ] Make more types like `Vec3d`.
- [ ] Implement Trajectory calculation from a Vec of poses (already almost
  entirely done!)
- [ ] Implement constraints. (e.g. max vel)
- [ ] Implement Serde bindings so that this can be used e.g. in web servers
- [ ] Make `Vec3d`-ish types actually be slices instead of tuples, generalize
  `Mul`/`Neg` types.
- [ ] Implement Rust API Guidelines
