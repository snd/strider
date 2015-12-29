# strider (WIP)

*this is a work in progress:
works well.
has a large test and benchmark suite.
documentation is unfinished and in rough shape.
lacks polish. will probably change a lot.

[![Build Status](https://travis-ci.org/snd/strider.svg?branch=master)](https://travis-ci.org/snd/strider/branches)
[![](https://meritbadge.herokuapp.com/strider)](https://crates.io/crates/strider)

> strider: one who walks rapidly with long steps

**useful for stepping (variable step) a window (variable size)
through a streaming (possibly infinite)
series of values [while avoiding
unnecessary memory allocations](https://snd.github.io/strider/strider/index.html#memory)**

this is needed for the [short-time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform)
and other data/signal processing methods.

to use add `strider = "0.1.0"`
to the `[dependencies]` section of your `Cargo.toml` and `extern crate strider;` in your code.

## [read on in the documentation](https://snd.github.io/strider/strider/index.html)

### [contributing](contributing.md)

### [license: MIT](LICENSE)
