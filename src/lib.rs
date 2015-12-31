/*!

**[ringbuffer operations on multiple values at once]
(trait.SliceRing.html)
with an
[efficient implementation]
(struct.SliceRingImpl.html).**

useful for moving a window with variable step
through a possibly infinite
stream of values
[while avoiding
unnecessary memory allocations]
(#memory)

handy when computing the [short-time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform).

to use add `strider = "0.1.1"`
to the `[dependencies]` section of your `Cargo.toml` and call `extern crate strider;` in your code.

## example

the following program reads a stream of chars from stdin.
it moves a window of size 4 (`WINDOW_SIZE`)
through that stream in steps of 2 (`STEP_SIZE`).
it writes the contents of every window to stdout.
for the input of `ABCDEFGHIJK` it produces the output `ABCDCDEFEFGHGHIJ`.
it uses constant memory and does **no** allocations after the initial ones.
you should be able to adapt it to your needs.

```no_run
use std::io;
use std::io::{Write, Read};

extern crate strider;
use strider::{SliceRing, SliceRingImpl};

fn main() {
    const WINDOW_SIZE: usize = 4;
    const STEP_SIZE: usize = 2;

    let mut ring = SliceRingImpl::<u8>::new();
    let mut input_buffer: &mut [u8] = &mut [0; 20];
    let mut window_buffer: &mut [u8] = &mut [0; WINDOW_SIZE];

    loop {
        let input_count = io::stdin().read(input_buffer).unwrap();
        // leave the loop when we reach end of file
        if input_count == 0 { break; }

        // add elements to the back of the ring
        ring.push_many_back(&input_buffer[..input_count]);
        // read as long as we can read windows of length `WINDOW_SIZE`
        while WINDOW_SIZE <= ring.len() {
            ring.read_many_front(window_buffer);
            io::stdout().write(window_buffer).unwrap();
            // do a step by dropping the first `STEP_SIZE` elements in ring
            ring.drop_many_front(STEP_SIZE);
        }
    }
}
```

### performance

the trait [strider::SliceRing](trait.SliceRing.html) is implemented for
[std::collections::VecDeque]
(https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html)
and [strider::SliceRingImpl](struct.SliceRingImpl.html).

[strider::SliceRingImpl](struct.SliceRingImpl.html) is
limited to the functionality in
[strider::SliceRing](trait.SliceRing.html)
but optimized for it and [**2 to 6 times faster**](https://travis-ci.org/snd/strider/jobs/99508425#L192)
than the implementation
for [std::collections::VecDeque](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html).

see the benchmark [results](https://travis-ci.org/snd/strider/jobs/99508425#L192)
and [implementation](https://github.com/snd/strider/blob/master/benches/lib.rs).

windowing the equivalent of 1 minute of 44100 hz audio samples
with a window_size of 1024 and step_size of 512
[only takes around 5 milliseconds](https://travis-ci.org/snd/strider/jobs/99508425#L207).

### memory

the following holds true for both implementations:

[strider::SliceRing::read_many_front](trait.SliceRing.html#tymethod.read_many_front)
never does allocate memory. it reads into a buffer that you
allocate and control.

[strider::SliceRing::drop_many_front](trait.SliceRing.html#tymethod.read_many_drop)
never does allocate memory.

[strider::SliceRing::push_many_back](trait.SliceRing.html#tymethod.push_many_back)
reads from a buffer that you
allocate and control.
it might allocate more capacity
when you push values and as a result there are more values in the ring than ever before.
if you repeatedly read and drop after each push, as in the example above,
the number of values will stay below a certain value and it will never
allocate memory after an initial 1 or 2 allocations.
*/

use std::collections::VecDeque;
use std::usize;
use std::mem;
use std::ptr;
use std::cmp;

/// ringbuffer operations on slices
pub trait SliceRing<T> {
    /// appends `values` to the back of this ring.
    fn push_many_back(&mut self, values: &[T]);
    /// removes `count` elements from the front of this ring.
    /// returns how many elements were removed.
    /// returns less than `count` if less elements are present
    /// in this ring.
    fn drop_many_front(&mut self, count: usize) -> usize;
    /// copies the first `output.len()` elements present in this ring
    /// into `output`.
    /// returns how many elements were copied.
    /// returns less than `output.len()` if there are less elements present
    /// in this ring.
    fn read_many_front(&self, output: &mut [T]) -> usize;
}

