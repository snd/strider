# strider

> strider: one who walks rapidly with long steps

**useful for stepping through a streaming, possibly infinite
series (sequence) of data (values) while fast and [while avoiding
unnecessary memory allocations](#memory).**

### [generated documentation](https://snd.github.io/strider/strider/index.html)

### motivation

the [short-time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform),
and other data processing methods, require stepping through a
series of values

useful for stepping
through a streaming, possible infinite series of data
while avoiding unnecessary memory allocations.

### example


two backing buffer types:
one simple for illustration
one optimized for performance
benchmarked against each other

often you want to do deque operations on multiple values at
once. operations implemented on std::collections::VecDeque
as well as an optimized implementation.








this is probably what you want

### memory

sliding-window uses a `std::collections::VecDeque` under the hood.

will double in size only when full.

for the common case this allocates memory once, maybe twice and is done with it.

fast

### performance

optimized for and restricted to working on multiple things at once.
which it can do faster.

performance is relative. so instead of claiming that this is fast, we'll instead prove
that it is much faster than a naive implementation building on `std::collections::VecDeque`.

### [contributing](contributing.md)

### [license: MIT](LICENSE)
