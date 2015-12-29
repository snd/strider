use std::collections::VecDeque;

#[macro_use]
extern crate strider;
use strider::SliceRing;
use strider::SliceRingImpl;

#[test]
fn test_slice_ring_unoptimized() {
    test_slice_ring!(VecDeque::<i32>::new());
}
#[test]
fn test_slice_ring_optimized() {
    test_slice_ring!(SliceRingImpl::<i32>::new());
}