impl<T: Clone> SliceRing<T> for VecDeque<T> {
    // `O(input.len())`
    fn push_many_back(&mut self, input: &[T]) {
        for value in input {
            // in most situations this should just be a pointer
            // copy and value copy without any reallocations
            self.push_back(value.clone());
        }
    }
    // `O(count)`
    fn drop_many_front(&mut self, count: usize) -> usize {
        let real_count = std::cmp::min(self.len(), count);
        for _ in 0..real_count {
            self.pop_front();
        }
        real_count
    }
    // `O(min(self.len(), output.len()))`
    fn read_many_front(&self, output: &mut [T]) -> usize {
        let count = std::cmp::min(self.len(), output.len());
        for i in 0..count {
            output[i] = self[i].clone();
        }
        count
    }
}

const INITIAL_CAPACITY: usize = 7; // 2^3 - 1
const MINIMUM_CAPACITY: usize = 1; // 2 - 1
// TODO don't know if this is correct
const MAXIMUM_ZST_CAPACITY: usize = usize::MAX;

/// readable area starts at `first_readable` and goes until
/// `next_writable`.
/// `next_writable` is one after the last readable and not readable.
///
/// ```ignore
/// R = first_readable
/// W = next_writable
/// o = occupied (len)
/// . = free
///
///  R             W
/// [o o o o o o o . . . .]
/// ```
pub struct SliceRingImpl<T> {
    /// index into `buf` of the first element that could be read.
    /// only gets incremented, never decremented.
    /// wraps around.
    pub first_readable: usize,
    /// index into `buf` where the next element could we written.
    /// only gets incremented, never decremented.
    /// wraps around at `buf.cap()`.
    pub next_writable: usize,
    pub buf: Vec<T>,
}

/// Calculate the number of elements left to be read in the buffer
#[inline]
fn count(tail: usize, head: usize, size: usize) -> usize {
    // size is always a power of 2
    (head.wrapping_sub(tail)) & (size - 1)
}

#[inline]
fn wrap_index(index: usize, size: usize) -> usize {
    // size is always a power of 2
    // TODO ?
    debug_assert!(size.is_power_of_two());
    // TODO or is this because the capacity preserves 1 always ?
    let max_index = size - 1;
    index & max_index
}

#[inline]
fn next_power_of_two(x: usize) -> usize {
    (if x.is_power_of_two() { x + 1 } else { x }).next_power_of_two()
}

/// ringbuffer focused on and optimized for operating on slices of values:
/// appending to the back, reading from the front
/// and dropping from the front.
/// which is much faster.
/// TODO call SliceRingImplImpl
impl<T> SliceRingImpl<T> {
    /// creates an empty `SliceRingImpl`.
    pub fn new() -> SliceRingImpl<T> {
        SliceRingImpl::with_capacity(INITIAL_CAPACITY)
    }

    /// creates an empty `SliceRingImpl` with space for at least `n` elements.
    pub fn with_capacity(n: usize) -> SliceRingImpl<T> {
        // +1 since the ringbuffer always leaves one space empty
        let cap = cmp::max(n + 1, MINIMUM_CAPACITY + 1).next_power_of_two();
        assert!(cap > n, "capacity overflow");

        SliceRingImpl {
            first_readable: 0,
            next_writable: 0,
            buf: Vec::with_capacity(cap),
        }
    }

