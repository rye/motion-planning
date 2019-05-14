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

The advantage of doing it this way is that `V` can be any `Copy`,
`Mul<f64, Output=V>`, `Add<V, Output=V>` type; this means that linear
actuation is supported with no additional code, as is _n_-dimensional vectors
provided that they have implementations to support it.  This means that our API
flexibly handles all data types for which these traits are implemented!

We also added a `Vec3d(V, V, V)` tuple struct and a couple of other things that
are all transparently supported.

### What the Heck is a Quintic Hermite spline?

Say you have a robot.  The robot drives around in paths that you specify, but
you want to plan the paths so that the robot hits a certain set of key
positions at certain velocities, accelerations, etc.; but you want your robot
to last as long as possible, so you don't want to thrash it around.

Since acceleration, velocity, and position are values in motion planning that
would ideally be smoothly controlled, and since the output of a position or
velocity equation with a time-parameter is controlled by a total of six of
these vector parameters, we calculate the outputs of a six-term (degree 5)
polynomial.  There are plenty of other ways to do this, one of the favorites
being B&eacute;zier curves.

Quintic refers to the degree of our resulting polynomial&mdash;for position
calculations we use a degree-5 polynomial, for velocity degree-4, etc.&mdash;
and Hermite polynomials were developed by Charles Hermite.

## Road Map

- [ ] Make more types like `Vec3d`.
- [ ] Implement Trajectory calculation from a Vec of poses (already almost
  entirely done!)
- [ ] Implement constraints. (e.g. max vel)
- [ ] Implement Serde bindings so that this can be used e.g. in web servers
- [ ] Make `Vec3d`-ish types actually be slices instead of tuples, generalize
  `Mul`/`Neg` types.
- [ ] Implement Rust API Guidelines
