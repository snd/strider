use std::collections::VecDeque;

#[macro_use]
extern crate sliding_window;
use sliding_window::SliceRing;
use sliding_window::OptimizedSliceRing;
use sliding_window::SlidingWindow;

#[test]
fn test_slice_ring_unoptimized() {
    test_slice_ring!(VecDeque::<i32>::new());
}
#[test]
fn test_slice_ring_optimized() {
    test_slice_ring!(OptimizedSliceRing::<i32>::new());
}

#[test]
fn test_sliding_window_unoptimized() {
    // TODO this type declaration is soo ugly
    test_sliding_window!(SlidingWindow::<i32, VecDeque<i32>>::new_unoptimized(
        4096, 512));
}
#[test]
fn test_sliding_window_optimized() {
    // TODO this type declaration is soo ugly
    test_sliding_window!(SlidingWindow::<i32, OptimizedSliceRing<i32>>::new(
        4096, 512));
}