    #[inline]
    pub fn cap(&self) -> usize {
        if mem::size_of::<T>() == 0 {
            // For zero sized types, we are always at maximum capacity
            MAXIMUM_ZST_CAPACITY
        } else {
            self.buf.capacity()
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap() - 1
    }

    #[inline]
    pub fn is_continuous(&self) -> bool {
        self.first_readable <= self.next_writable
    }

    /// returns the number of elements in the `SliceRingImpl`
    #[inline]
    pub fn len(&self) -> usize {
        count(self.first_readable, self.next_writable, self.cap())
    }

    /// returns the index into the underlying buffer
    /// for a given logical element
    /// index + addend
    #[inline]
    pub fn wrap_add(&self, index: usize, addend: usize) -> usize {
        // wrapping_add is a method of std::usize
        wrap_index(index.wrapping_add(addend), self.cap())
    }

    /// Copies a contiguous block of memory len long from src to dst
    /// we can use this if we own the data and move it around
    /// instead of copying it.
    /// the data still exists in only in one place.
    /// it is just moved to another place.
    #[inline]
    unsafe fn copy_nonoverlapping(&mut self, src: usize, dst: usize, len: usize) {
        debug_assert!(dst + len <= self.cap(), "dst={} src={} len={} cap={}", dst, src, len,
                      self.cap());
        debug_assert!(src + len <= self.cap(), "dst={} src={} len={} cap={}", dst, src, len,
                      self.cap());
    }

    /// this is the most complex part
    /// Frobs the head and tail sections around to handle the fact that we
    /// just reallocated. Unsafe because it trusts old_cap.
    #[inline]
    pub unsafe fn handle_cap_increase(&mut self, old_cap: usize) {
        // move the shortest contiguous section of the ring buffer
        // R = first_readable
        // W = next_writable
        // o = occupied
        // . = free
        // c = copied by `handle_cap_increase`

        // continuous !
        // before cap increase:
        //  R             W
        // [o o o o o o o . ]
        // after cap increase:
        //  R             W
        // [o o o o o o o . . . . . . . . . ]
        // after handle_cap_increase:
        //  R             W
        // [o o o o o o o . . . . . . . . . ]
        if self.is_continuous() {
            return
        }

        // shortest section at front:
        // before cap increase:
        //      W R
        // [o o . o o o o o ]
        // after cap increase:
        //      W R
        // [c c . o o o o o . . . . . . . . ]
        // after handle_cap_increase:
        //        R             W
        // [. . . o o o o o c c . . . . . . ]
        // TODO test this
        if self.next_writable < old_cap - self.first_readable {
            let next_writable = self.next_writable;
            let copy_src = 0;
            // after the previous
            let copy_dst = old_cap;
            // everything before next_writable
            let copy_len = next_writable;
            self.copy_nonoverlapping(copy_src, copy_dst, copy_len);
            self.next_writable += old_cap;
            debug_assert!(self.next_writable > self.first_readable);
            debug_assert!(self.next_writable < self.cap());
            debug_assert!(self.first_readable < self.cap());
            debug_assert!(self.cap().is_power_of_two());
            return
        }

        // TODO test this
        // shortest section at tail:
        // before cap increase:
        //            W R
        // [o o o o o . o o ]
        // after cap increase:
        //            W R
        // [o o o o o . c c . . . . . . . . ]
        // after handle_cap_increase:
        //            W                 R
        // [o o o o o . . . . . . . . . c c ]
        let new_cap = self.cap();
        let new_first_readable =
            new_cap - (old_cap - self.first_readable);
        let copy_src = self.first_readable;
        let copy_dst = new_first_readable;
        let copy_len = old_cap - self.first_readable;
        self.copy_nonoverlapping(copy_src, copy_dst, copy_len);
        self.first_readable = new_first_readable;
        debug_assert!(self.next_writable < self.first_readable);
        debug_assert!(self.next_writable < self.cap());
        debug_assert!(self.first_readable < self.cap());
        debug_assert!(self.cap().is_power_of_two());
    }
}

// TODO test with zero sized types and max length

impl<T: Clone> SliceRing<T> for SliceRingImpl<T> {
    // `O(input.len())`
    fn push_many_back(&mut self, input: &[T]) {
        // make enough space
        let additional = input.len();
        let required = self.len() + additional;
        if self.capacity() < required {
            let old_cap = self.cap();
            self.buf.reserve(next_power_of_two(required));
            unsafe {
                self.handle_cap_increase(old_cap);
            }
        }

        for i in 0..additional {
            // Unsafe code so this can be optimised to a memcpy (or something
            // similarly fast) when T is Copy. LLVM is easily confused, so any
            // extra operations during the loop can prevent this optimisation.
            // TODO benchmark a T (struct) that is Copy
            // vs a T (struct) that is Clone
            // TODO maybe replace by two loops that
            // each copy consecutive elements
            unsafe {
                let dst_index = self.wrap_add(self.next_writable, i);
                let dst = self.buf.get_unchecked_mut(dst_index);
                let src = input.get_unchecked(i).clone();
                ptr::write(dst, src);
            }
        }
        self.next_writable = self.wrap_add(self.next_writable, additional);
    }

    // `O(1)`
    fn drop_many_front(&mut self, count: usize) -> usize {
        // TODO improve name of real_count
        let real_count = std::cmp::min(self.len(), count);
        self.first_readable = self.wrap_add(
            self.first_readable, real_count);
        real_count
    }

