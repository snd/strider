use std::collections::VecDeque;

#[macro_use]
extern crate sliding_window;
use sliding_window::SliceRing;
use sliding_window::SliceRingImpl;

#[test]
fn test_slice_ring_unoptimized() {
    test_slice_ring!(VecDeque::<i32>::new());
}
#[test]
fn test_slice_ring_optimized() {
    test_slice_ring!(SliceRingImpl::<i32>::new());
}
