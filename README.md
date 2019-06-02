# motion-planning

This repository is a proof-of-concept implementation of motion planning in Rust.

## How it Works

We implement one struct, `Pose`, which contains information about the position,
velocity, and acceleration of the actor at certain key moments.  We implemented
a trait, `Trajectory`, on all types matching the following key values:

```rust
impl<V> Trajectory<V> for std::vec::Vec<Pose<V>>
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

## Maintenance Status

This project is still experimental.  We have not released v1 yet, so it does
not have a stable public API.  We have also not started releasing tagged
versions yet, either, so the code should be considered in an "in-development"
stage.

This project adheres to semantic versioning.  Once a major release occurs, any
subsequent releases that change the existing public API will result in another
major release.  New arguments to binaries, new features, or additional features
will warrant new minor releases.  Any fixes to existing behavior or
non-source-related changes will warrant new patch releases.  Tests will be a
canary for this change&mdash;if a change to a test is required, this indicates
that a new major version will be released, as a test is an encapsulated example
of what "public behavior" looks like.

We only maintain the most recent `MAJOR.MINOR` version.  (If the latest
release is `v6.7.3`, bugs will only be considered relevant if they are shown to
affect the `v6.7.X` series.)  This is because of the relatively small nature of
this project.

## License

This project is licensed under the MIT License.  (See [LICENSE](LICENSE))

## Contributing

You are welcome to fork this project and make contributions as you see fit.
See the [contributors' documentation](.github/CONTRIBUTING.md) for more
information.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, shall be licensed under the terms of the
MIT license.