    // `O(min(self.len(), output.len()))`
    fn read_many_front(&self, output: &mut [T]) -> usize {
        let real_count = std::cmp::min(self.len(), output.len());
        for i in 0..real_count {
            // Unsafe code so this can be optimised to a memcpy (or something
            // similarly fast) when T is Copy. LLVM is easily confused, so any
            // extra operations during the loop can prevent this optimisation.
            unsafe {
                let dst = output.get_unchecked_mut(i);
                let src_index = self.wrap_add(self.first_readable, i);
                let src = self.buf.get_unchecked(src_index).clone();
                ptr::write(dst, src);
            }
        }
        real_count
    }
}

/// macro containing a test run that is used to test and benchmark
/// different implementations of the `SliceRing` trait
#[macro_export]
macro_rules! test_slice_ring {
    ($new:expr) => {{
        let mut testable = $new;
        // we use debug_assert_eq! here because it is omitted in
        // release mode which we want when benchmarking
        debug_assert_eq!(testable.len(), 0);

        // read nothing
        let mut output: Vec<i32> = std::iter::repeat(0).take(1000).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 0);
        debug_assert_eq!(output, std::iter::repeat(0).take(1000).collect::<Vec<i32>>());

        // drop nothing
        debug_assert_eq!(testable.drop_many_front(505), 0);
        debug_assert_eq!(testable.len(), 0);

        // first push. forces cap increase
        // let capacity = testable.capacity();
        let input = (0..3000).collect::<Vec<i32>>();
        testable.push_many_back(&input[..]);
        debug_assert_eq!(testable.len(), 3000);
        // debug_assert!(capacity < testable.capacity());

        let mut output: Vec<i32> = std::iter::repeat(0).take(1000).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 1000);
        debug_assert_eq!(output, (0..1000).collect::<Vec<i32>>());

        // repeated read
        let mut output: Vec<i32> = std::iter::repeat(0).take(200).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 200);
        debug_assert_eq!(output, (0..200).collect::<Vec<i32>>());

        debug_assert_eq!(testable.drop_many_front(100), 100);
        debug_assert_eq!(testable.len(), 2900);

        // test effect of drop
        let mut output: Vec<i32> = std::iter::repeat(0).take(1000).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 1000);
        debug_assert_eq!(output, (100..1100).collect::<Vec<i32>>());

        debug_assert_eq!(testable.drop_many_front(505), 505);
        debug_assert_eq!(testable.len(), 2395);

        // test effect of drop. overread
        let mut output: Vec<i32> = std::iter::repeat(0).take(4000).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 2395);
        debug_assert_eq!(
            output, (605..3000).chain(std::iter::repeat(0).take(1605)).collect::<Vec<i32>>());

        // push without cap increase
        let input = (3000..4000).collect::<Vec<i32>>();
        testable.push_many_back(&input[..]);
        debug_assert_eq!(testable.len(), 3395);
        // debug_assert_eq!(testable.capacity(), 4095);

        // push that forces cap increase
        // let capacity = testable.capacity();
        let input = (4000..6000).collect::<Vec<i32>>();
        testable.push_many_back(&input[..]);
        debug_assert_eq!(testable.len(), 5395);
        // debug_assert!(capacity < testable.capacity());

        // overread
        let mut output: Vec<i32> = std::iter::repeat(0).take(6000).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 5395);
        debug_assert_eq!(
            output, (605..6000).chain(std::iter::repeat(0).take(605)).collect::<Vec<i32>>());

        debug_assert_eq!(testable.drop_many_front(395), 395);
        debug_assert_eq!(testable.len(), 5000);

        // test effect of drop
        let mut output: Vec<i32> = std::iter::repeat(0).take(5000).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 5000);
        debug_assert_eq!(
            output, (1000..6000).collect::<Vec<i32>>());

        debug_assert_eq!(testable.drop_many_front(4000), 4000);
        debug_assert_eq!(testable.len(), 1000);

        // test effect of drop. overread
        let mut output: Vec<i32> = std::iter::repeat(0).take(2000).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 1000);
        debug_assert_eq!(
            output, (5000..6000).chain(std::iter::repeat(0).take(1000)).collect::<Vec<i32>>());

        // drop more than contained
        debug_assert_eq!(testable.drop_many_front(1500), 1000);
        debug_assert_eq!(testable.len(), 0);
        // debug_assert_eq!(testable.capacity(), capacity);

        // read nothing
        let mut output: Vec<i32> = std::iter::repeat(0).take(2000).collect();
        debug_assert_eq!(testable.read_many_front(&mut output[..]), 0);
        debug_assert_eq!(
            output, std::iter::repeat(0).take(2000).collect::<Vec<i32>>());
    }};
}
