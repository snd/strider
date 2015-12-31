# strider

[![Build Status](https://travis-ci.org/snd/strider.svg?branch=master)](https://travis-ci.org/snd/strider/branches)
[![](https://meritbadge.herokuapp.com/strider)](https://crates.io/crates/strider)

**[ringbuffer operations on multiple values at once]
(https://snd.github.io/strider/strider/trait.SliceRing.html)
with an
[efficient implementation]
(https://snd.github.io/strider/strider/index.html#performance)**

useful for moving a window with variable step
through a possibly infinite
stream of values
[while avoiding unnecessary memory allocations]
(https://snd.github.io/strider/strider/index.html#memory)

handy when computing the [short-time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform).

to use add `strider = "0.1.2"`
to the `[dependencies]` section of your `Cargo.toml` and call `extern crate strider;` in your code.

## [read the documentation for an example and more !](https://snd.github.io/strider/strider/index.html)

### [contributing](contributing.md)

### [license: MIT](LICENSE)
