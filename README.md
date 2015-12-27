# sliding-window

often you want to do deque operations on multiple values at
once. operations implemented on std::collections::VecDeque
as well as an optimized implementation.

step through a (possibly infinite, streaming) series of data
while avoiding unnecessary memory allocations.



for stepping through a possibly infinite streaming series of data





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
