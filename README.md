# imu

This crate provides a Rust port of the IMU sensor fusion algorithm presented by Sebastian Madgwick in his paper [An efficient orientation filter for inertial and inertial/magnetic sensor arrays](http://x-io.co.uk/res/doc/madgwick_internal_report.pdf).

## Testing

Property based testing via [quickcheck](https://github.com/BurntSushi/quickcheck) and Rust foreign function invocation are used in order to ensure the behavior of this crate matches the behavior of the original c-lang implementation by Madgwick.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
