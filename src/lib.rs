use std::collections::VecDeque;
use std::usize;
use std::mem;
use std::ptr;
use std::cmp;

/// ringbuffer operations on slices
pub trait SliceRing<T> {
    fn push_many_back(&mut self, values: &[T]);
    fn drop_many_front(&mut self, count: usize) -> usize;
    fn read_many_front(&self, output: &mut [T]) -> usize;
}

impl<T: Copy> SliceRing<T> for VecDeque<T> {
    fn push_many_back(&mut self, values: &[T]) {
        for value in values {
            // in most situations this should just be a pointer
            // copy and value copy without any reallocations
            self.push_back(*value);
        }
    }
    fn drop_many_front(&mut self, count: usize) -> usize {
        let real_count = std::cmp::min(self.len(), count);
        for _ in 0..real_count {
            self.pop_front();
        }
        real_count
    }
    fn read_many_front(&self, output: &mut [T]) -> usize {
        let count = std::cmp::min(self.len(), output.len());
        for i in 0..count {
            output[i] = self[i];
        }
        count
    }
}

const INITIAL_CAPACITY: usize = 7; // 2^3 - 1
const MINIMUM_CAPACITY: usize = 1; // 2 - 1
const MAXIMUM_ZST_CAPACITY: usize = usize::MAX;

// readable area starts at first_readable and goes until (not including)
// next_writable is one after the last readable

// R = first_readable
// W = next_writable
// o = occupied (len)
// . = free
//
//  R             W
// [o o o o o o o . . . .]
pub struct OptimizedSliceRing<T> {
    /// index into `buf` of the first element that could be read.
    /// only to be incremented.
    pub first_readable: usize,
    /// index into `buf` where the next element could we written
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

// TODO implement only what's needed below
/// ringbuffer focused on and optimized for operating on slices of values:
/// appending to the back, reading from the front
/// and dropping from the front.
/// which is much faster.
/// TODO call OptimizedSliceRingImpl
impl<T> OptimizedSliceRing<T> {
    /// Creates an empty `OptimizedSliceRing`.
    pub fn new() -> OptimizedSliceRing<T> {
        OptimizedSliceRing::with_capacity(INITIAL_CAPACITY)
    }

    /// Creates an empty `OptimizedSliceRing` with space for at least `n` elements.
    pub fn with_capacity(n: usize) -> OptimizedSliceRing<T> {
        // +1 since the ringbuffer always leaves one space empty
        let cap = cmp::max(n + 1, MINIMUM_CAPACITY + 1).next_power_of_two();
        assert!(cap > n, "capacity overflow");

        OptimizedSliceRing {
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

    #[inline]
    pub fn len(&self) -> usize {
        count(self.first_readable, self.next_writable, self.cap())
    }

    /// - 1 because ...
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
        if self.is_continuous() { return }

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

impl<T: Copy> SliceRing<T> for OptimizedSliceRing<T> {
    /// increases `self.len()` by `count`.
    fn push_many_back(&mut self, input: &[T]) {
        // make enough space
        let additional = input.len();
        let required = self.buf.len() + additional;
        let cap = self.cap();
        if cap < required {
            self.buf.reserve(required);
            unsafe {
                self.handle_cap_increase(cap);
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

    // DONE !!!
    /// reduces `self.len()` by `count`.
    fn drop_many_front(&mut self, count: usize) -> usize {
        // TODO improve name of real_count
        let real_count = std::cmp::min(self.len(), count);
        self.first_readable = self.wrap_add(
            self.first_readable, real_count);
        real_count
    }

    // DONE !!!
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

// /// for safe and convenient ... fixed window and step size
// /// this is the main thing of this module
// /// two backing buffer types:
// /// one simple for illustration
// /// one optimized for performance
// /// benchmarked against each other
// pub struct SlidingWindow<T, Storage: SliceRing<T>> {
//     pub window_size: usize,
//     pub step_size: usize,
//     pub buf: T
// }
//
// impl<T, Storage: SliceRing<T>> SlidingWindow<Storage> {
//     pub fn from_storage(storage: Storage) {
//
//     }
//     pub fn new_slow(window_size: usize, step_size: usize) {
//         SlidingWindow {
//             window_size: window_size,
//             step_size: step_size,
//             // TODO initialize based on window and step size
//             buf: VecDeque::<T>::new()
//         }
//     }
// }
//
// drop `count` elements
// remove `step_size` values from the front of `ringbuffer`
// O(1) instead of O(n)
//
// impl<T> FixedSliceRing<T> {
//
//     /// returns the number of values appended
//     /// `O(n)` where `n = fill_me.len()`
//     pub fn push(&mut self, &[T]) -> usize {
//
//     }
//
//     /// write into `fill_me` the first `fill_me.len()` values
//     /// present in this ring.
//     /// `O(n)` where `n = fill_me.len()`
//     pub fn peak(&self, fill_me: &[T]) -> usize {
//
//     }
//
//     /// drop (remove) the first `count` values
//     /// present in this ring.
//     /// O(1)
//     pub fn pop(&mut self, count: usize) -> usize
//
//     }
//
//     pub fn len(&self) {
//
//     }
//
//     /// 
//     pub fn space(&self) {
//
//     }
// }
