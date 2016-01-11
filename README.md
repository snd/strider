# strider

[![Build Status](https://travis-ci.org/snd/strider.svg?branch=master)](https://travis-ci.org/snd/strider/branches)
[![](https://meritbadge.herokuapp.com/strider)](https://crates.io/crates/strider)

**[ringbuffer operations on multiple values at once]
(https://snd.github.io/strider/strider/trait.SliceRing.html)
with an
[efficient implementation]
(https://snd.github.io/strider/strider/index.html#performance).
written in [rust](https://www.rust-lang.org/).**

useful for moving a window with variable step
through a possibly infinite
stream of values
[while avoiding unnecessary memory allocations]
(https://snd.github.io/strider/strider/index.html#memory)

handy when computing the [short-time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform).

to use add `strider = "*"`
to the `[dependencies]` section of your `Cargo.toml` and call `extern crate strider;` in your code.

## [read the documentation for an example and more !](https://snd.github.io/strider/strider/index.html)

### [contributing](contributing.md)

### [license: MIT](LICENSE)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
